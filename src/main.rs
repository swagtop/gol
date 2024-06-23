use std::env;
use std::thread;
use std::time::Instant;
use nannou::rand::random_range;

mod parallel;
mod single;
mod state;
mod gui;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Run benchmark if arg is given.
    if args.len() != 1 {
        match args[1].as_str() {
            "benchmark" | "--benchmark" | "-b" => { run_benchmark(); }
            "help" | "--help" | "-h" => { print_help(); }
            "version" | "--version" | "-v" => { println!("gol version 1.0.0"); }
            "file" => { gui::run_gui(); },
            _ => ()
        }
        return;
    }

    gui::run_gui();
}

fn run_benchmark() {
    let start_bench_time = Instant::now();

    let mut time_vec = Vec::new();

    let updates_per_run = 500;
    let cell_amount = 3750;
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
            let cell = (random_range(-100, 100), random_range(-100, 100));
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
