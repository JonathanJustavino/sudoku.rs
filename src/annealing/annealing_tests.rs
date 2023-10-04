use std::collections::HashSet;
use std::hash::Hash;

#[cfg(test)]
mod tests {
    use crate::annealing::{generate_solution, annealing_tests::has_unique_elements};


    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_generate_solution() {
        let solution = generate_solution();
        assert_eq!(solution.len(), 9);
        assert!(has_unique_elements(solution));
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}