use nannou::color::{BLACK, WHITE};
use nannou::event::Key::*;
use nannou::prelude::MouseScrollDelta;
use nannou::prelude::Rect;
use nannou::prelude::{App, Frame, MouseButton::Left, MouseButton::Right, Update, Vec2};
use nannou::rand::random_range;
use nannou::window;
use nannou::winit::event::ElementState::{Pressed, Released};
use nannou::winit::event::WindowEvent as WinitEvent;
use std::time::{Duration, Instant};
use crate::file;

pub fn run_gui() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    state: Box<dyn crate::state::State>,
    view: Vec2,
    last_view: Vec2,
    cursor_location: Vec2,
    cursor_cell: (i32, i32),
    scale: f32,
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

    let view: Vec2 = (0.0, 0.0).into();
    let last_view: Vec2 = view.clone();
    let cursor_location: Vec2 = (0.0, 0.0).into();
    let cursor_cell: (i32, i32) = (0, 0);
    let scale: f32 = 10.0;
    let clicked: bool = false;
    let show_stats: bool = false;
    let dark_mode: bool = true;
    let paused: bool = false;
    let drawing: bool = false;
    let hovering_file: bool = false;

    let mut state = crate::state::state();

    // Spawn random amount of cells in random position within range.
    let cell_amount = random_range(250000, 500000);
    let mut collection = Vec::default();
    for _ in 0..cell_amount {
        let cell = (random_range(-1000, 1000), random_range(-1000, 1000));
        collection.push(cell);
    }
    state.insert_cells(collection);

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
                    }
                    Some(Equals) | Some(Plus) | Some(NumpadAdd) => {
                        let new_scale = model.scale + 2.0;
                        if new_scale > 1.0 && new_scale < 30.0 {
                            model.scale = new_scale
                        }
                    }
                    Some(H) => {
                        model.last_view = model.view.clone();
                        model.view = (0.0, 0.0).into();
                    }
                    Some(A) => model.view = (-100000.0, 1000000.0).into(),
                    Some(J) => {
                        model.last_view = model.view.clone();
                        let cells: Vec<(i32, i32)> =
                            model.state.collect_cells();
                        let random_cell = cells[random_range(0, cells.len())];
                        (model.view.x, model.view.y) =
                            (-random_cell.1 as f32, random_cell.0 as f32);
                    }
                    Some(Z) => {
                        let current_view = model.view.clone();
                        model.view = model.last_view;
                        model.last_view = current_view;
                    }
                    Some(Space) => model.paused = !model.paused,
                    Some(T) => {
                        model.state.tick();
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
            //model.cursor_cell = 
            let (frame_x, frame_y) = (app.window_rect().w() / 2.0, app.window_rect().h() / 2.0);
            let (x, y) = (position.x as f32 - frame_x, position.y as f32 - frame_y);
            let (x, y) = (x / model.scale, -y / model.scale);
            let (x, y) = (x - 0.5, y - 0.5);
            //let (x, y) = (x.trunc() + model.view.x.fract() - model.scale * 2.0, y.trunc() + model.view.y.fract() - model.scale * 2.0);
            let (x, y) = (x, y + 1.0);
            let (x, y) = (x - model.view.x, y - model.view.y);
            let (x, y) = {
                (x.floor() - model.view.x.fract(), y.floor() - model.view.y.fract())
            };
            model.cursor_cell = (x.floor() as i32 + 1, y.floor() as i32);

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
            //_app.main_window().set_cursor_visible(!model.drawing);
        }
        WinitEvent::MouseWheel {
            delta: MouseScrollDelta::LineDelta(_, y),
            ..
        } => {
            let new_scale = model.scale + y;
            if new_scale > 1.0 && new_scale < 30.0 {
                model.scale = new_scale
            }
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
        model.view.x -= app.mouse.x / 100.0 / model.scale;
        model.view.y -= app.mouse.y / 100.0 / model.scale;
    }

    // Update cells if enough time has passed.
    if model.last_update.elapsed() >= Duration::from_millis(25) && !model.paused {
        model.state.tick();
        model.last_update = Instant::now();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let (cell_color, background_color) = {
        match model.dark_mode {
            true => (WHITE, BLACK),
            _ => (BLACK, WHITE),
    }};

    let cells = model.state.collect_cells();
    let corner = Rect::from_w_h(0.0, 0.0).top_left_of(frame.rect());
    let (screen_left, screen_right) = (
        ((corner.x()) / model.scale + model.view.x) as i32 - 2, 
        ((corner.x() + frame.rect().w()) / model.scale + (model.view.x)) as i32 + 2
    );
    let (screen_top, screen_bottom) = (
        (((corner.y()) / model.scale + (model.view.y))) as i32 + 2,
        ((corner.y() - frame.rect().h()) / model.scale + (model.view.y)) as i32 - 2
    );

    let mut rendered = 0;
    draw.background().color(background_color);
    for cell in &cells {
        if cell.0 > screen_bottom && cell.0 < screen_top &&
            -cell.1 > screen_left && -cell.1 < screen_right {
            
            let coordinates: Vec2 = (cell.1 as f32 + model.view.x - 0.5, -cell.0 as f32 + model.view.y).into();
            draw.scale(model.scale)
                .line()
                .weight(1.0)
                .start((coordinates.x + 1.0, coordinates.y).into())
                .end(coordinates)
                .color(cell_color);
            
                rendered += 1;
        }
    }

    let coordinates = format!("{}, {}", (-model.view.x) as i32, (-model.view.y) as i32);

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
    /*
    let (x, y) = {
        let x_offset = ((x as i32) % (model.scale as i32)) as f32;
        let y_offset = ((y as i32) % (model.scale as i32)) as f32;
        ((x - x_offset), -(y - y_offset))
    }; 
    */
    
    //let (x, y) = (x - (0.5 * model.scale) * x.signum(), y - (0.5 * model.scale) * y.signum());
    //let cursor_cell = (((x / model.scale) - model.view.x) as i32, ((y / model.scale) - model.view.y) as i32);
    //model.cursor_cell = cursor_cell;
    let (x, y) = model.cursor_cell.into();
    let (cursor_x, cursor_y) = {
        (x as f32 + model.view.x - 0.5, y as f32 + model.view.y - 0.5)
    };
    if model.drawing {
        let points: [((_, _), _); 6] = [
            ((cursor_x, cursor_y), cell_color),
            ((cursor_x, cursor_y + 1.0), cell_color),
            ((cursor_x + 1.0, cursor_y + 1.0), cell_color),
            ((cursor_x + 1.0, cursor_y), cell_color),
            ((cursor_x, cursor_y), cell_color),
            ((cursor_x, cursor_y + 1.0), cell_color),
        ];
        draw.scale(model.scale)
            .polyline()
            .caps_square()
            .weight(0.1 + (app.time * 2.5).sin().abs() / 15.0)
            .points_colored(points);
    }
    /*
    if model.drawing && model.clicked {
    }
    */
    
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
        draw.text(&format!("{}, {}", model.cursor_cell.0, model.cursor_cell.1))
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
        draw.text(&cells.len().to_string())
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
            if model.paused {
                "Paused"
            } else {
                "Running"
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
