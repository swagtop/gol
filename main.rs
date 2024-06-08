use nannou::prelude::{ App, Update, Frame, MouseButton::Left, Vec2 };
use nannou::winit::event::WindowEvent as WinitEvent;
use nannou::winit::event::ElementState::{ Pressed, Released};
use nannou::prelude::MouseScrollDelta;
use nannou::color::{ BLACK, WHITE };
use nannou::event::Key::*;
use nannou::window;
use nannou::prelude::Rect;
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
    view: Vec2,
    scale: f32,
    clicked: bool,
    show_stats: bool,
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

    let view: Vec2 = Vec2::from((0.0, 0.0));
    let scale: f32 = 10.0;
    let clicked: bool = false;
    let show_stats: bool = false;

    // Spawn random amount of cells in random position within range
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
        scale,
        clicked,
        show_stats,
        last_update: Instant::now() 
    }
}

fn raw_window_event(_app: &App, model: &mut Model, winit_event: &WinitEvent) {
    match winit_event {
        WinitEvent::KeyboardInput { input, .. } => if input.state == Pressed { 
            //println!("{:?}", input);
            match input.virtual_keycode {
                Some(Minus) | Some(NumpadAdd) => {
                    let new_scale = model.scale - 2.0;
                    if new_scale > 1.0 && new_scale < 30.0 { model.scale = new_scale }
                },
                Some(Equals) | Some(Plus) | Some(NumpadSubtract) => {
                    let new_scale = model.scale + 2.0;
                    if new_scale > 1.0 && new_scale < 30.0 { model.scale = new_scale }
                }
                Some(H) => model.view = (0.0, 0.0).into(),
                Some(Tab) => model.show_stats = !model.show_stats,
                _ => (),
        }
    }
        WinitEvent::MouseInput { state: Pressed, button: Left, .. } => model.clicked = true,
        WinitEvent::MouseInput { state: Released, button: Left, .. } => model.clicked = false,
        WinitEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(_, y), .. } => {
                let new_scale = model.scale + y;
                if new_scale > 1.0 && new_scale < 30.0 { model.scale = new_scale }
        },
        _ => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let clicked = model.clicked;

    // Move view when clicked
    if clicked { 
        model.view.x -= app.mouse.x/200.0;
        model.view.y -= app.mouse.y/200.0;
    }

    // Update cells if enough time has passed
    if model.last_update.elapsed() >= Duration::from_millis(25) {
        update_cells(model);
        model.last_update = Instant::now();
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw()/*.scale(model.scale)*/;

    draw.background().color(BLACK);
    for cell in model.cells.iter() {
        draw.scale(model.scale).rect()
            .w_h(1.0, 1.0)
            .x((cell.0 as f32) + model.view.x)
            .y((cell.1 as f32) + model.view.y)
            .color(WHITE);
    }

    if model.show_stats {
        let corner = Rect::from_w_h(0.0, 0.0).top_left_of(frame.rect());
        let coordinates = ((-model.view.x) as i32).to_string() + ", " + &((-model.view.y) as i32).to_string();
        //let (width, height) = frame.rect().w_h();
        draw.text("Coordinates:").x(corner.x() + 100.0).y(corner.y() - 2.5).left_justify();
        draw.text(&coordinates).x(corner.x() + 100.0).y(corner.y() - 12.5).left_justify();
        
        draw.text("Live cells:").x(corner.x() + 100.0).y(corner.y() - 22.5).left_justify();
        draw.text(&model.cells.len().to_string()).x(corner.x() + 100.0).y(corner.y() - 32.5).left_justify();
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
