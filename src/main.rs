mod game_grid;
mod solver;
mod annealing;
mod utils;

use annealing::{log, log_headline, evaluate_solution};

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

    let mut grid = setup_empty_example();
    let temperature: f64 = 30.0;
    let cooling_ratio = 0.98;
    let total_runs = 6000;

    annealing::anneal(&mut grid, temperature, cooling_ratio, total_runs);

    annealing::log_headline("Solution would be", &setup_solved_example());

    // let matrix: [[u8; 9]; 9] = [
    //    [ 7, 8, 6, 5, 4, 1, 2, 9, 3, ],
    //    [ 1, 4, 2, 6, 3, 5, 7, 8, 9, ],
    //    [ 6, 5, 3, 2, 8, 4, 9, 1, 7, ],
    //    [ 5, 9, 4, 1, 2, 7, 6, 3, 8, ],
    //    [ 8, 2, 1, 7, 3, 9, 5, 4, 6, ],
    //    [ 3, 7, 5, 9, 6, 8, 4, 2, 1, ],
    //    [ 2, 1, 9, 3, 5, 6, 8, 7, 4, ],
    //    [ 9, 6, 8, 4, 7, 3, 1, 5, 2, ],
    //    [ 4, 1, 7, 8, 9, 2, 3, 6, 5, ],
    // ];
        // -------------------------
        // | 7 8 6 | 5 4 1 | 2 9 3 |
        // | 1 4 2 | 6 3 5 | 7 8 9 |
        // | 6 5 3 | 2 8 4 | 9 1 7 |
        // -------------------------
        // | 5 9 4 | 1 2 7 | 6 3 8 |
        // | 8 2 1 | 7 3 9 | 5 4 6 |
        // | 3 7 5 | 9 6 8 | 4 2 1 |
        // -------------------------
        // | 2 1 9 | 3 5 6 | 8 7 4 |
        // | 9 6 8 | 4 7 3 | 1 5 2 |
        // | 4 1 7 | 8 9 2 | 3 6 5 |
        // -------------------------

    // let grid: Grid = Grid::new(matrix);
    // let index: usize = 0;
    // let mut solution = grid.matrix[index].clone().to_vec();

    // let rank = evaluate_solution(&solution, index, &grid);
    // println!("{}", rank);


    // let matrix: [[u8;9];9] = [        //----------------------------------
    // [ 2, 8, 6, 5, 4, 3, 7, 9, 1 ],    //| 2, 8, 6, | 5, 4, 3, | 7, 9, 1, |
    // [ 1, 4, 2, 6, 9, 8, 5, 3, 7 ],    //| 1, 4, 2, | 6, 9, 8, | 5, 3, 7, |
    // [ 6, 7, 3, 2, 8, 5, 4, 1, 9 ],    //| 6, 7, 3, | 2, 8, 5, | 4, 1, 9, |
    //                                     //----------------------------------
    // [ 5, 6, 4, 9, 2, 1, 3, 7, 8 ],    //| 5, 6, 4, | 9, 2, 1, | 3, 7, 8, |
    // [ 8, 3, 5, 7, 1, 9, 2, 4, 6 ],    //| 8, 3, 5, | 7, 1, 9, | 2, 4, 6, |
    // [ 3, 2, 1, 4, 6, 7, 9, 8, 5 ],    //| 3, 2, 1, | 4, 6, 7, | 9, 8, 5, |
    //                                     //----------------------------------
    // [ 9, 1, 7, 3, 5, 6, 8, 2, 4 ],    //| 9, 1, 7, | 3, 5, 6, | 8, 2, 4, |
    // [ 7, 9, 8, 6, 3, 4, 1, 5, 2 ],    //| 7, 9, 8, | 6, 3, 4, | 1, 5, 2, |
    // [ 4, 5, 9, 8, 7, 2, 1, 6, 3 ],    //| 4, 5, 9, | 8, 7, 2, | 1, 6, 3, |
    // ];                                //----------------------------------

    // let grid: Grid = Grid::new(matrix);


    // log_headline("headline", &grid);

}
