use std::collections::HashSet;
use std::hash::Hash;

#[cfg(test)]
mod tests {
    use crate::annealing::{generate_solution, annealing_tests::has_unique_elements, generate_neighbourhood, swap};

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

    #[test]
    fn test_swap() {
        let mut solution = generate_solution();
        let check = solution.to_vec();
        let (first, second) = swap(&mut solution);

        assert_eq!(solution[first], check[second]);
        assert_eq!(solution[second], check[first]);
    }

    #[test]
    fn test_generate_neighbourhood() {
        let solution = generate_solution();
        let amount = 9;
        let neighbours = generate_neighbourhood(solution, amount);
        assert_eq!(neighbours.len(), amount as usize);

        println!("{:?}", neighbours);

        assert!(true);
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