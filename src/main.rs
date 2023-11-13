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
    let matrix:[[u8; 9]; 9] = [
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
        [0, 0, 1, 0, 4, 0, 7, 0, 9],
    ];

    let grid = Grid{matrix};

    let cache = annealing::Cache::new(&grid);

    println!("{}", cache);
    // let solver: Solver = Solver::new(grid);

    // println!("{}", solver.grid);
    // println!("{:?}", solver);

    let mut row: Vec<u8> = vec![0, 0, 1, 0, 4, 0, 7, 0, 9];
    println!("{:?}", row);
    let row_index: usize = 1;
    let free = annealing::gather_free_indices(row_index, &cache);
    println!("{:?}", free);
    // annealing::gather_fixed(&row);
    // annealing::gather_free_indices(&row);

    annealing::generate_initial_solution_fixed(&mut row, 4 as usize, &cache);

    let collisions = annealing::amount_of_conflicts(&row, 0 as usize, &grid);

    println!("conflicting values: {}", collisions);

    // let sln = annealing::generate_solution();

    // annealing::amount_of_conflicts(grid, sln);

}
