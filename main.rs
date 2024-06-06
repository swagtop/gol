use nannou::prelude::*;
use std::collections::HashSet;
use std::time::{Duration, Instant};

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    _cells: HashSet<(i32, i32)>,
    last_update: Instant,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    app.set_loop_mode(LoopMode::Rate { update_interval: Duration::from_secs(10000)});

    let mut _cells: HashSet<(i32, i32)> = HashSet::new();
    _cells.insert((0, 0));
    _cells.insert((-1, -1));
    _cells.insert((0, -1));
    _cells.insert((1, -1));
    _cells.insert((4, -1));
    _cells.insert((3, -1));
    _cells.insert((0, -2));

    Model { _window, _cells, last_update: Instant::now() }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.last_update.elapsed() < Duration::from_millis(100) { return };
    
    let mut kill_list = Vec::new();
    let mut check_list = Vec::new();
    let mut res_list = Vec::new();

    let mut cells = &mut _model._cells;

    for cell in cells.iter() {
        let neighbor_count: u16 = count_neighbors(&cell, &cells);
        if neighbor_count < 2 || neighbor_count > 3 {
            kill_list.push(cell.clone());
        } 
    }

    for cell in cells.iter() {
        let neighbor_list = get_neighbors(&cell);
        for neighbor in neighbor_list.iter() {
            if cells.contains(neighbor) { continue };
            check_list.push(neighbor.clone());       
        }
    }

    for dead_cell in check_list.iter() {
        let neighbor_count: u16 = count_neighbors(&dead_cell, &cells);
        if neighbor_count == 3 {
            res_list.push(dead_cell.clone());
        }
    }

    for cell in kill_list { cells.remove(&cell); }  
    for dead_cell in res_list { cells.insert(dead_cell.clone()); } 

    _model.last_update = Instant::now();
}

fn view (app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(BLACK);
    for i in _model._cells.iter() {
        draw.rect()
            .w_h(10.0, 10.0)
            .x_y((i.0 as f32) * 10.0, (i.1 as f32) * 10.0)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn get_neighbors(coordinates: &(i32, i32)) -> Vec<(i32, i32)> {
    let neighbor_list = Vec::from([
        (coordinates.0 - 1, coordinates.1 - 1),
        (coordinates.0,     coordinates.1 - 1),
        (coordinates.0 + 1, coordinates.1 - 1),
        (coordinates.0 - 1, coordinates.1    ),
        (coordinates.0 + 1, coordinates.1    ),
        (coordinates.0 - 1, coordinates.1 + 1),
        (coordinates.0,     coordinates.1 + 1),
        (coordinates.0 + 1, coordinates.1 + 1)
    ]);
    neighbor_list
}

fn count_neighbors(coordinates: &(i32, i32), cells: &HashSet<(i32, i32)>) -> u16 {
    let mut count: u16 = 0;
    let neighbor_list = get_neighbors(&coordinates);
    for neighbor in neighbor_list.iter() {
        if cells.contains(&neighbor) { count += 1 };
    }
    count
}
