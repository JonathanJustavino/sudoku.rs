use itertools::Itertools;
use ndarray::{s, Array1, Array2, Zip};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::grid::Grid;
use crate::utils;

pub fn generate_solution(current: &Array2<u8>, fixed_positions: &[usize]) -> Array2<u8> {
    let mut rng = thread_rng();
    let mut candidates: Vec<usize> = (0..9).collect();

    candidates.retain(|index| !fixed_positions.contains(index));

    let swap: (&usize, &usize) = candidates
        .choose_multiple(&mut rng, 2)
        .collect_tuple()
        .unwrap();
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

pub fn generate_neighbourhood(base: Array2<u8>, fixed_positions: &[usize]) -> Vec<Array2<u8>> {
    let mut neighbourhood: Vec<Array2<u8>> = vec![];
    let amount = 9;

    for _ in 0..amount {
        let neighbor = generate_solution(&base, fixed_positions);
        neighbourhood.push(neighbor);
    }

    neighbourhood
}

pub fn _assign_solution(solution: Array2<u8>, index: usize, grid: &mut Grid) {
    //TODO: Maybe dicrectly assign with index
    for (index_chunk, mut chunk) in grid.matrix.exact_chunks_mut((3, 3)).into_iter().enumerate() {
        if index == index_chunk {
            chunk.assign(&solution);
            break;
        }
    }
}

// pub fn crc(grid: &Grid) -> usize {
//     let mut row_collisions = 0;
//     for index in 0..9 {
//         row_collisions += grid.check_row(index);
//     }

//     row_collisions
// }

// pub fn ccc(grid: &Grid) -> usize {
//     let mut col_collisions = 0;
//     for index in 0..9 {
//         col_collisions += grid.check_col(index);
//     }

//     col_collisions
// }

pub fn compute_col_collisions(matrix: &Array2<u8>) -> i32 {
    let mut col_collisions = 0;
    let col_dim = 9;
    let row_dim = 9;

    for col_index in 0..col_dim {
        let row = matrix.column(col_index);
        if col_index >= 8 {
            break;
        }
        for compare_index in col_index + 1..col_dim {
            let next_row = matrix.column(compare_index);
            for row_index in 0..row_dim {
                let left = row[row_index];
                let right = next_row[row_index];
                if left == right {
                    col_collisions += 1;
                }
            }
        }
    }

    col_collisions
}

pub fn compute_row_collisions(matrix: &Array2<u8>) -> i32 {
    let mut row_collisions = 0;
    let col_dim = 9;
    let row_dim = 9;

    for row_index in 0..row_dim {
        let row = matrix.row(row_index);
        if row_index >= 8 {
            break;
        }
        for compare_index in row_index + 1..row_dim {
            let next_row = matrix.row(compare_index);
            for col_index in 0..col_dim {
                let left = row[col_index];
                let right = next_row[col_index];
                if left == right {
                    row_collisions += 1;
                }
            }
        }
    }

    row_collisions
}

pub fn check_completeness(matrix: &Array2<u8>) -> i32 {
    let mut collisions = 0;
    collisions += compute_col_collisions(matrix);

    collisions + compute_row_collisions(matrix)
}

pub fn _buggy_check_completeness(grid: &Grid) {
    let mut row_collisions = 0;

    for (row_index, row) in grid.matrix.rows().into_iter().enumerate() {
        if row_index >= 8 {
            break;
        }

        let subview = grid.matrix.slice(s![row_index + 1..9, ..]);
        for compare_row in subview.rows() {
            println!("Comparing {} {}", row, compare_row);
            let res: Array1<bool> = Zip::from(&row)
                .and(&compare_row)
                .map_collect(|&x, &y| x == y);

            let collisions = res.iter().filter(|comp_eq| **comp_eq).count();

            row_collisions += collisions;
        }

        println!("{:?}", row_collisions);
    }
}

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

pub fn _log(conflicts: usize) {
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

pub fn log_headline(headline: &str, grid: &Grid, conflicts: i32, temperature: f32) {
    let white_space = " ";
    let headline_len = headline.len() / 2;
    let grid_log_half = 12;

    println!(
        "{}{}\n{}\nConflicts => {} | Temperature => {}",
        white_space.repeat(grid_log_half - headline_len),
        headline,
        grid,
        conflicts,
        temperature
    );
}

pub fn calculate_temperature(grid: &Grid) -> f32 {
    const LENGTH: usize = 200;
    let mut scores = [0.0; LENGTH];

    for item in scores.iter_mut().take(LENGTH) {
        let mut test_init = grid.clone();
        test_init.initialize();
        *item = check_completeness(&test_init.matrix) as f32;
    }

    utils::compute_standard_deviation(&scores).unwrap()
}

pub fn estimate_attempts(fixed_positions: &[Vec<usize>]) -> f32 {
    /*
        Calculate the total number of iterations for the simulated annealing algorithm.
        The total number of iterations is equal to the square of the number of mutable
        cells on the board.
        args:
            board(Board): the board to calculate the total number of iterations for
        returns:
            (int) the total number of iterations to run for each temperature
    */

    let total_cells = 81.0;
    let mut fixed_values = 0;
    for row in fixed_positions.iter() {
        fixed_values += row.len();
    }

    (total_cells - fixed_values as f32).powi(2)
}

//TODO: use with lifetime annoations
pub fn accept(
    current: &Grid,
    proposed: &Grid,
    current_temperature: f32,
    _debug_index: usize,
) -> (Grid, i32) {
    let current_score = check_completeness(&current.matrix);

    let new_score = check_completeness(&proposed.matrix);

    // if new_score < current_score {
    //     return proposed.to_owned();
    // }

    let delta = new_score as f32 - current_score as f32;
    let returned_score = delta as i32;

    if delta < 0.0 {
        return (proposed.to_owned(), returned_score);
    }

    // 1 / (1 + e^( eval(v_current) - eval(v_n) ) / T)
    let criteria = -(1.0 / (1.0 + libm::expf(delta / current_temperature)));

    if criteria > 0.5 {
        return (proposed.to_owned(), returned_score);
    }

    // println!("{}", debug_index);

    (current.to_owned(), 0)
}

pub fn explore_new_state(
    subgrid_index: usize,
    grid: &mut Grid,
    neighbors: &[Array2<u8>],
    current_temperature: f32,
    debug_index: usize,
) -> i32 {
    let mut rng = thread_rng();
    let neighbour: &Array2<u8> = neighbors.choose(&mut rng).unwrap();
    let mut proposed = grid.clone();
    proposed.set_subgrid(neighbour, subgrid_index);
    let (selected, collision_diff) = accept(grid, &proposed, current_temperature, debug_index);

    if grid.matrix == selected.matrix {
        return 0;
    }

    grid.matrix = selected.matrix;

    collision_diff
}

pub fn anneal(initial_grid: &mut Grid, cooling_ratio: f32) {
    loop {
        let threshold = 100;
        let mut grid = initial_grid.clone();
        grid.initialize();
        let mut conflicts = check_completeness(&grid.matrix);

        if conflicts == 0 {
            log_headline("Solved", &grid, conflicts, 0.0);
            return;
        }

        let fixed = &grid.fixed_subgrid_positions;
        let total_attempts = estimate_attempts(fixed) as i32;

        let mut current_temperature = calculate_temperature(initial_grid);
        let mut rng = rand::thread_rng();
        let mut stuck_count = 0;
        let previous_conflicts = conflicts;

        for debug_index in 0..total_attempts {
            let index = rng.gen_range(0..9);
            let sln = grid.get_subgrid(index).to_owned();
            let neighbourhood = generate_neighbourhood(sln, &grid.fixed_subgrid_positions[index]);

            // log_headline("Before neighbor", &grid);

            let conflicts_diff = explore_new_state(
                index,
                &mut grid,
                &neighbourhood,
                current_temperature,
                debug_index as usize,
            );

            conflicts += conflicts_diff;
            current_temperature *= cooling_ratio;

            if conflicts == 0 {
                log_headline("Solved", &grid, conflicts, current_temperature);
                return;
            }

            if conflicts >= previous_conflicts {
                stuck_count += 1;
            }

            if stuck_count % 25 == 0 {
                print!(
                    "\rS:{} P:{} C:{} T:{}",
                    stuck_count, previous_conflicts, conflicts, current_temperature
                );
            }

            if stuck_count > threshold {
                break;
            }
        }
    }

    // log_headline("Guess", grid);
    // println!("end temperature {}", current_temperature);
}

#[cfg(test)]
mod annealing_tests;
