mod game_grid;
mod solver;
mod annealing;
mod utils;

use std::collections::BTreeSet;

use crate::annealing::Cache;
use crate::game_grid::Grid;
use crate::solver::Solver;

//TODO: https://docs.rs/heapless/latest/heapless/struct.Vec.html use heapless vectors

// use crate::annealing;


fn main() {

    // let matrix:[[u8; 9]; 9] = [
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    //     [0, 0, 1, 0, 4, 0, 7, 0, 9],
    // ];

    // let mut grid = Grid{matrix};

    // let cache = annealing::Cache::new(&grid);

    // // println!("{}", cache);

}
