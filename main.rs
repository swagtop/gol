use nannou::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut cells: HashSet<(i32, i32)> = HashSet::new();
    cells.insert((0, 0));
    cells.insert((-1, -1));
    cells.insert((0, -1));
    cells.insert((1, -1));
    for i in cells.iter() {
        println!("Cell {:?} has {} neighbors!", &i, count_neighbors(&i, &cells));
    }
    println!("HELLO WORLD I AM AN RUST");
    nannou::sketch(view).run()
}

fn count_neighbors(coordinates: &(i32, i32), cells: &HashSet<(i32, i32)>) -> u16 {
    let mut count: u16 = 0;
    if cells.contains(&((coordinates.0 - 1), coordinates.1 - 1)) { count += 1 };
    if cells.contains(&((coordinates.0), coordinates.1 - 1)) { count += 1};
    if cells.contains(&((coordinates.0 + 1), coordinates.1 - 1)) { count += 1 };
    if cells.contains(&((coordinates.0 - 1), coordinates.1)) { count += 1 };
    if cells.contains(&((coordinates.0 + 1), coordinates.1)) { count += 1 };
    if cells.contains(&((coordinates.0 - 1), coordinates.1 + 1)) { count += 1 };
    if cells.contains(&((coordinates.0), coordinates.1 + 1)) { count += 1 };
    if cells.contains(&((coordinates.0 + 1), coordinates.1 + 1)) { count += 1 };
    count
}

fn draw_cells() {}

fn view (app: &App, frame: Frame) {
    let draw = app.draw();

    // Draw a purple triangle in the top left half of the window.
    let win = app.window_rect();

    draw.background().color(CORNFLOWERBLUE);

    // Draw an ellipse to follow the mouse.
    // let t = app.time;

    // Draw a rect that follows a different inverse of the ellipse.
    draw.rect()
        .x_y(-505.0, 205.0)
        .w_h(10.0, 10.0)
        .color(WHITE);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

