# Advent of Code 2023

I probably won't finish it ¯\\\_(ツ)\_/¯

---

## Running

```
Usage: aoc2023.exe [OPTIONS] [PUZZLE]

Arguments:
  [PUZZLE]  Optional puzzle to run

Options:
  -p, --part <PART>      Optional part to run
  -r, --rounds <ROUNDS>  Benchmarking rounds [default: 1]
  -h, --help             Print help
```

Run all puzzles
```sh
> cargo run
```

Run a single puzzle (puzzle 1)
```sh
> cargo run -- 1
```

Run a single part of all puzzles (part 2)
```sh
> cargo run -- --part 2
```

Benchmark all puzzles (100 rounds)
```sh
> cargo run -- --rounds 100
```

Benchmark a single part of a single puzzle (puzzle 1, part 2, 100 rounds)
```sh
> cargo run -- --part 2 --rounds 100 1
```
