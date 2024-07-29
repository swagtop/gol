use nannou::event::Key::*;
use nannou::prelude::MouseScrollDelta;
use nannou::prelude::Rect;
use nannou::prelude::{App, Frame, MouseButton::Left, MouseButton::Right, Update, Vec2};
use nannou::window;
use nannou::winit::event::ElementState::{Pressed, Released};
use nannou::winit::event::WindowEvent as WinitEvent;
use std::time::{Duration, Instant};
use nannou::color::Rgb;
use std::sync::Mutex;
use std::io::{self, Write};
use crate::file;

lazy_static! {
    static ref START_CELLS: Mutex<Vec<(i32, i32)>> = Mutex::new(Vec::new());
}

pub fn run_gui(mut start_cells: Vec<(i32, i32)>) {
    START_CELLS.lock().unwrap().append(&mut start_cells);
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    state: Box<dyn crate::state::State>,
    view: (f64, f64),
    last_view: (f64, f64),
    cursor_location: Vec2,
    cursor_cell: (i32, i32),
    scale: f64,
    clicked: bool,
    show_stats: bool,
    dark_mode: bool,
    paused: bool,
    drawing: bool,
    hovering_file: bool,
    last_update: Instant,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    
    app.main_window().set_title("gol");
    app.set_exit_on_escape(false);

    let view: (f64, f64) = (0.0, 0.0).into();
    let last_view: (f64, f64) = view.clone();
    let cursor_location: Vec2 = (0.0, 0.0).into();
    let cursor_cell: (i32, i32) = (0, 0);
    let scale: f64 = 10.0;
    let clicked: bool = false;
    let show_stats: bool = false;
    let dark_mode: bool = true;
    let paused: bool = true;
    let drawing: bool = false;
    let hovering_file: bool = false;

    let mut state = crate::state::state();

    // Spawn random amount of cells in random position within range.
    /*
    let cell_amount = random_range(250000, 500000);
    let mut collection = Vec::default();
    for _ in 0..cell_amount {
        let cell = (random_range(-1000, 1000), random_range(-1000, 1000));
        collection.push(cell);
    }
    state.insert_cells(collection);
    */
    state.insert_cells(START_CELLS.lock().unwrap().to_vec());

    Model {
        _window,
        state,
        view,
        last_view,
        cursor_location,
        cursor_cell,
        scale,
        clicked,
        show_stats,
        dark_mode,
        paused,
        drawing,
        hovering_file,
        last_update: Instant::now(),
    }
}
    
fn update_cursor_cell(model: &mut Model) -> () {
    let (x, y) = (model.cursor_location.x as f64, model.cursor_location.y as f64);
    let (x, y) = (x / model.scale  as f64, -y / model.scale as f64);
    let (x, y) = (x - 0.5, y - 0.5);
    let (x, y) = (x, y + 1.0);
    let (x, y) = (x - model.view.0, y - model.view.1);
    model.cursor_cell = (x.floor() as i32 + 1, y.floor() as i32);
}

