use std::vec;
use rand::Rng;

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
