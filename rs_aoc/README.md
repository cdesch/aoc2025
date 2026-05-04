# AOC 2025

Advent of Code 2025 solutions in Rust.

## Project Structure

```
rs_aoc/
├── new_day.sh              # Generator script for new days
├── templates/              # Templates used by the generator
│   ├── dayXX.rs
│   └── dayXX_test.rs
├── src/
│   ├── main.rs             # CLI: cargo run -- <day>
│   ├── lib.rs              # Input helpers
│   └── days/
│       ├── mod.rs           # Day registry + Solution trait
│       └── day01.rs         # Day 01 solution (generated)
├── tests/
│   └── day01_test.rs        # Day 01 tests (generated)
└── input/
    ├── day01.txt            # Real puzzle input
    └── day01_sample.txt     # Sample input from problem
```

## Quick Start

### Generate a new day

```bash
./new_day.sh 1
```

This creates:
- `src/days/day01.rs` — solution stub
- `tests/day01_test.rs` — test file with sample + real input tests
- `input/day01.txt` — place your real input here
- `input/day01_sample.txt` — place the sample input here

### Run a solution

```bash
cargo run -- 1        # run day 1
cargo run -- all      # run all implemented days
```

### Test a day

```bash
cargo test --test day01_test                       # sample tests only
cargo test --test day01_test -- --include-ignored   # include real-input tests
```

## Workflow

1. `./new_day.sh <day>` to scaffold the day
2. Paste sample input into `input/dayXX_sample.txt`
3. Paste real input into `input/dayXX.txt`
4. Implement `part1` and `part2` in `src/days/dayXX.rs`
5. Update expected values in `tests/dayXX_test.rs`
6. Iterate with `cargo test --test dayXX_test`
