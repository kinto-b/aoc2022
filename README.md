# Advent of Code 2022

Doing AoC2022 in Rust, one puzzle per week.

To run, simply pass the day and (optionally) part to `cargo run`. For example, to run day 2 part 1, at the terminal type


```bash
$ cargo run -- 2 1 
```

Note that the time elapsed which gets printed is quite coarse estimate of the actual time the solution takes to run, since it includes the time taken to dispatch to the appropriate function (which itself takes 500-1000µs).
