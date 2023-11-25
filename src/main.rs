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

fn setup_empty_example() -> Grid {
    let matrix:[[u8; 9]; 9] = [        // -------------------------
        [0, 8, 0, 5, 0, 0, 0, 0, 0],   // | 0 8 0 | 5 0 0 | 0 0 0 |
        [1, 4, 2, 0, 0, 0, 0, 0, 0],   // | 1 4 2 | 0 0 0 | 0 0 0 |
        [6, 0, 3, 0, 8, 0, 0, 1, 0],   // | 6 0 3 | 0 8 0 | 0 1 0 |
                                        // -------------------------
        [0, 0, 4, 0, 2, 0, 0, 0, 8],   // | 0 0 4 | 0 2 0 | 0 0 8 |
        [8, 0, 0, 7, 0, 9, 0, 0, 6],   // | 8 0 0 | 7 0 9 | 0 0 6 |
        [3, 0, 0, 0, 6, 0, 0, 0, 0],   // | 3 0 0 | 0 6 0 | 0 0 0 |
                                        // -------------------------
        [0, 1, 0, 0, 5, 0, 8, 0, 4],   // | 0 1 0 | 0 5 0 | 8 0 4 |
        [0, 0, 0, 0, 0, 0, 1, 5, 2],   // | 0 0 0 | 0 0 0 | 1 5 2 |
        [0, 0, 0, 0, 0, 2, 0, 6, 0],   // | 0 0 0 | 0 0 2 | 0 6 0 |
    ];                                 // -------------------------

    let grid = Grid { matrix: matrix };

    grid
}


fn setup_solved_example() -> Grid {
    let matrix:[[u8; 9]; 9] = [        // -------------------------
        [7, 8, 9, 5, 4, 1, 6, 2, 3],   // | 7 8 9 | 5 4 1 | 6 2 3 |
        [1, 4, 2, 6, 9, 3, 7, 8, 5],   // | 1 4 2 | 6 9 3 | 7 8 5 |
        [6, 5, 3, 2, 8, 7, 4, 1, 9],   // | 6 5 3 | 2 8 7 | 4 1 9 |
                                        // -------------------------
        [9, 6, 4, 1, 2, 5, 3, 7, 8],   // | 9 6 4 | 1 2 5 | 3 7 8 |
        [8, 2, 1, 7, 3, 9, 5, 4, 6],   // | 8 2 1 | 7 3 9 | 5 4 6 |
        [3, 7, 5, 8, 6, 4, 2, 9, 1],   // | 3 7 5 | 8 6 4 | 2 9 1 |
                                        // -------------------------
        [2, 1, 7, 9, 5, 6, 8, 3, 4],   // | 2 1 7 | 9 5 6 | 8 3 4 |
        [4, 9, 6, 3, 7, 8, 1, 5, 2],   // | 4 9 6 | 3 7 8 | 1 5 2 |
        [5, 3, 8, 4, 1, 2, 9, 6, 7],   // | 5 3 8 | 4 1 2 | 9 6 7 |
    ];                                 // -------------------------

    let grid = Grid { matrix: matrix };

    grid
}


fn main() {

    // let mut matrix:[[u8; 9]; 9] = [
    //     [0, 8, 0, 5, 0, 0, 0, 0, 0],
    //     [1, 4, 2, 0, 0, 0, 0, 0, 0],
    //     [6, 0, 3, 0, 8, 0, 0, 1, 0],
    //     [0, 0, 4, 0, 2, 0, 0, 0, 8],
    //     [8, 0, 0, 7, 0, 9, 0, 0, 6],
    //     [3, 0, 0, 0, 6, 0, 0, 0, 0],
    //     [0, 1, 0, 0, 5, 0, 8, 0, 4],
    //     [0, 0, 0, 0, 0, 0, 1, 5, 2],
    //     [0, 0, 0, 0, 0, 2, 0, 6, 0],
    // ];


    // let mut grid = Grid { matrix: matrix };

    let mut grid = setup_empty_example();
    let temperature: f64 = 1000.0;
    let cooling_ratio = 0.95;
    let total_runs = 100;

    annealing::anneal(&mut grid, temperature, cooling_ratio, total_runs);
}
