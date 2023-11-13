#[cfg(test)]
mod tests {
    use std::vec;

    use crate::annealing::{gather_value_pool, generate_initial_solution_fixed};
    use crate::annealing::{utils::has_unique_elements, 
        conflicts_per_row, 
        amount_of_conflicts, 
        gather_fixed_indices, 
        gather_free_indices
    };

    use crate::annealing::Cache;
    use crate::game_grid::Grid;


    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }

    fn setup_grid() -> Grid {
        let matrix:[[u8; 9]; 9] = [
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
            [0, 0, 1, 0, 4, 0, 7, 0, 9],
        ];

        let grid = Grid{matrix};

        grid
    }

    fn setup_cache() -> (Cache, Grid) {
        let grid = setup_grid();
        let cache = Cache::new(&grid);

        (cache, grid)
    }

    fn setup_solution() -> Vec<u8> {
        vec![0, 0, 1, 0, 4, 0, 7, 0, 9]
    }

    #[test]
    fn test_amount_of_conflicts() {
        let grid = setup_grid();
        let solution = grid.matrix[0].to_vec();
        let conflicts = amount_of_conflicts(&solution, 0 as usize, &grid);

        assert_eq!(conflicts, 72);
    } 

    #[test]
    fn test_conflicts_per_row() {
        let grid = setup_grid();
        let conflicts = conflicts_per_row(&grid.matrix[0].to_vec(), &grid.matrix[0].to_vec());

        assert_eq!(conflicts, 9);
    }

    #[test]
    fn test_gather_fixed_indices() {
        let sln = setup_solution();
        let fixed_indices = vec![2, 4, 6, 8];
        let fixed = gather_fixed_indices(&sln);

        assert_eq!(fixed, fixed_indices);
    }


    #[test]
    fn test_gather_free_indices() {
        let row_index: usize = 1;
        // let sln = setup_solution();
        let (cache, _) = setup_cache();
        let free_indices = vec![0, 1, 3, 5, 7];
        let free = gather_free_indices(row_index, &cache);

        assert_eq!(free, free_indices);
    }


    #[test]
    fn test_gather_value_pool() {
        let mut sln = setup_solution();
        let pool = gather_value_pool(&mut sln);
        let available_values = vec![2, 3, 5, 6, 8];

        assert_eq!(available_values, pool);
    }

    #[test]
    fn test_generate_initial_solution_fixed(){
        let row_index: usize = 1;
        let sln = setup_solution();
        let (cache, _) = setup_cache();
        let mut generated_solution: Vec<u8> = sln.to_vec();
        let fixed_from_sln = gather_fixed_indices(&sln);
        let mut equal = true;

        generate_initial_solution_fixed(&mut generated_solution, row_index, &cache);

        for index in fixed_from_sln.iter() {
            let gen_value = generated_solution[*index];
            let sln_value = sln[*index];
            equal = gen_value == sln_value;
        }

        assert!(equal);
    }
}



