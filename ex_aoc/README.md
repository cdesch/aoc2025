# ExAoc

Advent of Code 2025 solutions in Elixir.

## Project Structure

```
ex_aoc/
├── new_day.sh                # Generator script for new days
├── templates/                # Templates used by the generator
│   ├── day_xx.ex
│   └── day_xx_test.exs
├── lib/
│   ├── ex_aoc.ex             # Input helpers + runner
│   └── ex_aoc/
│       └── day_01.ex         # Day 01 solution (generated)
├── test/
│   ├── test_helper.exs       # ExUnit config (excludes :real by default)
│   └── day_01_test.exs       # Day 01 tests (generated)
└── input/
    ├── day_01.txt            # Real puzzle input
    └── day_01_sample.txt     # Sample input from problem
```

## Quick Start

### Generate a new day

```bash
./new_day.sh 1
```

This creates:
- `lib/ex_aoc/day_01.ex` — solution module with `part1/1` and `part2/1`
- `test/day_01_test.exs` — tests for sample and real input
- `input/day_01.txt` — place your real input here
- `input/day_01_sample.txt` — place the sample input here

### Run a solution

```bash
mix run -e 'ExAoc.run(1)'
```

### Test a day

```bash
mix test test/day_01_test.exs                  # sample tests only
mix test test/day_01_test.exs --include real    # include real-input tests
```

## Workflow

1. `./new_day.sh <day>` to scaffold the day
2. Paste sample input into `input/day_XX_sample.txt`
3. Paste real input into `input/day_XX.txt`
4. Implement `part1/1` and `part2/1` in `lib/ex_aoc/day_XX.ex`
5. Update expected values in `test/day_XX_test.exs`
6. Iterate with `mix test test/day_XX_test.exs`
