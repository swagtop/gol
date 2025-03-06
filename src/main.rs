use std::thread;
use std::io::{self, Read};
use std::time::Instant;
use nannou::rand::rand::prelude::StdRng;
use nannou::rand::{SeedableRng, RngCore};
use crate::state::Cell;

use clap::{Arg, ArgAction, Command};

mod parallel;
mod single;
mod state;
mod gui;
mod file;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut start_cells: Vec<Cell> = Vec::default();

    // let mut r = StdRng::seed_from_u64(0);
    // for _ in 0..1000000 {
    //     let cell = (
    //         r.next_u32() as i32 % 1000,
    //         r.next_u32() as i32 % 1000
    //     );
    //     start_cells.push(cell);
    // }

    let matches = Command::new("gol")
        .version("0.1.1")
        .about("A simple Conway's Game of Life implementation")
        .long_about([
            "Control the GUI with the following inputs:",
            "| Input                    | Action                       |",
            "| :----------------------- | :--------------------------- |",
            "| `left-click`             | Move view in mouse direction |",
            "| `+` `-` (or scrollwheel) | Zoom in or out               |",
            "| `tab`                    | Toggle stats                 |",
            "| `c`                      | Toggle dark mode             |",
            "| `space`                  | Toggle pause                 |",
            "| `t`                      | Advance cells by one tick    |",
            "| `h`                      | Jump back home, to (0, 0)    |",
            "| `j`                      | Jump to random live cell     |",
            "| `z`                      | Undo last jump               |",
        ].join("\n"))
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .help("Perform benchmark")
                .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("input-bytes")
                .short('i')
                .long("input-bytes")
                .help("Build cells from bytes read from stdin")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("output-bytes")
                .short('o')
                .long("output-bytes")
                .help("Output bytes of cells to stdout")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    if let Some(benchmark_passes) = matches.get_one::<u32>("benchmark") {
        run_benchmark(*benchmark_passes);
        return;
    } 
    else if matches.contains_id("benchmark") {
        run_benchmark(1);
        return;
    }

    if matches.get_flag("input-bytes") {
        let mut buffer = Vec::new();
        let _ = io::stdin().lock().read_to_end(&mut buffer);
        start_cells.append(&mut from_bytes_to_cells(buffer));
    }
    let send_cells_to_stdout = matches.get_flag("output-bytes");

    gui::run_gui(start_cells, send_cells_to_stdout);
}

fn run_benchmark(benchmark_passes: u32) {
    let start_bench_time = Instant::now();

    let mut time_vec = Vec::new();

    let updates_per_run = 500;
    let cell_amount = 1000000;
    let runs = benchmark_passes;

    let label_string = format!(
        "   Running {} updates on {} cells, {} time(s) ↴   ",
        updates_per_run, cell_amount, runs
    );
    let line_len = label_string.chars().count();
    println!("{}", "-".repeat(line_len));
    println!("{}", label_string);
    println!("{}", "-".repeat(line_len));
    
    let progress_string = format!("0 out of {}", runs);
    eprint!("{: ^width$}\r", progress_string, width = line_len);
    for i in 0..runs {
        let mut state = state::state();

        let mut collection = Vec::default();
        let mut r = StdRng::seed_from_u64(0);

        for _ in 0..cell_amount {
            let cell = (
                r.next_u32() as i32 % 1000,
                r.next_u32() as i32 % 1000
            );
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

fn from_bytes_to_cells(bytes: Vec<u8>) -> Vec<Cell> {
    let cell_amount = (bytes.len() - (bytes.len() % 8)) / 8;
    let mut collection = Vec::default();

    if cell_amount > 0 {
        for i in 0..cell_amount {
            let i = i * 8;
            let cell_x = i32::from_le_bytes([
                bytes[i], 
                bytes[i+1], 
                bytes[i+2], 
                bytes[i+3],
            ]);

            let cell_y = i32::from_le_bytes([
                bytes[i+4], 
                bytes[i+5], 
                bytes[i+6], 
                bytes[i+7],
            ]);

            collection.push((cell_x, cell_y));
        }
    }

    collection
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
