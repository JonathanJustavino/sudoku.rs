use rand::{distributions::Uniform, Rng};

#[cfg(test)]
mod annealing_tests;

pub fn generate_solution() -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut results = rand::seq::index::sample(&mut rng, 9, 9).into_vec();
    results.iter_mut().for_each(|x| *x += 1);

    return results;
}