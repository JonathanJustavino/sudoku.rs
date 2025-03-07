# Sudoku.rs

A sudoku solver written in rust using the metaheuristic algorithm: [simmulated annealing](https://www.adrian.idv.hk/2019-01-30-simanneal/).

This project was inspired by [ChallengingLuck](https://github.com/challengingLuck/youtube/blob/master/sudoku/sudoku.py).

## Use

1. List available Grids
    - Run `cargo run -- --list`
2. Run solver
    - Run `cargo run -- -g <GRID_NAME> -c <COOLING_RATIO>`
3. Run `cargo run -- --help` to see all available options

```zsh
A sudoku solver using simmulated annealing

Usage: sudoku [OPTIONS]

Options:
  -g, --grid-name <GRID_NAME>
          Name of the grid to solve (located under /static) [default: easy.txt]
  -c, --cooling-ration <COOLING_RATION>
          value of cooling ratio (typically in the range [0.99,..,0.85]) [default: 0.99]
  -l, --list
          List available grids
  -h, --help
          Print help
  -V, --version
          Print version
```

## Available Grids

Custom grids can be added under `repo/static/`.
The grids need to be provided in this format to be parsable:

```txt
004000120
098026300
030097005
871354296
000060080
056009000
905070002
020030000
683205907
```
