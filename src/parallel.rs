use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use threadpool::ThreadPool;
use fxhash::FxHashSet as HashSet;
use crate::state::*;
use nannou::prelude::geom::Tri;
use nannou::color::Rgb;
use nannou::rand::random_range;
use std::collections::LinkedList;

pub struct ParallelState {
    cells: Arc<RwLock<HashSet<(i32, i32)>>>,
    thread_amount: usize,
    kill_lists: Arc<Vec<Mutex<Vec<(i32, i32)>>>>,
    res_lists: Arc<Vec<Mutex<Vec<(i32, i32)>>>>,
    tri_lists: Arc<Vec<Mutex<LinkedList<Tri<([f32; 3], Rgb)>>>>>,
    cells_vec: Arc<RwLock<Vec<(i32, i32)>>>,
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

    let tri_lists = Arc::new(
        (0..thread_amount)
            .map(|_| Mutex::new(LinkedList::new()))
            .collect::<Vec<_>>(),
    );

    let cells_vec = Arc::new(RwLock::new(Vec::default()));
    
    let workers = ThreadPool::new(
        thread::available_parallelism().unwrap().get() - 1
    );

    let generation: usize = 0;

    ParallelState {
        cells,
        thread_amount,
        kill_lists,
        res_lists,
        tri_lists,
        cells_vec,
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
            let thread_cells = &cells_vec;
            let thread_cells_set = &self.cells;
            let thread_kill_lists = &self.kill_lists;
            let thread_res_lists = &self.res_lists;
            
            let slice_start = (self.thread_amount - 1) * *thread_distribution;
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
            
        self.generation += 1;

        self.workers.join();

        let mut cells = self.cells.write().unwrap();
        for kill_list in self.kill_lists.iter() {
            let mut kill_list = kill_list.lock().unwrap();
            for cell in kill_list.iter() {
                cells.remove(&cell);
            }
            kill_list.clear();
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

    fn insert_cells_rel(&mut self, mut collection: Vec<(i32, i32)>, view: (f64, f64)) {
        let mut cells = self.cells.write().unwrap();
        
        for cell in collection.drain(0..) {
            cells.insert((cell.0 + view.1.floor() as i32, cell.1 - view.0.floor() as i32));
        }
    }
    
    fn insert_cell(&mut self, cell: (i32, i32)) {
        self.cells.write().unwrap().insert(cell);
    }
    
    fn insert_cell_rel(&mut self, cell: (i32, i32), view: (f64, f64)) {
        self.cells.write().unwrap().insert((cell.0 + view.1.floor() as i32, cell.1 - view.0.floor() as i32));
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

    fn random_cell(&self) -> (i32, i32) {
        let cells = self.cells.read().unwrap();
        let random_index = random_range(0, cells.len());
        *(cells.iter().nth(random_index).unwrap())
    }

    fn generation(&self) -> usize {
        self.generation
    }

    fn get_tris(
        &self, 
        view: (f64, f64), 
        cell_color: nannou::prelude::rgb::Rgb,
        //screen_left: i32,
        //screen_right: i32,
        //screen_top: i32,
        //screen_bottom: i32
        screen_left: i32,
        screen_right: i32,
        screen_top: i32,
        screen_bottom: i32

    ) -> LinkedList<Tri<([f32; 3], nannou::prelude::rgb::Rgb)>> {
        //self.cells_vec.write().unwrap().extend(self.cells.read().unwrap().iter().filter(|cell| cell.0 > screen_bottom && cell.0 < screen_top && -cell.1 > screen_left && -cell.1 < screen_right).copied());
        //
        self.cells_vec.write().unwrap().extend(
            self.cells
                .read()
                .unwrap()
                .iter()
                .filter(|cell| 
                    cell.0 > screen_left && 
                    cell.0 < screen_right &&
                    cell.1 > screen_bottom && 
                    cell.1 < screen_top
                )
                .copied()
        );
        let cell_amount = self.cells_vec.read().unwrap().len();
        let thread_distribution = Arc::new((cell_amount / self.thread_amount) as usize);

        // Worker threads
        for thread_number in 0..self.thread_amount - 1 {
            //println!("{}", self.workers.active_count());
            let thread_cells = Arc::clone(&self.cells_vec);
            let thread_tri_lists = Arc::clone(&self.tri_lists);
            let this_thread_distribution = Arc::clone(&thread_distribution);

            self.workers.execute(move || {
                let slice_start = thread_number * *this_thread_distribution;
                let slice = &thread_cells.read().unwrap()[slice_start .. (slice_start + *this_thread_distribution)];
                let mut tri_list = thread_tri_lists[thread_number].lock().unwrap();
                
                for cell in slice {
                    let point = [(cell.0 as f64 + view.0 - 0.5) as f32, (cell.1 as f64 + view.1 - 0.5) as f32];

                    let first_tri = nannou::prelude::geom::Tri([
                        ([point[0], point[1], 0.0], cell_color),
                        ([point[0] + 1.0, point[1], 0.0], cell_color),
                        ([point[0] + 1.0, point[1] + 1.0, 0.0], cell_color)
                    ]);

                    let second_tri = nannou::prelude::geom::Tri([
                        first_tri[0], 
                        ([point[0], point[1] + 1.0, 0.0], cell_color), 
                        first_tri[2]
                    ]);
                    
                    tri_list.push_front(first_tri);
                    tri_list.push_front(second_tri);
                }
            });
        }

        // Main thread
        {
            let thread_cells = &self.cells_vec;
            let thread_tri_lists = &self.tri_lists;
            
            let slice_start = (self.thread_amount - 1) * *thread_distribution;
            let slice = &thread_cells.read().unwrap()[slice_start .. thread_cells.read().unwrap().len()];

            let mut tri_list = thread_tri_lists[self.thread_amount - 1].lock().unwrap();
            
            for cell in slice {
                let point = [(cell.0 as f64 + view.0 - 0.5) as f32, (cell.1 as f64 + view.1 - 0.5) as f32];

                let first_tri = nannou::prelude::geom::Tri([
                    ([point[0], point[1], 0.0], cell_color),
                    ([point[0] + 1.0, point[1], 0.0], cell_color),
                    ([point[0] + 1.0, point[1] + 1.0, 0.0], cell_color)
                ]);

                let second_tri = nannou::prelude::geom::Tri([
                    first_tri[0], 
                    ([point[0], point[1] + 1.0, 0.0], cell_color), 
                    first_tri[2]
                ]);
                
                tri_list.push_front(first_tri);
                tri_list.push_front(second_tri);
            }
        }
            
        self.workers.join();

        let mut tris = LinkedList::default();
        for tri_list in self.tri_lists.iter() {
            let tri_list = &mut tri_list.lock().unwrap();
            tris.append(tri_list);
        }
        
        self.cells_vec.write().expect("").clear();
        
        tris
    }
}
