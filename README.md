<div align="center"> <img src="gol.webp" alt="screenshot" width="400"/> </div>
<div align="center" style="font-size: 40px;">

# gol

</div>
<br>

Very simple Conway's Game of Life implementation in Rust, using the [nannou framework](https://github.com/nannou-org/nannou) for rendering. This project was created as my first Rust project, to get a feel for the language.

The universe of the game contains $2^{32} \times 2^{32}$ unique cells. It is donut shaped, such that structures - like gliders - emerge from the opposite side of the universe when reaching the end.

The game keeps track of which cells are alive by storing the coordinates of live cells in a hash set. When cells are given life or killed, their coordinates are simply inserted into or removed from the hash set.

When compiling, I recommend that you use `cargo run --release`, as the performance enhancements in release mode makes the program much nicer to use. If you'd like to run a performance benchmark, add the `benchmark` arg to the binary or cargo command.

## Interactions

Here are some ways to interact with the game:

| Input                    | Action                       |
| :----------------------- | :--------------------------- |
| `left-click`             | Move view in mouse direction |
| `+` `-` (or scrollwheel) | Zooms in or out              |
| `tab`                    | Toggles stats                |
| `c`                      | Toggle dark mode             |
| `h`                      | Jump back home, to (0, 0)    |
| `j`                      | Jump to random live cell     |
| `z`                      | Undo last jump               |

## Insights

I tried out several hash set implementations (and a non-hash one), in search of which would be most performant for this project. 

Here we see the average time to complete 1000 updates, on 1000 randomly placed cells, over 10,000 runs:

| Set                 | Average Time     | Compared to `std::HashSet` |
| :------------------ | :--------------- | :------------------------- |
| `std::HashSet`      | 26.1372 ms       | 1                          |
| `std::BTreeSet`     | 19.9894 ms       | 1.3076                     |
| `ahash::AHashSet`   | 4.7835 ms        | 5.4640                     |
| `fxhash::FxHashSet` | 3.4792 ms        | 7.5124                     |

As you can see FxHashSet was 7.5 times faster than the standard HashSet implementation. Discourse online suggests that FxHash implements the fastest algorithm when it comes to small key sizes. This explains the massive performance increase we see here!

## Todo

1. Add UI with options for user to choose on start
2. Add ability to load cell setups from text files
4. Add drag-and-drop functionality such that text files can simply be dropped into the window
5. Add options to control update speed
