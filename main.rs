use nannou::prelude::{ App, Update, Frame, MouseButton::Left, };
use nannou::winit::event::WindowEvent as WinitEvent;
use nannou::winit::event::ElementState::{ Pressed, Released, };
use nannou::color::{ BLACK, WHITE };
use nannou::window;
use nannou::rand::random_range;
// use std::collections::HashSet;
use ahash::AHashSet as HashSet;
use std::time::{ Duration, Instant };

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    cells: HashSet<(i32, i32)>,
    kill_list: Vec<(i32, i32)>,
    check_list: Vec<(i32, i32)>,
    res_list: Vec<(i32, i32)>,
    neighbor_list: [(i32, i32); 8],
    view: (f32, f32),
    clicked: bool,
    last_update: Instant,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let mut cells: HashSet<(i32, i32)> = HashSet::new();
    let kill_list: Vec<(i32, i32)> = Vec::new();
    let check_list: Vec<(i32, i32)> = Vec::new();
    let res_list: Vec<(i32, i32)> = Vec::new();
    let neighbor_list: [(i32, i32); 8] = [(0, 0); 8];

    let view: (f32, f32) = (0.0, 0.0);
    let clicked: bool = false;

    let cell_amount = random_range(2500, 5000);
    for _ in 0..cell_amount {
        let cell = (random_range(-100, 100), random_range(-100, 100));
        cells.insert(cell);
    }

    Model { 
        _window,
        cells, 
        kill_list,
        check_list,
        res_list,
        neighbor_list,
        view,
        clicked,
        last_update: Instant::now() 
    }
}

fn raw_window_event(_app: &App, model: &mut Model, winit_event: &WinitEvent) {
    match winit_event {
        WinitEvent::MouseInput { state: Pressed, button: Left, .. } => model.clicked = true,
        WinitEvent::MouseInput { state: Released, button: Left, .. } => model.clicked = false,
        _ => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let clicked = model.clicked;

    // Move view when clicked
    if clicked { 
        model.view.0 -= app.mouse.x/20.0;
        model.view.1 -= app.mouse.y/20.0;
    }

    // Update cells if enough time has passed
    if model.last_update.elapsed() >= Duration::from_millis(25) {
        update_cells(model);
        model.last_update = Instant::now();
    };
}

fn view (app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    for i in model.cells.iter() {
        draw.rect()
            .w_h(10.0, 10.0)
            .x((i.0 as f32) * 10.0 + model.view.0)
            .y((i.1 as f32) * 10.0 + model.view.1)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

// Dumps the coordinates of the neighbors of the cells coordinates given
fn dump_neighbors(coordinates: &(i32, i32), list: &mut [(i32, i32); 8]) {
    let (x, y) = coordinates.clone();
    let (x_left, x_right) = (x.overflowing_sub(1).0, x.overflowing_add(1).0);
    let (y_up, y_down) = (y.overflowing_sub(1).0, y.overflowing_add(1).0);

    list[0] = (x_left,  y_up  ); 
    list[1] = (x,       y_up  );
    list[2] = (x_right, y_up  );
    list[3] = (x_left,  y     );
    list[4] = (x_right, y     );
    list[5] = (x_left,  y_down);
    list[6] = (x,       y_down);
    list[7] = (x_right, y_down);
}

fn count_living_neighbors(list: &[(i32, i32); 8], cells: &HashSet<(i32, i32)>) -> u8 {
    let mut count: u8 = 0;
    for neighbor in list {
        if cells.contains(&neighbor) { count += 1 };
    }
    count
}

fn update_cells(model: &mut Model) {
    let cells = &mut model.cells;
    
    let kill_list = &mut model.kill_list;
    let check_list = &mut model.check_list;
    let res_list = &mut model.res_list;
    let neighbor_list = &mut model.neighbor_list;

    for cell in cells.iter() {
        // Save list of potential new cells
        dump_neighbors(&cell, neighbor_list);
        for neighbor in neighbor_list.iter() {
            if cells.contains(&neighbor) { continue };
            check_list.push(neighbor.clone());       
        }
    
        // Save list of cells that should be killed
        let neighbor_count: u8 = count_living_neighbors(&neighbor_list, &cells);
        if neighbor_count < 2 || neighbor_count > 3 {
            kill_list.push(cell.clone());
        } 
    }

    // Check if potential cells pass requirements, save list of those that should be given life
    for dead_cell in check_list.drain(0..) {
        dump_neighbors(&dead_cell, neighbor_list);
        let neighbor_count: u8 = count_living_neighbors(&neighbor_list, &cells);
        if neighbor_count == 3 {
            res_list.push(dead_cell);
        }
    }

    for cell in kill_list.drain(0..) { cells.remove(&cell); }  
    for dead_cell in res_list.drain(0..) { cells.insert(dead_cell); } 
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
