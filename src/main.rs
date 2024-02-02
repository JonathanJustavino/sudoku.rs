mod game_grid;
mod solver;
mod annealing;
mod utils;

use annealing::{log, log_headline, evaluate_solution};
use rand::seq::index;

use crate::game_grid::Grid;


//TODO: https://docs.rs/heapless/latest/heapless/struct.Vec.html use heapless vectors


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

fn setup_missing_row() -> Grid {
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
        [0, 0, 0, 0, 0, 0, 0, 0, 0],   // | 0 0 0 | 0 0 0 | 0 0 0 |
    ];                                 // -------------------------

    let grid = Grid { matrix: matrix };

    grid
}


fn main() {

    let faulty_solution: [[u8; 9]; 9] = [
        [7, 5, 6, 4, 3, 9, 8, 1, 2],
        [4, 9, 8, 1, 6, 2, 7, 5, 3],
        [8, 1, 3, 5, 9, 4, 2, 6, 7],
        [6, 7, 5, 3, 2, 1, 9, 8, 4],
        [2, 4, 9, 8, 7, 6, 5, 3, 1],
        [4, 9, 8, 1, 6, 2, 7, 5, 3],
        [3, 6, 1, 9, 5, 7, 4, 2, 8],
        [5, 8, 7, 2, 4, 3, 1, 9, 6],
        [9, 2, 4, 6, 1, 8, 3, 7, 5]
    ];

    let faulty_grid = Grid{matrix: faulty_solution};
    let mut score;
    let mut index = 8;
    println!("{}", faulty_grid);

    score = annealing::fitness_subgrid(&faulty_grid, index);
    assert_eq!(score, 2);
    println!("score {}", score);

    // index = 3;
    // score = annealing::fitness_subgrid(&faulty_grid, index);
    // assert_eq!(score, 3);
    // println!("score {}", score);

}
