use nannou::color::{BLACK, WHITE};
use nannou::event::Key::*;
use nannou::prelude::MouseScrollDelta;
use nannou::prelude::Rect;
use nannou::prelude::{App, Frame, MouseButton::Left, Update, Vec2};
use nannou::rand::random_range;
use nannou::window;
use nannou::winit::event::ElementState::{Pressed, Released};
use nannou::winit::event::WindowEvent as WinitEvent;
use std::collections::HashSet;
// use std::collections::BTreeSet as HashSet;
// use ahash::AHashSet as HashSet;
// use fxhash::FxHashSet as HashSet;
use std::env;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Run benchmark if arg is given.
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
    workers: ThreadPool,
}

fn state() -> State {
    let cells: HashSet<(i32, i32)> = HashSet::default();

    let kill_list: Vec<(i32, i32)> = Vec::new();
    let res_list: Vec<(i32, i32)> = Vec::new();
    
    let workers = ThreadPool::new(
        thread::available_parallelism().unwrap().get()
    );

    State {
        cells,
        kill_list,
        res_list,
        workers,
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

    // Spawn random amount of cells in random position within range.
    let cell_amount = random_range(2500, 5000);
    //let cell_amount = 5000;
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

    // Move view when clicked.
    if clicked {
        model.view.x -= app.mouse.x / 100.0 / model.scale;
        model.view.y -= app.mouse.y / 100.0 / model.scale;
    }

    // Update cells if enough time has passed.
    if model.last_update.elapsed() >= Duration::from_millis(25) {
        model.state.par_tick();
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

impl State {
    fn tick(&mut self) {
        for cell in self.cells.iter() {
            // Mark cell for death by neighbor amount.
            let neighbors: [(i32, i32); 8] = get_neighbors(&cell);
            let neighbor_count = count_living_neighbors(&neighbors, &self.cells);
            if neighbor_count < 2 || neighbor_count > 3 {
                self.kill_list.push(*cell);
            }

            // Iterate through dead neighbors, mark ones deserving for life.
            for neighbor in neighbors
                .iter()
                .filter(|&&neighbor| !self.cells.contains(&neighbor))
            {
                let neighbor_neighbors: [(i32, i32); 8] = get_neighbors(&neighbor);
                let neighbor_count = count_living_neighbors(&neighbor_neighbors, &self.cells);
                if neighbor_count == 3 {
                    self.res_list.push(*neighbor);
                }
            }
        }

        for cell in self.kill_list.drain(0..) {
            self.cells.remove(&cell);
        }
        for resurrected_cell in self.res_list.drain(0..) {
            self.cells.insert(resurrected_cell);
        }
    }

    fn par_tick(&mut self) {
        let cells_vec: Vec<_> = self.cells.iter().copied().collect();
        let cell_amount = cells_vec.len();
        let thread_amount = thread::available_parallelism().unwrap().get();
        let thread_distribution = Arc::new((cell_amount / thread_amount) as usize);

        let cells = Arc::new(cells_vec);
        let cells_set = Arc::new(self.cells.clone());
        //let shared_kill_list = Arc::new(Mutex::<Vec<Vec<(i32, i32)>>>::new(Vec::default()));
        //let shared_res_list = Arc::new(Mutex::<Vec<Vec<(i32, i32)>>>::new(Vec::default()));

        //let mut shared_kill_list = Arc::new(vec![Vec::default(); thread_amount]);
        //let mut shared_res_list = Arc::new(vec![Vec::default(); thread_amount]);
        //let mut handles = Vec::default();
        //println!("Tick starting, with {} cells", cell_amount);

        let shared_kill_list = Arc::new(
            (0..thread_amount)
                .map(|_| Mutex::new(Vec::new()))
                .collect::<Vec<_>>(),
        );
        let shared_res_list = Arc::new(
            (0..thread_amount)
                .map(|_| Mutex::new(Vec::new()))
                .collect::<Vec<_>>(),
        );

        for i in 0..thread_amount {
            //println!("{}", self.workers.active_count());
            let thread_number = Arc::new(i);
            let thread_cells = Arc::clone(&cells);
            let thread_cells_set = Arc::clone(&cells_set);
            let thread_kill_list_list = Arc::clone(&shared_kill_list);
            let thread_res_list_list = Arc::clone(&shared_res_list);
            let this_thread_distribution = Arc::clone(&thread_distribution);

            self.workers.execute(move || {
                let slice_size = *thread_number * *this_thread_distribution;
                let slice = &thread_cells[slice_size .. (slice_size + *this_thread_distribution)];
                //println!("Thread {} starting, with {} cells", *thread_number, slice.len());
                for cell in slice {
                    let neighbors: [(i32, i32); 8] = get_neighbors(&cell);
                    let neighbor_count = count_living_neighbors(&neighbors, &thread_cells_set);
                    if neighbor_count < 2 || neighbor_count > 3 {
                        //thread_kill_list.push(*cell);
                        thread_kill_list_list[*thread_number].lock().unwrap().push(*cell);
                    }

                    // Iterate through dead neighbors, mark ones deserving for life.
                    for neighbor in neighbors
                        .iter()
                        .filter(|&&neighbor| !thread_cells.contains(&neighbor))
                    {
                        let neighbor_neighbors: [(i32, i32); 8] = get_neighbors(&neighbor);
                        let neighbor_count = count_living_neighbors(&neighbor_neighbors, &thread_cells_set);
                        if neighbor_count == 3 {
                            //thread_res_list.push(*neighbor);
                            thread_res_list_list[*thread_number].lock().unwrap().push(*neighbor);
                        }
                    }
                }
                //thread_kill_list_list[*thread_number].push(thread_kill_list);
                //thread_res_list_list[*thread_number].push(thread_kill_list);
                //thread_kill_list_list.lock().unwrap().push(thread_kill_list);
                //thread_res_list_list.lock().unwrap().push(thread_res_list);
                //println!("Thread {} finished", *thread_number);
            });
            //handles.push(handle);
        }
            
        /*for handle in handles {
            handle.join().unwrap();
        }*/
        //println!("Tick finished");
        self.workers.join();

        for kill_list in shared_kill_list.iter() {
            let kill_list = kill_list.lock().unwrap();
            for cell in kill_list.iter() {
                self.cells.remove(&cell);
            }
        }

        for res_list in shared_res_list.iter() {
            let res_list = res_list.lock().unwrap();
            for resurrected_cell in res_list.iter() {
                self.cells.insert(*resurrected_cell);
            }
        }
    }
}

// Returns arrays of the coordinates of the neighbors of the cells coordinates given.
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

fn count_living_neighbors(neighbors: &[(i32, i32); 8], cells: &HashSet<(i32, i32)>) -> u8 {
    cells.contains(&neighbors[0]) as u8
        + cells.contains(&neighbors[1]) as u8
        + cells.contains(&neighbors[2]) as u8
        + cells.contains(&neighbors[3]) as u8
        + cells.contains(&neighbors[4]) as u8
        + cells.contains(&neighbors[5]) as u8
        + cells.contains(&neighbors[6]) as u8
        + cells.contains(&neighbors[7]) as u8
}

fn run_benchmark() {
    let start_bench_time = Instant::now();

    let mut time_vec = Vec::new();

    let updates_per_run = 500;
    let cell_amount = 3750;
    let runs = 100;
    println!(
        "Running {} updates on {} cells, {} times",
        updates_per_run, cell_amount, runs
    );
    
    eprint!("0 out of {}", runs);
    for i in 1..=runs {
        let mut state = state();

        for _ in 0..cell_amount {
            let cell = (random_range(-100, 100), random_range(-100, 100));
            state.cells.insert(cell);
        }

        let begin_time = Instant::now();

        for _ in 0..updates_per_run {
            state.tick();
        }

        time_vec.push((Instant::now() - begin_time).subsec_millis() as f32);

        eprint!("\r{} out of {}", i, runs);
    }
    
    let runtime = time_vec.iter().sum::<f32>() / time_vec.len() as f32;
    println!(
        "\nTotal runtime: {} s",
        (Instant::now() - start_bench_time).as_secs()
    );
    println!(
        "Average runtime over {} runs: {} ms",
        time_vec.len(),
        runtime
    );
    println!(
        "Average tick runtime: {} ms",
        (runtime / updates_per_run as f32)
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
