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


pub fn fitness_score(solution: &Vec<u8>, neighbor: &Vec<u8>) -> usize {
    let mut conflicts: usize = 0;
    let collisions = neighbor.iter()
                                    .enumerate()
                                    .filter(|(index, item)| **item == solution[*index] && **item > 0).count();
    conflicts += collisions;

    conflicts
}


pub fn fitness<'a>(solution: &Vec<u8>, neighborhood: &'a Vec<Vec<u8>>, ) -> Vec<(usize, &'a Vec<u8>)>{
    let mut ranking: Vec<(usize, &Vec<u8>)> = vec![];

    for neighbor in neighborhood.iter() {
        let score = fitness_score(&solution, &neighbor);
        let item = (score, neighbor);
        ranking.push(item);
    }

    ranking
}


pub fn accept<'a>(new: &'a (usize, &'a Vec<u8>), old: &'a (usize, &'a Vec<u8>), current_temperature: f64) -> &'a (usize, &'a Vec<u8>) {
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
    let length: usize = 9;

    for (index, row) in matrix.iter_mut().enumerate() {
        let mut current_row = row.to_vec();
        generate_solution_fixed(&mut current_row, index, cache);

        for index in 0..length {
            row[index] = current_row[index];
        }
    }

    grid.matrix = matrix;
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


pub fn gather_value_pool(row: &Vec<u8>) -> Vec<u8> {
    let fixed_values = BTreeSet::from_iter(row);
    let mut free_values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    free_values.retain(|value| !fixed_values.contains(value));

    free_values
}
 

pub fn generate_solution_fixed(row: &mut Vec<u8>, row_index: usize, cache: &Cache){
    let free_positions = gather_free_indices(row_index, cache);
    let mut pool = gather_value_pool(&row);
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
        let mut candidate = vec![neighbour.clone()];

        neighbourhood.append(&mut candidate);
    }

    neighbourhood
}


pub fn evaluate_grid(grid: &Grid) -> usize {
    let length: usize = 8;
    let mut total_conflicts = 0;
    let matrix = grid.matrix;

    for (start, row) in matrix.iter().enumerate() {
        let solution = row.to_vec();
        for index in start..length {
            let next = index + 1;
            let neighbor = matrix[next].to_vec();
            let collisions = fitness_score(&solution, &neighbor);
            total_conflicts += collisions;
        }
    }

    total_conflicts
}

pub fn anneal(mut grid: &mut Grid, max_temperature: f64) {
    // initialize temperature
    let cooling_ration = 0.95;
    let mut temperature = max_temperature;
    let mut init_grid: [[u8; 9]; 9] = [[0; 9]; 9];

    init_grid.copy_from_slice(&grid.matrix);
    let cache_grid = Grid::new(init_grid);
    println!("{}", grid);

    let cache = Cache::new(&cache_grid);
    initial_assignment(&mut grid, &cache);

    println!("{}", cache);
    println!("{}", grid);
}

pub fn search() {

}

#[cfg(test)]
mod annealing_tests;