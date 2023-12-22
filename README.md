# [Advent of Code](https://adventofcode.com/)
Is yearly advent calendar where, between the 1st of December and the 25th, a new short programming puzzle is revealed each day. This free yearly event where you can participate using any programming language has been hosted by Eric Wastl since 2015.

## This repository
In 2023 I decided to pick up Rust and use the puzzles of prior events as fun little excercises. My solutions to these excercises are stored in this repository. My goal with these soltions is not to be the fastest, but to write elegant, extendable, idiomatic rust code, although I do try to keep my solutions fast by having low algorithmic complexity. I do also tend to go back to earlier solutions and apply newfound knowledge or abstractions to improve these already working solutions.

The repository is divided in four distinct projects:
1. `aoc_solvers` This is a library crate that contains so called "solvers" to solve the puzzles.
2. `aoc_lib` This is a library crate containing generic code to be reused throughout many different puzzles such as parsing, grids or specific datastructures.
3. `aoc_runner` This is the binary crate that executes the puzzles and contains code to read inputs, verify solutions for regressions, time the solutions, and execute puzzles in batches.
4. `aoc_runner_api` This is a library crate that acts as the interface or contract implemented by `aoc_solvers` to provide solutions to the `aoc_runner`.

## Executing the puzzles
It is against advent of code rules to store puzzle answers or solutions in your repository as they are considered "private information." This repository respects this wish and therefor requires you to load *your own* puzzle inputs in the `aoc_runner/inputs/` directory under the correct year. The inputs follow the format `day_xx.txt` where `xx` is the day number padded out on the left with a zero if applicable e.g. `day_08.txt`.

The solutions are contained in the `aoc_runner/solutions/` directory, also by their respective year, following the same `day_xx.txt` format as the inputs. The only catch is that the solutions are seperated by a `;` followed by a newline. The semicolon is there in the case of multi-line solutions that sometimes visually spell the actual answer.

To run the puzzles, navigate to the `aoc_runner` directory, the runner supports running various scopes:
- `cargo run` will run **all** the solvers registered by `aoc_solvers`.
- `cargo run 2023` will only run the puzzles from 2023.
- `cargo run 2023-12` will only run the puzzles from 2023 day 12.
- `cargo run 2023-12-1` will only run the first part of 2023 day 12.

Additionally it is possible to prefix any of the above commands with `verify`, e.g. `cargo run verify 2023-12`, this will verify if the solvers for 2023 day 12 produce the expected answers.