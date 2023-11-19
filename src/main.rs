mod game_grid;
mod solver;
mod annealing;
mod utils;

use std::collections::BTreeSet;

use crate::annealing::Cache;
use crate::game_grid::Grid;
use crate::solver::Solver;

use libm; // for exp
use fast_math; // for exp

//TODO: https://docs.rs/heapless/latest/heapless/struct.Vec.html use heapless vectors

// use crate::annealing;


fn main() {

    let mut matrix:[[u8; 9]; 9] = [
        [0, 8, 0, 5, 0, 0, 0, 0, 0],
        [1, 4, 2, 0, 0, 0, 0, 0, 0],
        [6, 0, 3, 0, 8, 0, 0, 1, 0],
        [0, 0, 4, 0, 2, 0, 0, 0, 8],
        [8, 0, 0, 7, 0, 9, 0, 0, 6],
        [3, 0, 0, 0, 6, 0, 0, 0, 0],
        [0, 1, 0, 0, 5, 0, 8, 0, 4],
        [0, 0, 0, 0, 0, 0, 1, 5, 2],
        [0, 0, 0, 0, 0, 2, 0, 6, 0],
    ];


    let mut grid = Grid { matrix: matrix };
    let max_temperature = 0.95;

    annealing::anneal(&mut grid, max_temperature);
}
