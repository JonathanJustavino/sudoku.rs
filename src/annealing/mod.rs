use core::fmt;
use itertools::{izip, Itertools};
use ndarray::{s, Array, Array2, ArrayBase, ArrayView1, ArrayView2, ArrayViewMut1, Dim, Ix2, SliceInfo, SliceInfoElem, ViewRepr};
use rand::thread_rng;
use rand::{distributions::Open01, seq::SliceRandom, Rng};
use std::{collections::BTreeSet, io::stdout, iter::FromIterator, ops::Index, usize, vec};

use crate::{grid, utils};
use crate::grid::Grid;


pub fn fitness_col(grid: &Grid, index: usize) -> usize {
    grid.check_col(index)
}

pub fn fitness_row(grid: &Grid, index: usize) -> usize {
    grid.check_row(index)
}


pub fn fitness_score_cols(grid: &Grid) -> usize {
    let mut conflicts: usize = 0;
    let collisions = 0;

    for index in 0..9 {
        conflicts += fitness_col(grid, index);
    }

    conflicts
}

pub fn fitness_score_rows(grid: &Grid) -> usize {
    let mut conflicts: usize = 0;

    for index in 0..9 {
        conflicts += fitness_row(grid, index);
    }

    conflicts
}

pub fn generate_solution(grid: &Grid, index: usize) -> Array2<u8> {
    let current: ArrayView2<u8> = grid.get_subgrid(index);
    let mut rng = thread_rng();
    let fixed = &grid.fixed_subgrid_positions[index];

    let mut candidates: Vec<usize> = (0..9).collect();

    candidates.retain(|index| !fixed.contains(index));

    let swap: (&usize, &usize) = candidates.choose_multiple(&mut rng, 2).collect_tuple().unwrap();
    let mut sln: Array2<u8> = current.clone().to_owned();

    let first_index = Grid::map_to_grid(*swap.0);
    let second_index = Grid::map_to_grid(*swap.1);

    let value_1 = sln[[first_index.0, first_index.1]];
    sln[[first_index.0, first_index.1]] = sln[[second_index.0, second_index.1]];
    sln[[second_index.0, second_index.1]] = value_1;

    sln
}

// pub fn fitness_grid<'a>(solution: &Vec<u8>, index: usize, grid: &'a Grid) -> Vec<(usize, &'a[u8; 9])>{
//     // Scores solution based on conflicts in grid
//     let mut ranking: Vec<(usize, &[u8; 9])> = vec![];

//     for (row_index, neighbor) in grid.matrix.iter().enumerate() {
//         if index == row_index {
//             continue;
//         }

//         let score = fitness_score_row(&solution, &neighbor);
//         let item = (score, neighbor);
//         ranking.push(item);
//     }

//     ranking
// }

// pub fn evaluate_solution(solution: &Vec<u8>, index: usize, grid: &Grid)  -> usize {
//     let ranking = fitness_grid(solution, index, grid);
//     let score: usize = ranking.iter().map(|(value, _) | *value ).sum();
//     // let subgrid_score: usize = fitness_subgrid(grid, index);
//     let subgrid_score = fitness_subgrids(&grid);

//     score + subgrid_score
// }

pub fn accept<'a>(new: (usize, Vec<u8>), old: (usize, Vec<u8>), current_temperature: f64, debug_index: usize) -> (usize, Vec<u8>) {
    let new_score = new.0;
    let old_score = old.0;
    if new_score < old_score {
        return new
    }

    let delta = new_score as f64 - old_score as f64;

    // 1 / (1 + e^( eval(v_current) - eval(v_n) ) / T)
    // let criteria = -(1.0 / (1.0 + libm::exp(delta / current_temperature)));
    let criteria = 1.0 / (1.0 + libm::exp(delta / current_temperature));

    if criteria > 0.5 {
        return new
    }

    old
}


// pub fn generate_solution_fixed(row: &mut Vec<u8>, row_index: usize, cache: &Cache){
//     let free_positions = gather_free_indices(row_index, cache);
//     let mut pool = gather_value_pool(&row, row_index, cache);
//     let mut rng = rand::thread_rng();

//     pool.shuffle(&mut rng);

//     for (value, position) in pool.iter().zip(free_positions.iter()) {
//         // let (value, position) = iterator;
//         row[*position] = *value;
//     }
// }


