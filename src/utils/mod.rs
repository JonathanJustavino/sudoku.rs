use std::collections::HashSet;
use std::hash::Hash;


pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn compute_mean(values: &[f64]) -> f64 {
    let sum: f64 = values.iter().sum();
    let count = values.len() as f64;
    sum / count
}

pub fn compute_standard_deviation(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

   let mean = compute_mean(values);

   let sum_of_squares: f64 = values
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum();

   let variance = sum_of_squares / (values.len() as f64);
   Some(variance.sqrt())
}

#[cfg(test)]
mod utils_tests;