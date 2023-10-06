use std::vec;
use rand::{Rng, seq::SliceRandom};

#[cfg(test)]
mod annealing_tests;

pub fn generate_solution() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut sampled = rand::seq::index::sample(&mut rng, 9, 9).into_vec();
    sampled.iter_mut().for_each(|x| *x += 1);
    let result = sampled.iter().map(|&e| e as u8).collect();

    result
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