// pub fn swap_values(solution: &mut Vec<u8>, index: usize, cache: &Cache) -> Option<(usize, usize)> {
//     let fixed_indexes = BTreeSet::from_iter(&cache.fixed_positions[index]);

//     if fixed_indexes.len() == 9 {
//         return None;
//     }

//     let mut pool: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
//     let mut rng_first = rand::thread_rng();
//     let mut rng_second = rand::thread_rng();

//     pool.retain(|value|!fixed_indexes.contains(value));
//     let mut second_pool = pool.clone();
//     let first = match pool.choose(&mut rng_first) {
//         Some(value) => *value,
//         None => 0,
//     };

//     second_pool.retain(|value| *value != first);
//     let second = match second_pool.choose(&mut rng_second) {
//         Some(value) => *value,
//         None => 0,
//     };

//     if first == second {
//         return None;
//     }

//     let first_value = solution[first];

//     solution[first] = solution[second];
//     solution[second] = first_value;

//     Some((first, second))
// }


pub fn swap_index_with_value(mut solution: Vec<u8>) -> Vec<u8> {
    let chunk_size = 2;
    let mut rng = rand::thread_rng();
    let amount = rng.gen_range(2..9);
    let mut sampled = rand::seq::index::sample(&mut rng, 9, amount).into_vec();
    let sampled_length = sampled.len();

    println!("sampled {:?}", sampled);

    if sampled_length % 2 != 0 {
        let first = sampled[0];
        sampled.push(first);
    }

    for pair in sampled.chunks(chunk_size) {
        solution.swap(pair[0], pair[1]);
    }

    solution
}


// pub fn generate_neighbourhood(solution: Vec<u8>, row_index: usize, amount: u8, cache: &Cache) -> Vec<Vec<u8>> {
//     let mut neighbourhood: Vec<Vec<u8>> = vec![];

//     for _ in 0..amount {
//         let mut neighbour = solution.clone();
//         //TODO: changing from solution fixed to swap
//         // generate_solution_fixed(&mut neighbour, row_index, &cache);

//         let indices = swap_values(&mut neighbour, row_index, &cache);
//         match indices {
//             Some((_, _)) => neighbourhood.push(neighbour),
//             None => continue
//         }
//     }

//     neighbourhood
// }

// pub fn assign_solution(solution: Vec<u8>, index: usize, grid: &mut Grid) {
//     let new = match TryFrom::try_from(solution) {
//         Ok(ba) => ba,
//         Err(_) => panic!("Could not convert solution vec to [ ]")
//     };
//     grid.matrix[index] = new;

//     // for (solution_index, value) in solution.iter().enumerate() {
//     //     grid.matrix[index][solution_index] = *value;
//     // }
// }


// pub fn evaluate_grid(new: &Vec<u8>, index: usize, grid: &Grid) -> usize {
//     let length: usize = 8;
//     let mut total_conflicts = 0;
//     let matrix = grid.matrix;
//     fitness_grid(new, index, &grid);

//     // for (start, row) in matrix.iter().enumerate() {
//     //     let solution = row.to_vec();
//     //     for index in start..length {
//     //         let next = index + 1;
//     //         let neighbor = matrix[next];
//     //         let collisions = fitness_score_grid(&solution, &neighbor);
//     //         total_conflicts += collisions;
//     //     }
//     // }

//     for (index, row) in grid.matrix.iter().enumerate() {


//     }

//     total_conflicts
// }


// pub fn check_completeness(grid: &Grid) -> usize {
//     let mut total_conflicts = 0;
//     let matrix = grid.matrix;

//     for (row_index, row) in matrix.iter().enumerate() {
//         let solution = row.to_vec();
//         for next_row in matrix.iter().skip(row_index + 1) {
//             let collisions = fitness_score_row(&solution, next_row);
//             total_conflicts += collisions;
//         }
//     }

//     let subgrid_conflicts = fitness_subgrids(grid);
//     total_conflicts + subgrid_conflicts
// }

// pub fn explore(item_index: usize, grid: &Grid, cache: &Cache, temperature: f64, neighbourhood_size: u8) -> (usize, Vec<u8>) {
//     // Select current point
//     let start = grid.matrix[item_index].to_vec();
//     let current_solution_score: usize = evaluate_solution(&start, item_index, &grid);
//     let mut current_solution: (usize, Vec<u8>) = (current_solution_score, start.clone());
//     // Generate Neighborhood from current point
//     let neighborhood = generate_neighbourhood(start.clone(), item_index, neighbourhood_size, cache);
//     let mut neighbor_solution_score: usize;

