<div align="center"> <img src="gol.webp" alt="screenshot" width="400"/> </div>
<div align="center" style="font-size: 40px;">

# gol

</div>

Very simple multithreaded Conway's Game of Life implementation in Rust, using the [nannou framework](https://github.com/nannou-org/nannou) for rendering. This project was created as my first Rust project, for messing around in Rust and getting a feel for the language.

The universe of the game contains $2^{32} \times 2^{32}$ unique cells. It is donut shaped, such that structures - like gliders - emerge from the opposite side of the universe when reaching the end. The game keeps track of which cells are alive by storing the coordinates of live cells in a hash set. When cells are given life or killed, their coordinates are simply inserted into or removed from the hash set.

Remember to compile in release mode: `cargo run --release`. If you'd like to run a performance benchmark, add the `benchmark` to the binary or cargo command.

## Interactions

Here are some ways to interact with gol:

| Input                    | Action                                |
| :----------------------- | :------------------------------------ |
| `+` `-` (or scrollwheel) | Zoom in or out                        |
| `left-click`             | Move view, or draw if in drawing mode |
| `right-click`            | Toggle drawing mode                   |
| `tab`                    | Toggle stats                          |
| `c`                      | Toggle dark mode                      |
| `space`                  | Toggle pause                          |
| `t`                      | Advance cells by one tick             |
| `h`                      | Jump back home, to (0, 0)             |
| `j`                      | Jump to random live cell              |
| `z`                      | Undo last jump                        |

You can also drag and drop files into the game window ([unless you are on Wayland](https://github.com/rust-windowing/winit/issues/720)), and it will load a cell configuration into the universe, based on the characters in the file. So long as you only use ASCII characters, the program should be able to work out which characters represent cells, and which represent empty space.

## Piping

You can pipe cell configurations in and out of gol by using the `-fb` (from bytes), `-tb` (to bytes), or both `-fbtb`. Each cell is encoded as 8 bytes in little endian.

To pipe out whatever you've drawn in GUI mode into a file, start gol like so: 
```bash
gol -tb > cells_in_file
``` 
You can then load the file into gol to get the same configuration back: 
```bash
gol -fb < cells_in_file
```
&nbsp;

Multiple instances of gol could also be chained together like this: 
```bash
gol -tb | gol -fbtb | gol -fb
```
There is no real use for doing this, I just thought it was cool.
&nbsp;

With piping it is possible to load any file as a cell configuration, including gol itself.

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
2. Add options to control update speed
3. Add more piping options

