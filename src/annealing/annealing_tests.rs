#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use ndarray::Array2;
    use std::collections::BTreeSet;

    // use crate::annealing::{
    //     assign_solution, check_completeness, evaluate_solution, fitness_grid, fitness_score_row, fitness_subgrid, fitness_subgrids, gather_fixed_indices, gather_free_indices, gather_value_pool, generate_neighbourhood, generate_solution_fixed, initial_assignment, swap_values
    // };

    use crate::{annealing, grid::Grid};

    #[test]
    fn test_gather_fixed_indices() {
        let index = 4;
        let grid = Grid::from_file("empty_example.txt");
        println!("{}", grid.get_subgrid(index));
        let collected = Grid::collect_fixed_indices(&grid.matrix, index);
        let ground_truth = vec![1, 3, 5, 7];

        assert_eq!(collected, ground_truth);
    }

    #[test]
    fn test_gather_free_indices() {
        dotenv().ok();
        let index: usize = 1;
        let grid = Grid::from_file("empty_example.txt");
        let ground_truth = vec![1, 2, 3, 4, 5, 6, 8];
        let collected = grid.collect_free_indices(index);

        assert_eq!(collected, ground_truth);
    }

    #[test]
    fn test_gather_value_pool() {
        let index: usize = 7;
        let grid = Grid::from_file("empty_example.txt");
        let ground_truth: Vec<u8> = vec![1, 3, 4, 6, 7, 8, 9];
        let pool = grid.determine_value_pool(index);

        println!("{}", grid);
        assert_eq!(pool, ground_truth);
    }

    #[test]
    fn test_map_to_grid() {
        let coordinates: Vec<usize> = (0..9).collect();
        let grid_coords: [(usize, usize); 9] = [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        for index in coordinates {
            let mapping = Grid::map_to_grid(index);
            assert_eq!(grid_coords[index], mapping);
        }
    }

    #[test]
    fn test_generate_solution_fixed() {
        let mut equal = true;
        let subgrid_index: usize = 1;
        let mut grid = Grid::from_file("empty_example.txt");
        grid.initialize();

        let current = grid.get_subgrid(subgrid_index).to_owned();
        let fixed = &grid.fixed_subgrid_positions[subgrid_index];

        let sln: Array2<u8> = annealing::generate_solution(&current, fixed);

        let fixed_subgrid = &grid.fixed_subgrid_positions[subgrid_index];
        let ground_truth = grid.get_subgrid(subgrid_index);
        for index in fixed_subgrid.iter() {
            let (x, y) = Grid::map_to_grid(*index);
            equal &= sln[[x, y]] == ground_truth[[x, y]];
        }

        assert!(equal);

        let mut sln_values = sln.clone().into_raw_vec();
        let legal_values: Vec<u8> = (0..10).collect();
        sln_values.retain(|value| legal_values.contains(value));

        assert_eq!(sln_values.len(), 9);
    }

    #[test]
    fn test_assign_solution() {
        let index = 2;
        let mut grid = Grid::from_file("empty_example.txt");
        let sln: Array2<u8> =
            Array2::from_shape_vec((3, 3), vec![2, 3, 4, 5, 6, 7, 8, 1, 9]).unwrap();
        let ground_truth = sln.clone();

        annealing::_assign_solution(sln, index, &mut grid);

        let subgrid = grid.get_subgrid(index);

        assert_eq!(ground_truth, subgrid);
        //FIXME: fixed positions are not preserved!
        println!("{}", grid);
    }

    #[test]
    fn test_check_completeness() {
        // let complete_solution: [[u8; 9]; 9] = [
        //     [7, 5, 6, 4, 3, 9, 8, 1, 2],
        //     [4, 9, 8, 1, 6, 2, 7, 5, 3],
        //     [1, 3, 2, 7, 8, 5, 6, 4, 9],
        //     [8, 1, 3, 5, 9, 4, 2, 6, 7],
        //     [6, 7, 5, 3, 2, 1, 9, 8, 4],
        //     [2, 4, 9, 8, 7, 6, 5, 3, 1],
        //     [3, 6, 1, 9, 5, 7, 4, 2, 8],
        //     [5, 8, 7, 2, 4, 3, 1, 9, 6],
        //     [9, 2, 4, 6, 1, 8, 3, 7, 5]
        // ];

        let complete_grid = Grid::from_file("solved.txt");
        let collisions = annealing::check_completeness(&complete_grid.matrix);

        assert_eq!(collisions, 0);

        // let faulty_solution: [[u8; 9]; 9] = [
        //     [7, 5, 6, 4, 3, 9, 8, 1, 2],
        //     [4, 9, 8, 1, 6, 2, 7, 5, 3],
        //     [8, 1, 3, 5, 9, 4, 2, 6, 7],
        //     [6, 7, 5, 3, 2, 1, 9, 8, 4],
        //     [2, 4, 9, 8, 7, 6, 5, 3, 1],
        //     [4, 9, 8, 1, 6, 2, 7, 5, 3],
        //     [3, 6, 1, 9, 5, 7, 4, 2, 8],
        //     [5, 8, 7, 2, 4, 3, 1, 9, 6],
        //     [9, 2, 4, 6, 1, 8, 3, 7, 5]
        // ];

        // let faulty_grid = Grid { matrix: faulty_solution };
        // let faulty_grid = Grid::from_file("faulty.txt");

        // let mut conflicts_faulty = annealing::compute_col_collisions(&faulty_grid);
        // conflicts_faulty += annealing::compute_row_collisions(&faulty_grid);

        // assert_eq!(conflicts_faulty, 21);

        // let grid_solved = setup_solved_example();
        // let conflicts_solved = check_completeness(&grid_solved);

        // assert_eq!(conflicts_solved, 0);
    }

    //     #[test]
    //     fn test_evaluate_solution() {
    //         let complete_grid = setup_solved_example();
    //         let index: usize = 0;
    //         let solution = complete_grid.matrix[index].to_vec();
    //         let score = evaluate_solution(&solution, index, &complete_grid);

    //         assert_eq!(score, 0);

    //         let faulty_grid = setup_grid();
    //         let index: usize = 2;
    //         let solution = faulty_grid.matrix[index].to_vec();
    //         let score = evaluate_solution(&solution, index, &faulty_grid);

    //         assert_eq!(score, 126);
    //     }

    //     #[test]
    //     fn test_assign_solution() {
    //         let index: usize = 0;
    //         let solution: Vec<u8> = vec![1; 9];
    //         let mut grid = setup_solved_example();
    //         let row = grid.matrix[index];

    //         let check: [u8; 9] = [1,1,1,1,1,1,1,1,1];
    //         assert_ne!(check, row);

    //         assign_solution(solution, index, &mut grid);

    //         let row = grid.matrix[index];
    //         assert_eq!(check, row);
    //     }

    #[test]
    fn test_initial_assignment() {
        let mut grid: Grid = Grid::from_file("empty_example.txt");
        grid.initialize();
        let mut contains_zeros = false;

        for row in grid.matrix.rows() {
            let check = BTreeSet::from_iter(row);
            contains_zeros = check.contains(&0);
        }

        assert!(!contains_zeros);
    }

    #[test]
    fn test_generate_neighborhood() {
        let mut grid: Grid = Grid::from_file("empty_example.txt");
        grid.initialize();
        let subgrid_index: usize = 1;
        let start = grid.get_subgrid(subgrid_index).to_owned();
        let fixed = &grid.fixed_subgrid_positions[subgrid_index];

        let neighborhood = annealing::generate_neighbourhood(start.clone(), fixed);
        let fixed_posititions = &grid.fixed_subgrid_positions[subgrid_index];

        for sub_grid in neighborhood {
            assert_ne!(start, sub_grid);
            for position in fixed_posititions {
                let (x, y) = Grid::map_to_grid(*position);
                assert_eq!(start[[x, y]], sub_grid[[x, y]]);
            }
        }
    }

    //     #[test]
    //     fn test_complete() {

    //         let grid = setup_solved_example();
    //         let subgrid_scores = fitness_subgrids(&grid);
    //         let score = check_completeness(&grid);

    //         assert_eq!(0, score + subgrid_scores);

    //         let off_by_one = setup_off_by_one();
    //         let off_by_one_subgrid_scores = fitness_subgrids(&off_by_one);
    //         let off_by_one_score = check_completeness(&off_by_one);

    //         assert_eq!(2, off_by_one_score + off_by_one_subgrid_scores);
    //     }
}
