use core::fmt;
use std::{vec, collections::BTreeSet, iter::FromIterator,};
use rand::{Rng, seq::SliceRandom};

use crate::game_grid::Grid;


pub struct Cache {
    pub fixed_positions: Vec<Vec<usize>>,
}


impl Cache {

    pub fn new(grid: &Grid) -> Self {
        println!("{}", grid);
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


pub fn amount_of_conflicts(solution: &Vec<u8>, row_index: usize, grid: &Grid) -> usize {
    let mut conflicts: usize = 0;

    let mut free_values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for (index, row) in grid.matrix.iter().enumerate() {
        if index == row_index {
            continue;
        }

        let fixed_values = BTreeSet::from_iter(row);
        free_values.retain(|value| !fixed_values.contains(value));
        let collisions = row.into_iter().filter(|item| solution.contains(item)).count();
        conflicts += collisions;
    }

    conflicts
}


pub fn conflicts_per_row(solution: &Vec<u8>, neighbor: &Vec<u8>) -> usize {
    let mut conflicts: usize = 0;
    let collisions = neighbor.into_iter().filter(|item| solution.contains(item)).count();
    conflicts += collisions;

    conflicts
}


pub fn initial_assignment(row: &Vec<u8>) {
    println!("initial assignment");
    let mut fixed = gather_fixed_indices(row);

}


pub fn gather_fixed_indices(row: &Vec<u8>) -> Vec<usize> {
    let fixed_indices: Vec<usize> = row.iter()
                                        .enumerate()
                                        .filter(|(_, &value)| value != 0)
                                        .map(|(i, _)| i)
                                        .collect();
    fixed_indices
}


pub fn gather_free_indices(row: &Vec<u8>) -> Vec<usize> {
    let available_indices: Vec<usize> = row.iter()
                                        .enumerate()
                                        .filter(|(_, &value)| value == 0)
                                        .map(|(i, _)| i)
                                        .collect();
    available_indices
}


pub fn gather_value_pool(row: &Vec<u8>) -> Vec<u8> {
    let fixed_values = BTreeSet::from_iter(row);
    let mut free_values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    free_values.retain(|value| !fixed_values.contains(value));

    free_values
}
 

pub fn generate_solution_fixed(row: &mut Vec<u8>){
    let free_positions = gather_free_indices(&row);
    let mut pool = gather_value_pool(&row);
    let mut rng = rand::thread_rng();

    pool.shuffle(&mut rng);

    for iterator in pool.iter().zip(free_positions.iter()) {
        let (value, position) = iterator;
        row[*position] = *value;
    }
}


// pub fn generate_solution() -> Vec<u8> {
//     let mut rng = rand::thread_rng();
//     let mut sampled = rand::seq::index::sample(&mut rng, 9, 9).into_vec();
//     sampled.iter_mut().for_each(|x| *x += 1);
//     let result = sampled.iter().map(|&e| e as u8).collect();

//     result
// }


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

pub fn generate_neighbourhood(solution: Vec<u8>, amount: u8) -> Vec<Vec<u8>> {
    let mut neighbourhood: Vec<Vec<u8>> = vec![];

    for _ in 0..amount {
        let mut neighbour = solution.to_vec();
        swap(&mut neighbour);
        let mut x = vec![neighbour];

        neighbourhood.append(&mut x);
    }

    neighbourhood
}

use crate::utils;
#[cfg(test)]
mod annealing_tests;