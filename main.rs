use nannou::color::{BLACK, WHITE};
use nannou::event::Key::*;
use nannou::prelude::MouseScrollDelta;
use nannou::prelude::Rect;
use nannou::prelude::{App, Frame, MouseButton::Left, Update, Vec2};
use nannou::rand::random_range;
use nannou::window;
use nannou::winit::event::ElementState::{Pressed, Released};
use nannou::winit::event::WindowEvent as WinitEvent;
// use std::collections::HashSet;
// use std::collections::BTreeSet as HashSet;
// use ahash::AHashSet as HashSet;
use fxhash::FxHashSet as HashSet;
use std::env;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Run benchmark if arg is given
    if args.len() != 1 {
        if &args[1] == "benchmark" {
            run_benchmark();
            return;
        }
    }

    nannou::app(model).update(update).run();
}

struct State {
    cells: HashSet<(i32, i32)>,
    kill_list: Vec<(i32, i32)>,
    res_list: Vec<(i32, i32)>,
}

fn state() -> State {
    let cells: HashSet<(i32, i32)> = HashSet::default();

    let kill_list: Vec<(i32, i32)> = Vec::new();
    let res_list: Vec<(i32, i32)> = Vec::new();

    State {
        cells,
        kill_list,
        res_list,
    }
}

struct Model {
    _window: window::Id,
    state: State,
    view: Vec2,
    last_view: Vec2,
    scale: f32,
    clicked: bool,
    show_stats: bool,
    generation: usize,
    dark_mode: bool,
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
    let scale: f32 = 10.0;
    let clicked: bool = false;
    let show_stats: bool = false;
    let generation: usize = 0;
    let dark_mode: bool = true;

    let mut state = state();

    // Spawn random amount of cells in random position within range
    let cell_amount = random_range(2500, 5000);
    for _ in 0..cell_amount {
        let cell = (random_range(-100, 100), random_range(-100, 100));
        state.cells.insert(cell);
    }

    Model {
        _window,
        state,
        view,
        last_view,
        scale,
        clicked,
        show_stats,
        generation,
        dark_mode,
        last_update: Instant::now(),
    }
}

