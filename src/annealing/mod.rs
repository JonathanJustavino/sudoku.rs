use core::fmt;
use std::{collections::BTreeSet, io::stdout, iter::FromIterator, ops::Index, usize, vec};
use rand::{distributions::Open01, seq::SliceRandom, Rng};
use itertools::{izip, Itertools};


use crate::utils;
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

pub fn fitness_subgrid(grid: &Grid, index: usize) -> usize {
    let stride = 3;
    let mut subgrid;
    let mut duplicates: usize = 0;
    let column_offset = index % 3;
    let row_offset: usize = match index {
        0 ..= 2 => 0,
        3 ..= 5 => 1,
        _ => 2,
    };

    for row_triple in grid.matrix.windows(stride).step_by(stride).skip(row_offset) {
        for (row_subgrid_0, row_subgrid_1, row_subgrid_2) in izip!(row_triple[0].windows(3), row_triple[1].windows(3), row_triple[2].windows(3)).step_by(3).skip(column_offset){
            subgrid = vec![];
            subgrid.extend_from_slice(row_subgrid_0);
            subgrid.extend_from_slice(row_subgrid_1);
            subgrid.extend_from_slice(row_subgrid_2);
            subgrid.sort();
            let uniques = subgrid.iter().unique().count();
            let sub_grid_collisions = 9 - uniques;
            duplicates += sub_grid_collisions;
            break;
        }
        break;
    }

    duplicates
}

pub fn fitness_subgrids(grid: &Grid) -> usize {
    let mut total_duplicates: usize = 0;
    let range: usize = 9;
    for index in 0..range {
        total_duplicates += fitness_subgrid(&grid, index);
    }
    total_duplicates
}

pub fn fitness_score_col(column_index: usize, matrix: &[[u8;9];9]) {
    let mut conflicts: usize = 0;
    let collisions = 0;

    for (index, item) in matrix.iter().enumerate() {
        println!("{}, {:?}", index, item[column_index]);
    }
}

pub fn fitness_score_row(sample_row: &Vec<u8>, row: &[u8;9]) -> usize {
    // Yields amount of overlapping values of vector and slice
    let mut conflicts: usize = 0;
    let collisions = row.iter()
                                    .enumerate()
                                    .filter(|(index, item)| **item == sample_row[*index] && **item > 0).count();
    conflicts += collisions;

    conflicts
}

pub fn fitness_grid<'a>(solution: &Vec<u8>, index: usize, grid: &'a Grid) -> Vec<(usize, &'a[u8; 9])>{
    // Scores solution based on conflicts in grid
    let mut ranking: Vec<(usize, &[u8; 9])> = vec![];

    for (row_index, neighbor) in grid.matrix.iter().enumerate() {
        if index == row_index {
            continue;
        }

        let score = fitness_score_row(&solution, &neighbor);
        let item = (score, neighbor);
        ranking.push(item);
    }

    ranking
}

pub fn evaluate_solution(solution: &Vec<u8>, index: usize, grid: &Grid)  -> usize {
    let ranking = fitness_grid(solution, index, grid);
    let score: usize = ranking.iter().map(|(value, _) | *value ).sum();
    // let subgrid_score: usize = fitness_subgrid(grid, index);
    let subgrid_score = fitness_subgrids(&grid);

    score + subgrid_score
}

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

    for (value, position) in pool.iter().zip(free_positions.iter()) {
        // let (value, position) = iterator;
        row[*position] = *value;
    }
}


pub fn swap_values(solution: &mut Vec<u8>, index: usize, cache: &Cache) -> Option<(usize, usize)> {
    let fixed_indexes = BTreeSet::from_iter(&cache.fixed_positions[index]);

    if fixed_indexes.len() == 9 {
        return None;
    }

    let mut pool: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut rng_first = rand::thread_rng();
    let mut rng_second = rand::thread_rng();

    pool.retain(|value|!fixed_indexes.contains(value));
    let mut second_pool = pool.clone();
    let first = match pool.choose(&mut rng_first) {
        Some(value) => *value,
        None => 0,
    };

    second_pool.retain(|value| *value != first);
    let second = match second_pool.choose(&mut rng_second) {
        Some(value) => *value,
        None => 0,
    };

    if first == second {
        return None;
    }

    let first_value = solution[first];

    solution[first] = solution[second];
    solution[second] = first_value;

    Some((first, second))
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
        let mut neighbour = solution.clone();
        //TODO: changing from solution fixed to swap
        // generate_solution_fixed(&mut neighbour, row_index, &cache);

        let indices = swap_values(&mut neighbour, row_index, &cache);
        match indices {
            Some((_, _)) => neighbourhood.push(neighbour),
            None => continue
        }
    }

    neighbourhood
}

pub fn assign_solution(solution: Vec<u8>, index: usize, grid: &mut Grid) {
    let new = match TryFrom::try_from(solution) {
        Ok(ba) => ba,
        Err(_) => panic!("Could not convert solution vec to [ ]")
    };
    grid.matrix[index] = new;

    // for (solution_index, value) in solution.iter().enumerate() {
    //     grid.matrix[index][solution_index] = *value;
    // }
}


