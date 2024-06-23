<div align="center"> <img src="gol.webp" alt="screenshot" width="400"/> </div>
<div align="center" style="font-size: 40px;">

# gol

</div>

Very simple multithreaded Conway's Game of Life implementation in Rust, using the [nannou framework](https://github.com/nannou-org/nannou) for rendering. This project was created as my first Rust project, for messing around in Rust and getting a feel for the language.

The universe of the game contains $2^{32} \times 2^{32}$ unique cells. It is donut shaped, such that structures - like gliders - emerge from the opposite side of the universe when reaching the end. The game keeps track of which cells are alive by storing the coordinates of live cells in a hash set. When cells are given life or killed, their coordinates are simply inserted into or removed from the hash set.

When compiling, I recommend that you use `cargo run --release`, as the performance enhancements in release mode makes the program much nicer to use. If you'd like to run a performance benchmark, add the `benchmark` arg to the binary or cargo command.

## Interactions

Here are some ways to interact with the game:

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
| `z`                      | Undo last jump               |

You can also drag and drop files into the game window ([unless you are on Wayland](https://github.com/rust-windowing/winit/issues/720)), and it will load a cell configuration into the universe, based on the characters in the file. So long as you only use ASCII characters, the program should be able to work out which characters represent cells, and which represent empty space.

## Insights

A lot of time spent making this project, was toying around with optimizations. Does the program run faster or slower if I create a new vector here, or re-use the same vector each time? One of the things I tried out, was a couple of different hash set implementations, and a non-hash one.

Here we see the average time to complete 500 updates, on 3750 randomly placed cells, over 100 runs:

| Set                 | Average Time | Compared to `std::HashSet` |
| :------------------ | :----------- | :------------------------- |
| `std::BTreeSet`     | 347.71 ms    | 0.623                      |
| `std::HashSet`      | 216.51 ms    | 1                          |
| `ahash::AHashSet`   | 105.20 ms    | 2.058                      |
| `fxhash::FxHashSet` | 89.040 ms    | 2.431                      |

As you can see FxHashSet was almost 2.5 times faster than the standard HashSet implementation. Many forum threads say that xxHash is even faster. I tried it, but it ran hundreds of times slower than even the standard HashSet, so we will be sticking to FxHash.

## Todo

1. Add UI with options for user to choose on start
2. Add ability to load cell setups from text files
3. Add drag-and-drop functionality such that text files can simply be dropped into the window
4. Add options to control update speed
