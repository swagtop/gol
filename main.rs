use nannou::prelude::{ App, Update, Frame };
use nannou::color::{ BLACK, WHITE };
use nannou::rand::random_range;
use nannou::window;
// use std::collections::HashSet;
use ahash::AHashSet as HashSet;
use std::time::{ Duration, Instant };

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    _cells: HashSet<(i32, i32)>,
    _kill_list: Vec<(i32, i32)>,
    _check_list: Vec<(i32, i32)>,
    _res_list: Vec<(i32, i32)>,
    last_update: Instant,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let mut _cells: HashSet<(i32, i32)> = HashSet::new();
    let mut _kill_list: Vec<(i32, i32)> = Vec::new();
    let mut _check_list: Vec<(i32, i32)> = Vec::new();
    let mut _res_list: Vec<(i32, i32)> = Vec::new();

    let cell_amount = random_range(100, 2000);
    for _ in 0..cell_amount {
        let cell = (random_range(-31, 31), random_range(-31, 31));
        _cells.insert(cell);
    }

    Model { 
        _window,
        _cells, 
        _kill_list,
        _check_list,
        _res_list,
        last_update: Instant::now() 
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.last_update.elapsed() < Duration::from_millis(25) { return };
    
    let cells = &mut _model._cells;
    
    let kill_list = &mut _model._kill_list;
    let check_list = &mut _model._check_list;
    let res_list = &mut _model._res_list;

    let mut neighbor_list: [(i32, i32); 8] = [(0, 0); 8];

    for cell in cells.iter() {
        // Save list of potential new cells
        dump_neighbors(&cell, &mut neighbor_list);
        for neighbor in neighbor_list {
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
        dump_neighbors(&dead_cell, &mut neighbor_list);
        let neighbor_count: u8 = count_living_neighbors(&neighbor_list, &cells);
        if neighbor_count == 3 {
            res_list.push(dead_cell);
        }
    }

    for cell in kill_list.drain(0..) { cells.remove(&cell); }  
    for dead_cell in res_list.drain(0..) { cells.insert(dead_cell); } 

    _model.last_update = Instant::now();
}

fn view (app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    for i in _model._cells.iter() {
        draw.rect()
            .w_h(10.0, 10.0)
            .x_y((i.0 as f32) * 10.0, (i.1 as f32) * 10.0)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

// Dumps the coordinates of the neighbors of the cells coordinates given
fn dump_neighbors(coordinates: &(i32, i32), list: &mut [(i32, i32); 8]) {
    list[0] = (coordinates.0 - 1, coordinates.1 - 1);
    list[1] = (coordinates.0,     coordinates.1 - 1);
    list[2] = (coordinates.0 + 1, coordinates.1 - 1);
    list[3] = (coordinates.0 - 1, coordinates.1    );
    list[4] = (coordinates.0 + 1, coordinates.1    );
    list[5] = (coordinates.0 - 1, coordinates.1 + 1);
    list[6] = (coordinates.0,     coordinates.1 + 1);
    list[7] = (coordinates.0 + 1, coordinates.1 + 1);
}

fn count_living_neighbors(list: &[(i32, i32); 8], cells: &HashSet<(i32, i32)>) -> u8 {
    let mut count: u8 = 0;
    for neighbor in list {
        if cells.contains(&neighbor) { count += 1 };
    }
    count
}
