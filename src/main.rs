use std::env;
use std::thread;
use std::io::{self, Read, Write};
use std::time::Instant;
use nannou::rand::random_range;

mod parallel;
mod single;
mod state;
mod gui;
mod file;

#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start_cells: Vec<(i32, i32)> = Vec::default();

    eprintln!("{:?}: {:?}", (255, 255), from_cells_to_bytes(vec![(255, 255)]));
    eprintln!("{:?}: {:?}", (256, 256), from_cells_to_bytes(vec![(256, 256)]));
    
    eprintln!("{:?}: {:?}", (0, 0, 0, 255, 0, 0, 0, 255), from_bytes_to_cells(vec![0u8, 0u8, 0u8, 255u8, 0u8, 0u8, 0u8, 255u8]));
    eprintln!("{:?}: {:?}", (0, 0, 1, 0, 0, 0, 1, 0), from_bytes_to_cells(vec![0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8]));

    // Run benchmark if arg is given.
    if args.len() != 1 {
        match args[1].as_str() {
            "benchmark" | "--benchmark" | "-b" => { run_benchmark(); }
            "help" | "--help" | "-h" => { print_help(); }
            "version" | "--version" | "-v" => { println!("gol version 1.0.0"); }
            "file" => { gui::run_gui(start_cells); },
            "-fb" => {
                let mut buffer = Vec::new();
                let _ = io::stdin().read_to_end(&mut buffer);
                start_cells.append(&mut from_bytes_to_cells(buffer));
                gui::run_gui(start_cells);
            }
            "-fbtb" => {
                let mut buffer = Vec::new();
                let _ = io::stdin().read_to_end(&mut buffer);
                start_cells.append(&mut from_bytes_to_cells(buffer));
                let end_cells = gui::run_gui(start_cells);
                io::stdout().lock().write_all(&from_cells_to_bytes(end_cells));
            }
            "-tb" => {
                let end_cells = gui::run_gui(start_cells);
                io::stdout().lock().write_all(&from_cells_to_bytes(end_cells));
            }
            _ => ()
        }
        return;
    }

    gui::run_gui(start_cells);
}

fn run_benchmark() {
    let start_bench_time = Instant::now();

    let mut time_vec = Vec::new();

    let updates_per_run = 500;
    let cell_amount = 375000;
    let runs = 100;

    let label_string = format!(
        "   Running {} updates on {} cells, {} times ↴   ",
        updates_per_run, cell_amount, runs
    );
    let line_len = label_string.chars().count();
    println!("{}", "-".repeat(line_len));
    println!("{}", label_string);
    println!("{}", "-".repeat(line_len));
    
    let progress_string = format!("0 out of {}", runs);
    eprint!("{: ^width$}\r", progress_string, width = line_len);
    for i in 1..=runs {
        let mut state = state::state();

        let mut collection = Vec::default();
        for _ in 0..cell_amount {
            let cell = (random_range(-1000, 1000), random_range(-1000, 1000));
            collection.push(cell);
        }
        state.insert_cells(collection);

        let begin_time = Instant::now();

        for _ in 0..updates_per_run {
            state.tick();
        }

        time_vec.push((Instant::now().duration_since(begin_time)).as_millis() as f32);

        //eprint!("{} out of {}\r", i, runs);
        let progress_string = format!("{} out of {}", i, runs);
        eprint!("{:^width$}\r", progress_string, width = line_len);
    }
    
    let runtime = time_vec.iter().sum::<f32>() / time_vec.len() as f32;
    println!(
        "Total runtime: {:>width$}  s",
        (Instant::now().duration_since(start_bench_time)).as_millis() as f32 / 1000.0,
        width = line_len - 18
    );
    println!(
        "Average run duration: {:>width$} ms",
        runtime,
        width = line_len - 25
    );
    println!(
        "Average tick duration: {:>width$} ms",
        (runtime / updates_per_run as f32),
        width = line_len - 26
    );
    match thread::available_parallelism() {
        Ok(i) => println!("Thread count: {:>width$}   ", i, width = line_len - 17),
        Err(_) => println!("No multithreading"),
    }
}

fn print_help() {
    let help_string = "\
| Input                    | Action                       |
| :----------------------- | :--------------------------- |
| `left-click`             | Move view in mouse direction |
| `+` `-` (or scrollwheel) | Zoom in or out               |
| `tab`                    | Toggle stats                 |
| `c`                      | Toggle dark mode             |
| `space`                  | Toggle pause                 |
| `t`                      | Advance cells by one tick    |
| `h`                      | Jump back home, to (0, 0)    |
| `j`                      | Jump to random live cell     |
| `z`                      | Undo last jump               |\
    ";

    println!("{}", help_string);
}

fn from_bytes_to_cells(bytes: Vec<u8>) -> Vec<(i32, i32)> {
    let cell_amount = (bytes.len() - (bytes.len() % 8)) / 8;
    eprintln!("cell amt: {}", cell_amount);
    eprintln!("bytes len: {}", bytes.len());
    let mut collection = Vec::default();

    if cell_amount > 0 {
        for i in 0..cell_amount {
            let i = i * 8;

            let cell_x = i32::from_le_bytes([
                bytes[i+3],
                bytes[i+2], 
                bytes[i+1], 
                bytes[i], 
            ]);

            let cell_y = i32::from_le_bytes([
                bytes[i+7],
                bytes[i+6], 
                bytes[i+5], 
                bytes[i+4], 
            ]);

            collection.push((cell_x, cell_y));
        }
    }

    collection
}

fn from_cells_to_bytes(collection: Vec<(i32, i32)>) -> Vec<u8> {
    let mut bytes = Vec::default();

    for cell in collection {
        let x_bytes = cell.0.to_le_bytes();
        bytes.push(x_bytes[3]);
        bytes.push(x_bytes[2]);
        bytes.push(x_bytes[1]);
        bytes.push(x_bytes[0]);
        
        let y_bytes = cell.1.to_le_bytes();
        bytes.push(y_bytes[3]);
        bytes.push(y_bytes[2]);
        bytes.push(y_bytes[1]);
        bytes.push(y_bytes[0]);
    }

    bytes
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
