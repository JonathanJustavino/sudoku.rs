use core::fmt;
use std::collections::HashMap;
use std::{vec, collections::BTreeSet, iter::FromIterator,};
use rand::{Rng, seq::SliceRandom};

use crate::game_grid::Grid;


pub struct Cache {
    pub fixed_positions: Vec<Vec<usize>>,
}


impl Cache {
    pub fn new(grid: &Grid) -> Self {
        let mut fixed_positions: Vec<Vec<usize>> = vec![vec![]; 9];
        for (index, row) in grid.matrix.iter().enumerate() {
            let mut fixed = gather_fixed_indices(&row.to_vec());
            fixed_positions[index].append(&mut fixed);
        }

        Self { fixed_positions: fixed_positions }
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("Cache\n").to_owned();

        for (index, row) in self.fixed_positions.iter().enumerate() {
            output.push_str(&format!("row {} -> [", index + 1));
            let mut row_iter = row.iter().peekable();
            while let Some(col_value) = row_iter.next() {
                if row_iter.peek().is_none() {
                    output.push_str(&format!(" {} ]\n", *col_value));
                    continue;
                }
                output.push_str(&format!(" {},", *col_value));
            }
        }

        let print = output.to_string();
        write!(f, "{}", print)
    }
}


pub fn fitness_score_grid(solution: &Vec<u8>, row: &[u8;9]) -> usize {
    // Yields amount of overlapping values of vector and slice
    let mut conflicts: usize = 0;
    let collisions = row.iter()
                                    .enumerate()
                                    .filter(|(index, item)| **item == solution[*index] && **item > 0).count();
    conflicts += collisions;

    conflicts
}

pub fn fitness_grid<'a>(solution: &Vec<u8>, index: usize, grid: &'a Grid) -> Vec<(usize, &'a[u8; 9])>{
    // Scores solution based on conflicts in neighborhood
    let mut ranking: Vec<(usize, &[u8; 9])> = vec![];

    for (row_index, neighbor) in grid.matrix.iter().enumerate() {
        if index == row_index {
            continue;
        }

        let score = fitness_score_grid(&solution, &neighbor);
        let item = (score, neighbor);
        ranking.push(item);
    }

    ranking
}

pub fn evaluate_solution(solution: &Vec<u8>, index: usize, grid: &Grid)  -> usize {
    let ranking = fitness_grid(solution, index, grid);
    let score = ranking.iter().map(|(value, _) | *value ).sum();

    score
}

pub fn accept2<'a>(new: &'a (usize, &'a Vec<u8>), old: &'a (usize, &'a Vec<u8>), current_temperature: &f64) -> &'a (usize, &'a Vec<u8>) {
    let new_score = new.0;
    let old_score = old.0;
    if new_score < old_score {
        return new
    }

    let delta = new_score as f64 - old_score as f64;

    // 1 / (1 + e^( eval(v_current) - eval(v_n) ) / T)
    let criteria = 1.0 / (1.0 + libm::exp(delta / current_temperature));

    if criteria > 0.5 {
        return new
    }

    old
}

pub fn accept<'a>(new: (usize, Vec<u8>), old: (usize, Vec<u8>), current_temperature: f64) -> (usize, Vec<u8>) {
    let new_score = new.0;
    let old_score = old.0;
    if new_score < old_score {
        return new
    }

    let delta = new_score as f64 - old_score as f64;

    // 1 / (1 + e^( eval(v_current) - eval(v_n) ) / T)
    let criteria = 1.0 / (1.0 + libm::exp(delta / current_temperature));

    if criteria > 0.5 {
        return new
    }

    old
}


pub fn initial_assignment(grid: &mut Grid, cache: &Cache) {
    let mut matrix = grid.matrix;

    for (index, row) in matrix.iter_mut().enumerate() {
        let mut current_row = row.to_vec();
        generate_solution_fixed(&mut current_row, index, cache);
        assign_solution(current_row, index, grid);
    }
}


pub fn gather_fixed_indices(row: &Vec<u8>) -> Vec<usize> {
    let fixed_indices: Vec<usize> = row.iter()
                                        .enumerate()
                                        .filter(|(_, &value)| value != 0)
                                        .map(|(i, _)| i)
                                        .collect();
    fixed_indices
}


pub fn gather_free_indices(row_index: usize, cache: &Cache) -> Vec<usize> {
    let mut free_positions: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut fixed_positions = cache.fixed_positions[row_index].clone();

    fixed_positions.copy_from_slice(cache.fixed_positions[row_index].as_slice());
    let fixed_positions = BTreeSet::from_iter(fixed_positions);
    free_positions.retain(|value| !fixed_positions.contains(value));
    let free_values = free_positions.iter()
                                        .map(|value| *value as usize)
                                        .collect();

    free_values
}


pub fn gather_value_pool(row: &Vec<u8>, index: usize, cache: &Cache) -> Vec<u8> {
    let mut fixed_values = vec![];

    for idx in &cache.fixed_positions[index] {
        fixed_values.push(row[*idx])
    }

    let fixed_values = BTreeSet::from_iter(fixed_values);

    let mut free_values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    free_values.retain(|value| !fixed_values.contains(value));

    free_values
}
 

pub fn generate_solution_fixed(row: &mut Vec<u8>, row_index: usize, cache: &Cache){
    let free_positions = gather_free_indices(row_index, cache);
    let mut pool = gather_value_pool(&row, row_index, cache);
    let mut rng = rand::thread_rng();

    pool.shuffle(&mut rng);

    for iterator in pool.iter().zip(free_positions.iter()) {
        let (value, position) = iterator;
        row[*position] = *value;
    }
}


pub fn swap(solution: &mut Vec<u8>) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let first = rng.gen_range(1..9);
    let second = rng.gen_range(1..9);
    let first_value = solution[first];

    solution[first] = solution[second];
    solution[second] = first_value;

    (first, second)
}


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


