mod game_grid;
mod solver;
mod annealing;
mod utils;
mod grid;

use annealing::{evaluate_solution, log, log_headline, Cache};
use rand::seq::index;
use ndarray::{array, Array2};
use std::path::Path;
use std::env::current_dir;

// use crate::game_grid::Grid;
use crate::grid::Grid;


//TODO: https://docs.rs/heapless/latest/heapless/struct.Vec.html use heapless vectors

fn main() {

    let mut grid = Grid::from_file("solved.txt");
    println!("{}", grid);
}

