# [Advent of Code](https://adventofcode.com/)
Is yearly advent calendar where, between the 1st of December and the 25th, a new short programming puzzle is revealed each day. This free yearly event where you can participate using any programming language has been hosted by Eric Wastl since 2015.

## This repository
In 2023 I decided to pick up Rust and use the puzzles of prior events as fun little excercises. My solutions to these excercises are stored in this repository. My goal with these solutions is not to be the fastest, but to write elegant, extendable, idiomatic rust code, although I do try to keep my solutions fast by having low algorithmic complexity. I do also tend to go back to earlier solutions and apply newfound knowledge or abstractions to improve these already working solutions.

## Executing the puzzles
This repository uses my own "[jikan](https://github.com/newo-2001/jikan)" Advent of Code framework to execute and time puzzles.

It is against advent of code rules to store puzzle answers or solutions in your repository as they are considered "private information." This repository respects this wish and therefor requires you to load *your own* puzzle inputs in the `aoc_runner/data/yyyy` directory under the correct year. The inputs follow the format `day_xx.yaml` where `xx` is the day number padded out on the left with a zero if applicable e.g. `data/2024/day_08.yaml`.

- `cargo run` will run **all** the solvers.
- `cargo run --scope 2023` will only run the puzzles from 2023.
- `cargo run --scope 2023-12` will only run the puzzles from 2023 day 12.
- `cargo run --scope 2023-12-1` will only run the first part of 2023 day 12.

Additionally it is possible to attach `--verify` to any of the above commands with e.g. `cargo run --verify --scope 2023-12`, this will verify if the solvers for 2023 day 12 produce the expected answers.
Or attach `--examples` to additionally execute to examples.
