use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use threadpool::ThreadPool;
use fxhash::FxHashSet as HashSet;
use nannou::prelude::Vec2;
use crate::state::*;

pub struct ParallelState {
    cells: Arc<RwLock<HashSet<(i32, i32)>>>,
    thread_amount: usize,
    kill_lists: Arc<Vec<Mutex<Vec<(i32, i32)>>>>,
    res_lists: Arc<Vec<Mutex<Vec<(i32, i32)>>>>,
    workers: ThreadPool,
    generation: usize,
}

pub fn parallel_state() -> ParallelState {
    let cells = Arc::new(RwLock::new(HashSet::default()));

    let thread_amount = thread::available_parallelism().unwrap().get();
    
    let kill_lists = Arc::new(
    (0..thread_amount)
        .map(|_| Mutex::new(Vec::new()))
        .collect::<Vec<_>>(),
    );

    let res_lists = Arc::new(
        (0..thread_amount)
            .map(|_| Mutex::new(Vec::new()))
            .collect::<Vec<_>>(),
    );
    
    let workers = ThreadPool::new(
        thread::available_parallelism().unwrap().get() - 1
    );

    let generation: usize = 0;

    ParallelState {
        cells,
        thread_amount,
        kill_lists,
        res_lists,
        workers,
        generation,
    }
}

impl State for ParallelState {
    fn tick(&mut self) {
        let cells_vec = Arc::new(RwLock::new(self.cells.read().unwrap().iter().copied().collect::<Vec<_>>()));
        let cell_amount = cells_vec.read().unwrap().len();
        let thread_distribution = Arc::new((cell_amount / self.thread_amount) as usize);

        // Worker threads
        for thread_number in 0..self.thread_amount - 1 {
            //println!("{}", self.workers.active_count());
            let thread_cells = Arc::clone(&cells_vec);
            let thread_cells_set = Arc::clone(&self.cells);
            let thread_kill_lists = Arc::clone(&self.kill_lists);
            let thread_res_lists = Arc::clone(&self.res_lists);
            let this_thread_distribution = Arc::clone(&thread_distribution);

            self.workers.execute(move || {
                let slice_start = thread_number * *this_thread_distribution;
                let slice = &thread_cells.read().unwrap()[slice_start .. (slice_start + *this_thread_distribution)];
                let cells = thread_cells_set.read().unwrap();
                let mut kill_list = thread_kill_lists[thread_number].lock().unwrap();
                let mut res_list = thread_res_lists[thread_number].lock().unwrap();
                
                for cell in slice {
                    let neighbors: [(i32, i32); 8] = get_neighbors(&cell);
                    let neighbor_count = count_living_neighbors(&neighbors, &cells);
                    if neighbor_count < 2 || neighbor_count > 3 {
                        kill_list.push(*cell);
                    }

                    // Iterate through dead neighbors, mark ones deserving for life.
                    for neighbor in neighbors
                        .iter()
                        .filter(|&&neighbor| !cells.contains(&neighbor))
                    {
                        let neighbor_neighbors: [(i32, i32); 8] = get_neighbors(&neighbor);
                        let neighbor_count = count_living_neighbors(&neighbor_neighbors, &cells);
                        if neighbor_count == 3 {
                            res_list.push(*neighbor);
                        }
                    }
                }
            });
        }

        // Main thread
        {
            let thread_cells = Arc::clone(&cells_vec);
            let thread_cells_set = Arc::clone(&self.cells);
            let thread_kill_lists = Arc::clone(&self.kill_lists);
            let thread_res_lists = Arc::clone(&self.res_lists);
            let this_thread_distribution = Arc::clone(&thread_distribution);
            
            let slice_start = (self.thread_amount - 1) * *this_thread_distribution;
            let slice = &thread_cells.read().unwrap()[slice_start .. thread_cells.read().unwrap().len()];

            let cells = thread_cells_set.read().unwrap();
            let mut kill_list = thread_kill_lists[self.thread_amount - 1].lock().unwrap();
            let mut res_list = thread_res_lists[self.thread_amount - 1].lock().unwrap();
            
            for cell in slice {
                let neighbors: [(i32, i32); 8] = get_neighbors(&cell);
                let neighbor_count = count_living_neighbors(&neighbors, &cells);
                if neighbor_count < 2 || neighbor_count > 3 {
                    kill_list.push(*cell);
                }

                // Iterate through dead neighbors, mark ones deserving for life.
                for neighbor in neighbors
                    .iter()
                    .filter(|&&neighbor| !cells.contains(&neighbor))
                {
                    let neighbor_neighbors: [(i32, i32); 8] = get_neighbors(&neighbor);
                    let neighbor_count = count_living_neighbors(&neighbor_neighbors, &cells);
                    if neighbor_count == 3 {
                        res_list.push(*neighbor);
                    }
                }
            }
        }
            
        self.workers.join();

        let mut cells = self.cells.write().unwrap();
        if !cells.is_empty() {
            self.generation += 1;
        }
        for kill_list in self.kill_lists.iter() {
            let mut kill_list = kill_list.lock().unwrap();
            for cell in kill_list.drain(0..) {
                cells.remove(&cell);
            }
        }
        for res_list in self.res_lists.iter() {
            let mut res_list = res_list.lock().unwrap();
            for resurrected_cell in res_list.drain(0..) {
                cells.insert(resurrected_cell);
            }
        }    
    }

    fn insert_cells(&mut self, mut collection: Vec<(i32, i32)>) {
        let mut cells = self.cells.write().unwrap();

        for cell in collection.drain(0..) {
            cells.insert(cell);
        }
    }

    fn insert_cells_rel(&mut self, mut collection: Vec<(i32, i32)>, view: Vec2) {
        let mut cells = self.cells.write().unwrap();
        
        for cell in collection.drain(0..) {
            cells.insert((cell.1 + (view.y as i32), cell.0 - (view.x as i32)));
        }
    }

    fn collect_cells(&self) -> Vec<(i32, i32)> {
        let cells = self.cells.read().unwrap();
        let mut collection = Vec::default();

        for cell in cells.iter() {
            collection.push(cell.clone());
        }
        
        collection
    }

    fn count_cells(&self) -> usize {
        self.cells.read().unwrap().len()
    }

    fn generation(&self) -> usize {
        self.generation
    }
}
