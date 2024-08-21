use fxhash::FxHashSet as HashSet;
use std::thread;
use crate::parallel;
use crate::single;
use nannou::prelude::geom::Tri;
use std::collections::LinkedList;

pub type Cell = (i32, i32);

pub trait State {
    fn tick(&mut self);
    fn insert_cells(&mut self, cells: Vec<Cell>);
    fn insert_cells_rel(&mut self, cells: Vec<Cell>, view: (f64, f64));
    fn insert_cell(&mut self, cell: Cell);
    fn collect_cells(&self) -> Vec<Cell>;
    fn count_cells(&self) -> usize;
    fn random_cell(&self) -> Cell;
    fn generation(&self) -> usize;
    fn get_tris(
        &self, 
        view: (f64, f64), 
        cell_color: nannou::prelude::rgb::Rgb,
        screen_left: i32,
        screen_right: i32,
        screen_top: i32,
        screen_bottom: i32
    ) -> LinkedList<Tri<([f32; 3], nannou::prelude::rgb::Rgb)>>;
}

pub fn state() -> Box<dyn State> {
    match thread::available_parallelism() {
        Ok(_) => Box::new(parallel::parallel_state()),
        Err(_) => Box::new(single::single_state()),
    }
}

// Returns arrays of the coordinates of the neighbors of the cells coordinates given.
pub fn get_neighbors(coordinates: &Cell) -> [Cell; 8] {
    let (x, y) = *coordinates;
    let (x_left, x_right) = (x - 1, x + 1);
    let (y_up, y_down) = (y - 1, y + 1);

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

pub fn count_living_neighbors(neighbors: &[Cell; 8], cells: &HashSet<Cell>) -> u8 {
    cells.contains(&neighbors[0]) as u8
        + cells.contains(&neighbors[1]) as u8
        + cells.contains(&neighbors[2]) as u8
        + cells.contains(&neighbors[3]) as u8
        + cells.contains(&neighbors[4]) as u8
        + cells.contains(&neighbors[5]) as u8
        + cells.contains(&neighbors[6]) as u8
        + cells.contains(&neighbors[7]) as u8
}
