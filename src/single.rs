use fxhash::FxHashSet as HashSet;

use crate::state::*;

pub struct SingleState {
    cells: HashSet<(i32, i32)>,
    kill_list: Vec<(i32, i32)>,
    res_list: Vec<(i32, i32)>,
}

pub fn single_state() -> SingleState {
    let cells: HashSet<(i32, i32)> = HashSet::default();

    let kill_list: Vec<(i32, i32)> = Vec::new();
    let res_list: Vec<(i32, i32)> = Vec::new();

    SingleState {
        cells,
        kill_list,
        res_list,
    }
}

impl State for SingleState {
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

    fn insert_cells(&mut self, mut collection: Vec<(i32, i32)>) {
        for cell in collection.drain(0..) {
            self.cells.insert(cell);
        }
    }

    fn collect_cells(&self) -> Vec<(i32, i32)> {
        let mut collection = Vec::default();

        for cell in self.cells.iter() {
            collection.push(cell.clone());
        }

        collection
    }
}