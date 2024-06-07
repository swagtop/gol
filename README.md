# gol

Very simple and inefficient conway's game of life implementation in Rust, using the [nannou framework](https://github.com/nannou-org/nannou) for rendering.

This project was created as my first Rust project, to get a feel for the language. The universe contains $(2^{32})^2$ unique cells, and is donut shaped, such that structures - like gliders - emerge from the opposite side of the universe when reaching the end. 

The game keeps track of which cells are alive by storing the coordinates of live cells in a hash set. When cells are given life or killed, their coordinates are simply inserted into or removed from the hash set.

When compiling, I recommend that you use `cargo run --release`, as the performance enhancements in release mode makes the program much nicer to use. You can move the view by clicking in the direction you want to travel, and zoom in and out by scrolling, or pressing `+` and `-` on the keyboard. If you get lost in the seemingly endless darkness, press  the `h` key, and you will safely be taken home to (0.0, 0.0).
