# advent-of-code-2025

https://adventofcode.com/

Now in rust!

In Rust, `main.rs` is always the entry point, so start there if you want to see how the runner works. It just handles command line input and reading a file and sending it to the day/part specific code. 

Each day and part has its own file where all the real interesting stuff is.

Run with `cargo run -- $day#-part# path/input-file.txt`, e.g. `cargo run -- 1-2 inputs/day1.txt` or `cargo run -- 10-1 inputs/day10-test.txt`.
