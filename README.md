# advent-of-code2021
My solutions for Advent of Code 2021, in Rust
Comment out the line to run part 1 or 2 of each day depending on which you want to solve.
Most of this is just me trying to learn rust and understand how different design decisions affect performance.  My goal was to minimize execution time and learn about functional programming in Rust.  That means that a lot of the solutions are not that readable, but are built like "one-liners" and are very efficient.

# Running
To run, use cargo and select the binary for the day you want to run.  Some may require the nightly build.  The release build will run significantly faster than debug builds.
```
cargo +nightly run --bin day19 --release
```