fn raw_window_event(_app: &App, model: &mut Model, winit_event: &WinitEvent) {
    match winit_event {
        WinitEvent::KeyboardInput { input, .. } => {
            //println!("{:?}", input);
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
                    Some(J) => {
                        model.last_view = model.view.clone();
                        let cells: Vec<(i32, i32)> =
                            model.state.cells.clone().into_iter().collect();
                        let random_cell = cells[random_range(0, cells.len())];
                        (model.view.x, model.view.y) =
                            (-random_cell.0 as f32, -random_cell.1 as f32);
                    }
                    Some(Z) => {
                        let current_view = model.view.clone();
                        model.view = model.last_view;
                        model.last_view = current_view;
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
        WinitEvent::MouseInput {
            state: Pressed,
            button: Left,
            ..
        } => model.clicked = true,
        WinitEvent::MouseInput {
            state: Released,
            button: Left,
            ..
        } => model.clicked = false,
        WinitEvent::MouseWheel {
            delta: MouseScrollDelta::LineDelta(_, y),
            ..
        } => {
            let new_scale = model.scale + y;
            if new_scale > 1.0 && new_scale < 30.0 {
                model.scale = new_scale
            }
        }
        _ => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let clicked = model.clicked;

    // Move view when clicked
    if clicked {
        model.view.x -= app.mouse.x / 100.0 / model.scale;
        model.view.y -= app.mouse.y / 100.0 / model.scale;
    }

    // Update cells if enough time has passed
    if model.last_update.elapsed() >= Duration::from_millis(25) {
        update_cells(&mut model.state);
        model.generation += 1;
        model.last_update = Instant::now();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let cell_color;
    let background_color;

    if model.dark_mode {
        cell_color = WHITE;
        background_color = BLACK;
    } else {
        cell_color = BLACK;
        background_color = WHITE;
    }

    draw.background().color(background_color);
    for cell in model.state.cells.iter() {
        draw.scale(model.scale)
            .rect()
            .w_h(1.0, 1.0)
            .x((cell.0 as f32) + model.view.x)
            .y((cell.1 as f32) + model.view.y)
            .color(cell_color);
    }

    if model.show_stats {
        let corner = Rect::from_w_h(0.0, 0.0).top_left_of(frame.rect());
        let coordinates = format!("{}, {}", (-model.view.x) as i32, (-model.view.y) as i32);

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

        draw.text("Generation:")
            .x(corner.x() + 100.0)
            .y(corner.y() - 22.5)
            .color(cell_color)
            .left_justify();
        draw.text(&model.generation.to_string())
            .x(corner.x() + 100.0)
            .y(corner.y() - 32.5)
            .color(cell_color)
            .left_justify();

        draw.text("Live cells:")
            .x(corner.x() + 100.0)
            .y(corner.y() - 42.5)
            .color(cell_color)
            .left_justify();
        draw.text(&model.state.cells.len().to_string())
            .x(corner.x() + 100.0)
            .y(corner.y() - 52.5)
            .color(cell_color)
            .left_justify();
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update_cells(state: &mut State) {
    let cells = &mut state.cells;

    let kill_list = &mut state.kill_list;
    let res_list = &mut state.res_list;

    for cell in cells.iter() {
        // Mark cell for death by neighbor amount
        let neighbors: [(i32, i32); 8] = get_neighbors(&cell);
        let neighbor_count: u8 = count_living_neighbors(&neighbors, &cells);
        if neighbor_count < 2 || neighbor_count > 3 {
            kill_list.push(*cell);
        }

        // Iterate through dead neighbors, mark ones deserving for life
        for neighbor in neighbors.iter().filter(|&&neighbor| !cells.contains(&neighbor)) {
            let neighbor_neighbors: [(i32, i32); 8] = get_neighbors(&neighbor);
            let neighbor_count: u8 = count_living_neighbors(&neighbor_neighbors, &cells);
            if neighbor_count == 3 {
                res_list.push(*neighbor);
            }
        }
    }

    for cell in kill_list.drain(0..) {
        cells.remove(&cell);
    }
    for resurrected_cell in res_list.drain(0..) {
        cells.insert(resurrected_cell);
    }
}

// Returns arrays of the coordinates of the neighbors of the cells coordinates given
fn get_neighbors(coordinates: &(i32, i32)) -> [(i32, i32); 8] {
    let (x, y) = *coordinates;
    let (x_left, x_right) = (x.overflowing_sub(1).0, x.overflowing_add(1).0);
    let (y_up, y_down) = (y.overflowing_sub(1).0, y.overflowing_add(1).0);

    [
        (x_left, y_up),
        (x, y_up),
        (x_right, y_up),
        (x_left, y),
        (x_right, y),
        (x_left, y_down),
        (x, y_down),
        (x_right, y_down),
    ]
}

fn count_living_neighbors(list: &[(i32, i32); 8], cells: &HashSet<(i32, i32)>) -> u8 {
    let mut count: u8 = 0;
    for neighbor in list {
        if cells.contains(&neighbor) {
            count += 1
        };
    }
    count
}

fn run_benchmark() {
    let mut time_vec = Vec::new();

    let updates_per_run = 1000;
    let cell_amount = 1000;
    let runs = 10000;
    println!(
        "Running {} updates on {} cells, {} times",
        updates_per_run, cell_amount, runs
    );
    for i in 0..runs {
        let mut state = state();

        let begin_time = Instant::now();

        for _ in 0..cell_amount {
            let cell = (random_range(-100, 100), random_range(-100, 100));
            state.cells.insert(cell);
        }
        for _ in 0..updates_per_run {
            update_cells(&mut state);
        }

        let print_string = format!("{} out of {}", i + 1, runs);
        print!("\r{}", print_string);

        time_vec.push((Instant::now() - begin_time).subsec_millis() as f32);
    }

    println!("");
    println!(
        "Average runtime over {} runs: {} ms",
        time_vec.len(),
        time_vec.iter().sum::<f32>() / time_vec.len() as f32
    );
}

//
//
//                     # #                     # #
//                     # #                     # #
//                     # #                     # #
//                     # #                     # #
//                     # # # #             # # # #
//                     # # # #             # # # #
//
//
//     # # # # # #         # # # #     # # # #         # # # # # #
//     # # # # # #         # # # #     # # # #         # # # # # #
//             # #     # #     # #     # #     # #     # #
//             # #     # #     # #     # #     # #     # #
//                     # # # #             # # # #
//                     # # # #             # # # #
//
//
//
//
//                     # # # #             # # # #
//                     # # # #             # # # #
//             # #     # #     # #     # #     # #     # #
//             # #     # #     # #     # #     # #     # #
//     # # # # # #         # # # #     # # # #         # # # # # #
//     # # # # # #         # # # #     # # # #         # # # # # #
//
//
//                     # # # #             # # # #
//                     # # # #             # # # #
//                     # #                     # #
//                     # #                     # #
//                     # #                     # #
//                     # #                     # #
//
//
