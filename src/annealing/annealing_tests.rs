#[cfg(test)]
mod tests {
    use std::vec;

    use crate::annealing::{gather_value_pool, generate_solution_fixed, self};
    use crate::annealing::{ 
        gather_fixed_indices, 
        gather_free_indices,
        fitness_score,
        fitness,
        evaluate_grid,
    };

    use crate::annealing::Cache;
    use crate::game_grid::Grid;


    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }


    fn setup_empty_example() -> Grid {
        let matrix:[[u8; 9]; 9] = [      // -------------------------
            [0, 8, 0, 5, 0, 0, 0, 0, 0], // | 0 8 0 | 5 0 0 | 0 0 0 |
            [1, 4, 2, 0, 0, 0, 0, 0, 0], // | 1 4 2 | 0 0 0 | 0 0 0 |
            [6, 0, 3, 0, 8, 0, 0, 1, 0], // | 6 0 3 | 0 8 0 | 0 1 0 |
                                         // -------------------------
            [0, 0, 4, 0, 2, 0, 0, 0, 8], // | 0 0 4 | 0 2 0 | 0 0 8 |
            [8, 0, 0, 7, 0, 9, 0, 0, 6], // | 8 0 0 | 7 0 9 | 0 0 6 |
            [3, 0, 0, 0, 6, 0, 0, 0, 0], // | 3 0 0 | 0 6 0 | 0 0 0 |
                                         // -------------------------
            [0, 1, 0, 0, 5, 0, 8, 0, 4], // | 0 1 0 | 0 5 0 | 8 0 4 |
            [0, 0, 0, 0, 0, 0, 1, 5, 2], // | 0 0 0 | 0 0 0 | 1 5 2 |
            [0, 0, 0, 0, 0, 2, 0, 6, 0], // | 0 0 0 | 0 0 2 | 0 6 0 |
                                         // -------------------------
        ];

        let grid = Grid { matrix: matrix };

        grid
    }


    fn setup_solved_example() -> Grid {
        let matrix:[[u8; 9]; 9] = [      // -------------------------
            [7, 8, 9, 5, 4, 1, 6, 2, 3], // | 7 8 9 | 5 4 1 | 6 2 3 |
            [1, 4, 2, 6, 9, 3, 7, 8, 5], // | 1 4 2 | 6 9 3 | 7 8 5 |
            [6, 5, 3, 2, 8, 7, 4, 1, 9], // | 6 5 3 | 2 8 7 | 4 1 9 |
                                         // -------------------------
            [9, 6, 4, 1, 2, 5, 3, 7, 8], // | 9 6 4 | 1 2 5 | 3 7 8 |
            [8, 2, 1, 7, 3, 9, 5, 4, 6], // | 8 2 1 | 7 3 9 | 5 4 6 |
            [3, 7, 5, 8, 6, 4, 2, 9, 1], // | 3 7 5 | 8 6 4 | 2 9 1 |
                                         // -------------------------
            [2, 1, 7, 9, 5, 6, 8, 3, 4], // | 2 1 7 | 9 5 6 | 8 3 4 |
            [4, 9, 6, 3, 7, 8, 1, 5, 2], // | 4 9 6 | 3 7 8 | 1 5 2 |
            [5, 3, 8, 4, 1, 2, 9, 6, 7], // | 5 3 8 | 4 1 2 | 9 6 7 |
                                         // -------------------------
        ];

        let grid = Grid { matrix: matrix };

        grid
    }


    fn setup_empty_grid() -> Grid {
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

    fn setup_grid() -> Grid {
        let matrix:[[u8; 9]; 9] = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];

        let grid = Grid{matrix};

        grid
    }

    fn setup_cache() -> (Cache, Grid) {
        let grid = setup_empty_grid();
        let cache = Cache::new(&grid);

        (cache, grid)
    }

    fn setup_empty_solution() -> Vec<u8> {
        vec![0, 0, 1, 0, 4, 0, 7, 0, 9]
    }

    fn setup_solution() -> Vec<u8> {
        vec![8,3,1,2,4,5,7,6,9]
    }

    #[test]
    fn test_fitness() {
        let grid = setup_grid();
        let sln = grid.matrix[0].to_vec();
        let neighborhood: Vec<Vec<u8>> = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; 9
        ];

        let collisions: u8 = 81;
        let ranking = fitness(&sln, &neighborhood);
        let sum = ranking.iter().fold(0, |acc, (collisions, _)| acc + *collisions as u8);

        assert_eq!(collisions, sum)
    }

    #[test]
    fn test_fitness_score() {
        let sln = setup_solution();
        let neighbour: Vec<u8> = vec![3,8,1,2,4,5,7,6,9];
        let collisions:usize = 7;
        let score = fitness_score(&sln, &neighbour);

        assert_eq!(score, collisions);

        let sln = setup_empty_solution();
        let collisions:usize = 4;
        let neighbour = sln.clone();
        let score = fitness_score(&sln, &neighbour);

        assert_eq!(score, collisions);
    }

    #[test]
    fn test_gather_fixed_indices() {
        let sln = setup_empty_solution();
        let fixed_indices = vec![2, 4, 6, 8];
        let fixed = gather_fixed_indices(&sln);

        assert_eq!(fixed, fixed_indices);
    }


    #[test]
    fn test_gather_free_indices() {
        let row_index: usize = 1;
        // let sln = setup_empty_solution();
        let (cache, _) = setup_cache();
        let free_indices = vec![0, 1, 3, 5, 7];
        let free = gather_free_indices(row_index, &cache);

        assert_eq!(free, free_indices);
    }


    #[test]
    fn test_gather_value_pool() {
        let mut sln = setup_empty_solution();
        let pool = gather_value_pool(&mut sln);
        let available_values = vec![2, 3, 5, 6, 8];

        assert_eq!(available_values, pool);
    }

    #[test]
    fn test_generate_solution_fixed(){
        let row_index: usize = 1;
        let sln = setup_empty_solution();
        let (cache, _) = setup_cache();
        let mut generated_solution: Vec<u8> = sln.to_vec();
        let fixed_from_sln = gather_fixed_indices(&sln);
        let mut equal = true;

        generate_solution_fixed(&mut generated_solution, row_index, &cache);

        for index in fixed_from_sln.iter() {
            let gen_value = generated_solution[*index];
            let sln_value = sln[*index];
            equal = gen_value == sln_value;
        }

        assert!(equal);
    }

    #[test]
    fn test_evaluate_grid() {

        let complete_solution: [[u8; 9]; 9] = [
            [7, 5, 6, 4, 3, 9, 8, 1, 2],
            [4, 9, 8, 1, 6, 2, 7, 5, 3],
            [1, 3, 2, 7, 8, 5, 6, 4, 9],
            [8, 1, 3, 5, 9, 4, 2, 6, 7],
            [6, 7, 5, 3, 2, 1, 9, 8, 4],
            [2, 4, 9, 8, 7, 6, 5, 3, 1],
            [3, 6, 1, 9, 5, 7, 4, 2, 8],
            [5, 8, 7, 2, 4, 3, 1, 9, 6],
            [9, 2, 4, 6, 1, 8, 3, 7, 5]
        ];

        let complete_grid = Grid { matrix: complete_solution };
        let conflicts_complete = evaluate_grid(&complete_grid);

        assert_eq!(conflicts_complete, 0);

        let faulty_solution: [[u8; 9]; 9] = [
            [7, 5, 6, 4, 3, 9, 8, 1, 2],
            [4, 9, 8, 1, 6, 2, 7, 5, 3],
            [8, 1, 3, 5, 9, 4, 2, 6, 7],
            [6, 7, 5, 3, 2, 1, 9, 8, 4],
            [2, 4, 9, 8, 7, 6, 5, 3, 1],
            [4, 9, 8, 1, 6, 2, 7, 5, 3],
            [3, 6, 1, 9, 5, 7, 4, 2, 8],
            [5, 8, 7, 2, 4, 3, 1, 9, 6],
            [9, 2, 4, 6, 1, 8, 3, 7, 5]
        ];

        let faulty_grid = Grid { matrix: faulty_solution };
        let conflicts_faulty = evaluate_grid(&faulty_grid);
        
        assert_eq!(conflicts_faulty, 9);
    }

}
