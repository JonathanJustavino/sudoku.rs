mod annealing;
mod game_grid;
mod grid;
mod utils;

// use annealing::{log, log_headline};
use dotenv::dotenv;
// use crate::game_grid::Grid;
use crate::grid::Grid;

//TODO: https://docs.rs/heapless/latest/heapless/struct.Vec.html use heapless vectors

fn main() {
    dotenv().ok();

    // -------------------------
    // | 0 8 0 | 5 0 0 | 0 0 0 |
    // | 1 4 2 | 0 0 0 | 0 0 0 |
    // | 6 0 3 | 0 8 0 | 0 1 0 |
    // -------------------------
    // | 0 0 4 | 0 2 0 | 0 0 8 |
    // | 8 0 0 | 7 0 9 | 0 0 6 |
    // | 3 0 0 | 0 6 0 | 0 0 0 |
    // -------------------------
    // | 0 1 0 | 0 5 0 | 8 0 4 |
    // | 0 0 0 | 0 0 0 | 1 5 2 |
    // | 0 0 0 | 0 0 2 | 0 6 0 |
    // -------------------------

    // let mut grid = Grid::from_file("empty_example.txt");
    let mut grid = Grid::from_file("empty_example.txt");
    println!("{}", grid);
    let cooling_ratio = 0.99;
    annealing::anneal(&mut grid, cooling_ratio);

    // grid.initialize();

    // println!("{}", grid);
    // let conflicts = annealing::check_completeness(&grid.matrix);
    // println!("{}", conflicts);

    // let temp = annealing::calculate_temperature(&grid);
    // println!("{}", temp);

    // let res = annealing::estimate_attempts(&grid.fixed_subgrid_positions);
}