pub fn generate_neighbourhood(solution: Vec<u8>, row_index: usize, amount: u8, cache: &Cache) -> Vec<Vec<u8>> {
    let mut neighbourhood: Vec<Vec<u8>> = vec![];

    for _ in 0..amount {
        let mut neighbour = solution.to_vec();
        generate_solution_fixed(&mut neighbour, row_index, &cache);
        // let mut candidate = vec![neighbour.clone()];
        // neighbourhood.append(&mut candidate);
        neighbourhood.push(neighbour);
    }

    neighbourhood
}

pub fn assign_solution(solution: Vec<u8>, index: usize, grid: &mut Grid) {
    let new = match TryFrom::try_from(solution) {
        Ok(ba) => ba,
        Err(_) => panic!("Could not convert solution vec to [ ]")
    };
    grid.matrix[index] = new;
}


pub fn evaluate_grid(grid: &Grid) -> usize {
    let length: usize = 8;
    let mut total_conflicts = 0;
    let matrix = grid.matrix;

    for (start, row) in matrix.iter().enumerate() {
        let solution = row.to_vec();
        for index in start..length {
            let next = index + 1;
            let neighbor = matrix[next];
            let collisions = fitness_score_grid(&solution, &neighbor);
            total_conflicts += collisions;
        }
    }

    total_conflicts
}

pub fn explore(item_index: usize, grid: &Grid, cache: &Cache, temperature: f64) -> (usize, Vec<u8>) {
    let start = grid.matrix[0].to_vec();
    let mut current_solution_score: usize = evaluate_solution(&start, item_index, &grid);
    let mut current_solution: (usize, Vec<u8>) = (current_solution_score, start);

    for (index, row) in grid.matrix.iter().enumerate() {
        if index == item_index {
            continue;
        }
        // Select current point
        let solution = row.to_vec();
        // Generate Neighborhood from current point
        let neighborhood = generate_neighbourhood(solution.clone(), index, 9, &cache);
        // Evaluate current point
        current_solution_score = evaluate_solution(&solution, index, &grid);
        let current = (current_solution_score, solution.clone());
        current_solution = current;

        for neighbor in neighborhood {
            // Evaluate neighbor
            let neighbor_score = evaluate_solution(&solution, index, &grid);
            // Select new current point
            current_solution = accept((neighbor_score, neighbor), current_solution, temperature);
        }
    }

    current_solution

}

pub fn anneal(mut grid: &mut Grid, temperature: f64, cooling_ratio: f64, total_attempts: u32) {
    // initialize temperature
    let mut init_grid: [[u8; 9]; 9] = [[0; 9]; 9];

    init_grid.copy_from_slice(&grid.matrix);
    let cache_grid = Grid::new(init_grid);
    println!("{}", grid);

    let cache = Cache::new(&cache_grid);
    initial_assignment(&mut grid, &cache);
    let conflicts = evaluate_grid(&grid);

    if conflicts == 0 {
        println!("{}", cache);
        println!("{}", grid);
        return;
    }

    let mut current_temperature = temperature;

    for _ in 0..total_attempts {
        // for (index, row) in grid.matrix.iter().enumerate() {
        for index in 0..grid.matrix.len() {
            let new = explore(index, grid, &cache, temperature);
            assign_solution(new.1, index, grid);
            let conflicts = evaluate_grid(&grid);

            if conflicts == 0 {
                println!("{}", cache);
                println!("{}", grid);
                return;
            }
        }

        // let conflicts = evaluate_grid(&grid);
        current_temperature = current_temperature * cooling_ratio;
    }

    let log_start = "+";
    let log_dash = "-";

    println!("{}{}{}", log_start, log_dash.repeat(10), log_start);
    println!("Solution not found!");
    println!("{}{}{}", log_start, log_dash.repeat(10), log_start);
    println!("{}", cache);
    println!("{}", grid);
}


#[cfg(test)]
mod annealing_tests;