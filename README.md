# gol

Very simple conway's game of life implementation in Rust, using the [nannou framework](https://github.com/nannou-org/nannou) for rendering.

This project was created as my first Rust project, to get a feel for the language. The universe contains $(2^{32})^2$ unique cells, and is donut shaped, such that structures such as gliders emerge from the opposite side of the universe when reaching the end. 

The game keeps track of which cells are alive by storing the coordinates of live cells in a hash set. When cells are given life or killed, their coordinates are simply inserted into or removed from the hash set.

Can be checked out by running `cargo run` in the main directory.