// https://docs.rs/winit/0.28.7/winit/event/enum.WindowEvent.html
fn raw_window_event(app: &App, model: &mut Model, winit_event: &WinitEvent) {
    match winit_event {
        WinitEvent::KeyboardInput { input, .. } => {
            if input.state == Pressed {
                match input.virtual_keycode {
                    Some(Minus) | Some(NumpadSubtract) => {
                        let new_scale = model.scale - 2.0;
                        if new_scale > 1.0 && new_scale < 30.0 {
                            model.scale = new_scale
                        }
                        update_cursor_cell(model);
                    }
                    Some(Equals) | Some(Plus) | Some(NumpadAdd) => {
                        let new_scale = model.scale + 2.0;
                        if new_scale > 1.0 && new_scale < 30.0 {
                            model.scale = new_scale
                        }
                        update_cursor_cell(model);
                    }
                    Some(H) => {
                        model.last_view = model.view.clone();
                        model.view = (0.0, 0.0).into();
                        update_cursor_cell(model);
                    }
                    Some(A) => model.view = (0b01111111111111111111111111111110i32 as f64, 0b01111111111111111111111111111110i32 as f64),
                    Some(J) => {
                        if model.state.count_cells() != 0 {
                            model.last_view = model.view.clone();
                            let random_cell = model.state.random_cell();
                            model.view = (-random_cell.1 as f64, random_cell.0 as f64);
                            update_cursor_cell(model);
                        }
                    }
                    Some(Z) => {
                        let current_view = model.view.clone();
                        model.view = model.last_view;
                        model.last_view = current_view;
                        update_cursor_cell(model);
                    }
                    Some(Space) => model.paused = !model.paused,
                    Some(T) => {
                        model.state.tick();
                    }
                    Some(Escape) => {
                        let _ = io::stdout().lock().write_all(&from_cells_to_bytes(model.state.collect_cells()));
                        app.quit();
                    }
                    _ => (),
                }
            } else {
                match input.virtual_keycode {
                    Some(Tab) => model.show_stats = !model.show_stats,
                    Some(C) => model.dark_mode = !model.dark_mode,
                    _ => (),
                }
            }
        }
        WinitEvent::CursorMoved { position, .. } => {
            let (frame_x, frame_y) = (app.window_rect().w() / 2.0, app.window_rect().h() / 2.0);
            let (x, y) = (
                (position.x as f32 / app.main_window().scale_factor()) - frame_x, 
                (position.y as f32 / app.main_window().scale_factor()) - frame_y
            );
            model.cursor_location = (x, y).into();
            update_cursor_cell(model);

            if model.drawing && model.clicked {
                model.state.insert_cell((-model.cursor_cell.1, model.cursor_cell.0));
            }
        }
        WinitEvent::MouseInput {
            state: Pressed,
            button: Left,
            ..
        } => {
            model.clicked = true;
            if model.drawing && model.clicked {
                model.state.insert_cell((-model.cursor_cell.1, model.cursor_cell.0));
            }
        },
        WinitEvent::MouseInput {
            state: Released,
            button: Left,
            ..
        } => model.clicked = false,
        WinitEvent::MouseInput {
            state: Pressed,
            button: Right,
            ..
        } => {
            model.drawing = !model.drawing;
        }
        WinitEvent::MouseWheel {
            delta: MouseScrollDelta::LineDelta(_, y),
            ..
        } => {
            let new_scale = model.scale + *y as f64;
            if new_scale > 1.0 && new_scale < 30.0 {
                model.scale = new_scale
            }
            update_cursor_cell(model);
        }
        WinitEvent::HoveredFile { .. } => model.hovering_file = true,
        WinitEvent::DroppedFile(path) => {
            model.hovering_file = false;
            model.state.insert_cells_rel(file::cells_from_file(path.as_path().to_str().unwrap().to_string()), model.view);
        }
        WinitEvent::HoveredFileCancelled => model.hovering_file = false,
        _ => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Move view when clicked.
    if model.clicked && !model.drawing {
        model.view.0 -= app.mouse.x as f64 / 100.0 / model.scale;
        model.view.1 -= app.mouse.y as f64 / 100.0 / model.scale;

        if model.view.0 < -2147483647.0 || model.view.0 > 2147483647.0 {
            model.view.0 *= -1.0;
        }
        if model.view.1 < -2147483647.0 || model.view.1 > 2147483647.0 {
            model.view.1 *= -1.0;
        }

        update_cursor_cell(model);
    }

    // Update cells if enough time has passed.
    if model.last_update.elapsed() >= Duration::from_millis(25) && !model.paused && model.state.count_cells() != 0 {
        model.state.tick();
        model.last_update = Instant::now();

        if model.drawing && model.clicked {
            model.state.insert_cell((-model.cursor_cell.1, model.cursor_cell.0));
        }
   }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let (cell_color, background_color) = {
        let black = Rgb::from_components((-3.0, -3.0, -3.0)); 
        let white = Rgb::from_components((1.0, 1.0, 1.0)); 
        match model.dark_mode {
            true => (white, black),
            _ => (black, white),
        }
    };

    //let cells = model.state.collect_cells();
    let corner = Rect::from_w_h(0.0, 0.0).top_left_of(frame.rect());
    let (screen_left, screen_right) = (
        ((corner.x() as f64) / model.scale + model.view.0) as i32 - 2, 
        ((corner.x() + frame.rect().w()) as f64 / model.scale + (model.view.0)) as i32 + 2
    );
    let (screen_top, screen_bottom) = (
        (((corner.y() as f64) / model.scale + (model.view.1))) as i32 + 2,
        ((corner.y() - frame.rect().h()) as f64 / model.scale + (model.view.1)) as i32 - 2
    );

    draw.background().color(background_color);

    let (tris, rendered) = model.state.get_tris(
        model.view,
        cell_color,
        screen_left,
        screen_right,
        screen_top,
        screen_bottom,
    );

    draw.scale(model.scale as f32)
        .mesh()
        .tris_colored(tris);

    if model.hovering_file {
        let points: [((_, _), _); 5] = [
            ((corner.x(), corner.y()), cell_color),
            ((corner.x() + frame.rect().w(), corner.y()), cell_color),
            ((corner.x() + frame.rect().w(), corner.y() - frame.rect().h()), cell_color),
            ((corner.x(), corner.y() - frame.rect().h()), cell_color),
            ((corner.x(), corner.y()), cell_color),
        ];
        draw.polyline()
            .weight(4.0 + ((app.time * 2.5).sin().abs() * 4.0))
            .points_colored(points);
    }

    let (x, y) = model.cursor_cell.into();
    let (cursor_x, cursor_y) = (x as f64 + model.view.0 - 0.5, y as f64 + model.view.1 - 0.5);
    let (cursor_x, cursor_y) = (cursor_x as f32, cursor_y as f32);
    if model.drawing {
        let cell_color_points: [((_, _), _); 6] = [
            ((cursor_x, cursor_y), cell_color),
            ((cursor_x, cursor_y + 1.0), cell_color),
            ((cursor_x + 1.0, cursor_y + 1.0), cell_color),
            ((cursor_x + 1.0, cursor_y), cell_color),
            ((cursor_x, cursor_y), cell_color),
            ((cursor_x, cursor_y + 1.0), cell_color),
        ];
        draw.scale(model.scale as f32)
            .polyline()
            .weight(0.1 + (app.time * 2.5).sin().abs() / 15.0)
            .points_colored(cell_color_points);
    }
    
    let coordinates = format!("{}, {}", (-model.view.0) as i32, (-model.view.1) as i32);
    let cursor = format!("{}, {}", model.cursor_cell.0, model.cursor_cell.1);

    if model.show_stats {
        draw.text("Coordinates:")
            .x(corner.x() + 100.0)
            .y(corner.y() - 2.5)
            .color(cell_color)
            .left_justify();
        draw.text(&coordinates)
            .x(corner.x() + 100.0)
            .y(corner.y() - 12.5)
            .color(cell_color)
            .left_justify();

        draw.text("Cursor:")
            .x(corner.x() + 100.0)
            .y(corner.y() - 22.5)
            .color(cell_color)
            .left_justify();
        draw.text(&cursor)
            .x(corner.x() + 100.0)
            .y(corner.y() - 32.5)
            .color(cell_color)
            .left_justify();

        draw.text("Generation:")
            .x(corner.x() + 100.0)
            .y(corner.y() - 42.5)
            .color(cell_color)
            .left_justify();
        draw.text(&model.state.generation().to_string())
            .x(corner.x() + 100.0)
            .y(corner.y() - 52.5)
            .color(cell_color)
            .left_justify();

        draw.text("Live cells:")
            .x(corner.x() + 100.0)
            .y(corner.y() - 62.5)
            .color(cell_color)
            .left_justify();
        draw.text(&model.state.count_cells().to_string())
            .x(corner.x() + 100.0)
            .y(corner.y() - 72.5)
            .color(cell_color)
            .left_justify();

        draw.text("Rendered cells:")
            .x(corner.x() + 100.0)
            .y(corner.y() - 82.5)
            .color(cell_color)
            .left_justify();
        draw.text(&rendered.to_string())
            .x(corner.x() + 100.0)
            .y(corner.y() - 92.5)
            .color(cell_color)
            .left_justify();

        let status = {
            match model.paused {
                true => "Paused",
                _ => "Running"
            }
        };

        draw.text(status)
            .x(corner.x() + 100.0)
            .y(corner.y() - 102.5)
            .color(cell_color)
            .left_justify();
    }

    draw.to_frame(app, &frame).unwrap();
}

fn from_cells_to_bytes(collection: Vec<(i32, i32)>) -> Vec<u8> {
    let mut bytes = Vec::default();

    for cell in collection {
        let x_bytes = cell.0.to_le_bytes();
        bytes.push(x_bytes[0]);
        bytes.push(x_bytes[1]);
        bytes.push(x_bytes[2]);
        bytes.push(x_bytes[3]);
        
        let y_bytes = cell.1.to_le_bytes();
        bytes.push(y_bytes[0]);
        bytes.push(y_bytes[1]);
        bytes.push(y_bytes[2]);
        bytes.push(y_bytes[3]);
    }

    bytes
}