//     for neighbor in neighborhood.iter() {
//         // Evaluate current point
//         neighbor_solution_score = evaluate_solution(&neighbor, item_index, &grid);
//         current_solution = accept((neighbor_solution_score, neighbor.clone()), current_solution, temperature, item_index);
//     }

//     current_solution

// }

pub fn log(conflicts: usize) {
    let log_start = "+";
    let log_dash = "-";
    let white_space = " ";

    if conflicts == 0 {
        println!("{}{}{}", log_start, log_dash.repeat(23), log_start);
        println!("{}Solution found!", white_space.repeat(3));
        println!("{}{}{}", log_start, log_dash.repeat(23), log_start);
        return;
    }

    print!("\rconflicts: {}", conflicts);
    // stdout().flush().unwrap();
}

pub fn log_headline(headline: &str, grid: &Grid) {
    let white_space = " ";
    let headline_len = headline.len() / 2;
    let grid_log_half = 12;

    println!("{}{}\n{}", white_space.repeat(grid_log_half - headline_len), headline, grid);
}

// pub fn calculate_temperature(matrix: [[u8;9];9], cache_grid: Grid) -> f64 {

//     let mut assign_grid = Grid::new(matrix.clone());
//     let assign_cache = Cache::new(&cache_grid.clone());
//     const LENGTH: usize = 10;
//     let mut scores = [0.0 as f64; LENGTH];

//     for index in 0..10 {
//         initial_assignment(&mut assign_grid, &assign_cache);
//         let subgrid_score = fitness_subgrids(&assign_grid);
//         let grid_score = check_completeness(&assign_grid);
//         scores[index] = subgrid_score as f64 + grid_score as f64;
//     }

//     utils::compute_standard_deviation(&scores).unwrap()
// }

// pub fn anneal(mut grid: &mut Grid, cooling_ratio: f64, _total_attempts: u32, neighbourhood_size: u8) {
//     /*TODO:
//     1. calculate amount of tries per fixed sudoku
//     2. after the amount of tries test if temperature calculation is correct
//     */

//     // initialize temperature
//     let mut init_grid: [[u8; 9]; 9] = [[0; 9]; 9];

//     init_grid.copy_from_slice(&grid.matrix);
//     let cache_grid = Grid::new(init_grid);
//     log_headline("Initial Grid", grid);

//     let cache = Cache::new(&cache_grid);
//     let total_attempts: usize = cache.fixed_positions.iter().map(Vec::len).sum();
//     initial_assignment(&mut grid, &cache);

//     // calculate the std
//     let temperature = calculate_temperature(grid.matrix, cache_grid);
//     // let mut assign_grid = Grid::new(grid.matrix.clone());
//     // let assign_cache = Cache::new(&cache_grid);
//     // const LENGTH: usize = 10;
//     // let mut scores = [0.0 as f64; LENGTH];
//     // for index in 0..10 {
//     //     initial_assignment(&mut assign_grid, &assign_cache);
//     //     let subgrid_score = fitness_subgrids(&assign_grid);
//     //     let grid_score = check_completeness(&assign_grid);
//     //     scores[index] = subgrid_score as f64 + grid_score as f64;
//     // }
//     // let mut temperature = utils::compute_standard_deviation(&scores).unwrap();

//     log_headline("After initial assignment", grid);

//     let conflicts = check_completeness(&grid);

//     if conflicts == 0 {
//         log_headline("Solved", grid);
//         return;
//     }

//     let mut current_temperature = temperature;
//     let rows: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6 ,7, 8];

//     for _ in 0..total_attempts {

//         let random_row = rows.choose(&mut rand::thread_rng()).unwrap();
//         let index = *random_row as usize;
//         let new = explore(index, grid, &cache, temperature, neighbourhood_size);
//         assign_solution(new.1, index, grid);
//         let conflicts = check_completeness(grid);
//         log(conflicts);
//         if conflicts == 0 {
//             println!("{}", grid);
//             return;
//         }

//         current_temperature = current_temperature * cooling_ratio;
//     }

//     log_headline("Guess", grid);
//     println!("end temperature {}", current_temperature);
// }


#[cfg(test)]
mod annealing_tests;