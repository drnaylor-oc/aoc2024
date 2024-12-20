# Advent of Code 2024

Here be some probably quite awful Rust code for (some of) AoC 2024.

This code uses the standard Rust toolchain, as of writing I was using Rust 1.83.0, using the Rust 2021 standard (though I don't use anything that'll break in Rust 2024... I think!).

**Puzzle inputs are not included**.[^1] Create a `data` directory and add your inputs (without trailing newlines because I'm too lazy to remove them in code!) with the filename `day[nn]a.txt`, replacing `[nn]` with the two digit day number (so, the input for day 1 is `day01a.txt`). In the very unlikely event that a different input is needed for the second part of a day, that will be suffixed with `b` rather than `a`.

To run the code in dev mode, run via cargo, including the day numbers you want to run:

```bash
cargo run -- <day numbers>

# for example
cargo run -- 1
cargo run -- 1 2
```

or, for all days

```bash
cargo run -- --all
```


Or, compile the binary using cargo:

```bash
cargo build -r
```

then run the binary (careful with working directories, you'll need `data` to be there!):

```bash
target/debug/aoc2024 <day numbers>
```

Tests are included in the source files, as per Rust convention. Use `cargo test` to run the tests. File IO is not tested, so is safe to run without input files.

[^1]: See https://adventofcode.com/2024/about, specifically "Can I copy/redistribute part of Advent of Code?", which says:
    
    "If you're posting a code repository somewhere, please don't include parts of Advent of Code like the puzzle text or your inputs."