pub fn evaluate_grid(new: &Vec<u8>, index: usize, grid: &Grid) -> usize {
    let length: usize = 8;
    let mut total_conflicts = 0;
    let matrix = grid.matrix;
    fitness_grid(new, index, &grid);

    // for (start, row) in matrix.iter().enumerate() {
    //     let solution = row.to_vec();
    //     for index in start..length {
    //         let next = index + 1;
    //         let neighbor = matrix[next];
    //         let collisions = fitness_score_grid(&solution, &neighbor);
    //         total_conflicts += collisions;
    //     }
    // }

    for (index, row) in grid.matrix.iter().enumerate() {


    }

    total_conflicts
}


pub fn check_completeness(grid: &Grid) -> usize {
    let mut total_conflicts = 0;
    let matrix = grid.matrix;

    for (row_index, row) in matrix.iter().enumerate() {
        let solution = row.to_vec();
        for next_row in matrix.iter().skip(row_index + 1) {
            let collisions = fitness_score_row(&solution, next_row);
            total_conflicts += collisions;
        }
    }

    let subgrid_conflicts = fitness_subgrids(grid);
    total_conflicts + subgrid_conflicts
}

pub fn explore(item_index: usize, grid: &Grid, cache: &Cache, temperature: f64, neighbourhood_size: u8) -> (usize, Vec<u8>) {
    // Select current point
    let start = grid.matrix[item_index].to_vec();
    let current_solution_score: usize = evaluate_solution(&start, item_index, &grid);
    let mut current_solution: (usize, Vec<u8>) = (current_solution_score, start.clone());
    // Generate Neighborhood from current point
    let neighborhood = generate_neighbourhood(start.clone(), item_index, neighbourhood_size, cache);
    let mut neighbor_solution_score: usize;

    for neighbor in neighborhood.iter() {
        // Evaluate current point
        neighbor_solution_score = evaluate_solution(&neighbor, item_index, &grid);
        current_solution = accept((neighbor_solution_score, neighbor.clone()), current_solution, temperature, item_index);
    }

    current_solution

}

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

pub fn calculate_temperature(matrix: [[u8;9];9], cache_grid: Grid) -> f64 {

    let mut assign_grid = Grid::new(matrix.clone());
    let assign_cache = Cache::new(&cache_grid.clone());
    const LENGTH: usize = 10;
    let mut scores = [0.0 as f64; LENGTH];

    for index in 0..10 {
        initial_assignment(&mut assign_grid, &assign_cache);
        let subgrid_score = fitness_subgrids(&assign_grid);
        let grid_score = check_completeness(&assign_grid);
        scores[index] = subgrid_score as f64 + grid_score as f64;
    }

    utils::compute_standard_deviation(&scores).unwrap()
}

pub fn anneal(mut grid: &mut Grid, cooling_ratio: f64, _total_attempts: u32, neighbourhood_size: u8) {
    /*TODO: 
    1. calculate amount of tries per fixed sudoku
    2. after the amount of tries test if temperature calculation is correct
    */

    // initialize temperature
    let mut init_grid: [[u8; 9]; 9] = [[0; 9]; 9];

    init_grid.copy_from_slice(&grid.matrix);
    let cache_grid = Grid::new(init_grid);
    log_headline("Initial Grid", grid);

    let cache = Cache::new(&cache_grid);
    let total_attempts: usize = cache.fixed_positions.iter().map(Vec::len).sum();
    initial_assignment(&mut grid, &cache);

    // calculate the std 
    let temperature = calculate_temperature(grid.matrix, cache_grid);
    // let mut assign_grid = Grid::new(grid.matrix.clone());
    // let assign_cache = Cache::new(&cache_grid);
    // const LENGTH: usize = 10;
    // let mut scores = [0.0 as f64; LENGTH];
    // for index in 0..10 {
    //     initial_assignment(&mut assign_grid, &assign_cache);
    //     let subgrid_score = fitness_subgrids(&assign_grid);
    //     let grid_score = check_completeness(&assign_grid);
    //     scores[index] = subgrid_score as f64 + grid_score as f64;
    // }
    // let mut temperature = utils::compute_standard_deviation(&scores).unwrap();

    log_headline("After initial assignment", grid);

    let conflicts = check_completeness(&grid);

    if conflicts == 0 {
        log_headline("Solved", grid);
        return;
    }

    let mut current_temperature = temperature;
    let rows: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6 ,7, 8];

    for _ in 0..total_attempts {

        let random_row = rows.choose(&mut rand::thread_rng()).unwrap();
        let index = *random_row as usize;
        let new = explore(index, grid, &cache, temperature, neighbourhood_size);
        assign_solution(new.1, index, grid);
        let conflicts = check_completeness(grid);
        log(conflicts);
        if conflicts == 0 {
            println!("{}", grid);
            return;
        }

        current_temperature = current_temperature * cooling_ratio;
    }

    log_headline("Guess", grid);
    println!("end temperature {}", current_temperature);
}


#[cfg(test)]
mod annealing_tests;