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
        [1, 2, 3, 4, 5, 6, 7, 8, 9], 
        [1, 2, 3, 4, 5, 6, 7, 8, 9], 
        [1, 2, 3, 4, 5, 6, 7, 8, 9], 
        [1, 2, 3, 4, 5, 6, 7, 8, 9], 
        [1, 2, 3, 4, 5, 6, 7, 8, 9], 
        [9, 8, 7, 6, 5, 4, 3, 2, 1], 
        [9, 8, 7, 6, 5, 4, 3, 2, 1], 
        [9, 8, 7, 6, 5, 4, 3, 2, 1], 
        [9, 8, 7, 6, 5, 4, 3, 2, 1], 
    ];

    let sln: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let neighborhood: Vec<Vec<u8>> = vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 
    ];

    let mut score = annealing::fitness(&sln, &neighborhood);

    let min = score.iter().min().unwrap();

    println!("{:?}", min);

}
