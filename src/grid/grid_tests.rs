#[cfg(test)]
mod tests {
    use crate::grid::{Grid};
    use crate::utils;
    use ndarray::{self, Array2, array};

    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }

    fn create_template() -> Array2<u8> {
       let data = array![
            [0, 8, 9, 5, 4, 1, 6, 2, 0],
            [0, 4, 2, 6, 9, 3, 7, 8, 0],
            [6, 5, 3, 2, 8, 7, 4, 1, 9],
            [9, 6, 4, 1, 2, 5, 3, 7, 8],
            [8, 2, 1, 7, 3, 9, 5, 4, 6],
            [3, 7, 5, 8, 6, 4, 2, 9, 1],
            [2, 1, 7, 9, 5, 6, 8, 3, 4],
            [4, 9, 6, 3, 7, 8, 1, 5, 2],
            [5, 3, 8, 4, 1, 2, 9, 6, 7],
        ];

        return data;
    }

    // #[test]
    fn test_insert_subgrid() {
        let matrix = Array2::<u8>::zeros((9,9));

        let mut grid = Grid::new(matrix);

        for i in 0..9 {
            let mut sub_grid = Array2::<u8>::ones((3, 3));
            sub_grid += i;
            grid.insert_subgrid(&sub_grid, i as usize);
        }

        println!("{}", grid);
    }

    #[test]
    fn test_collect_fixed() {
        let data = create_template();
        let grid = Grid::new(data.clone());
        let mut collected = grid.collect_fixed_indices(0);
        let mut fixed_values: Vec<usize> = vec![1, 2, 4, 5, 6, 7, 8];

        fixed_values.sort();
        collected.sort();

        assert_eq!(fixed_values, collected);

    }

    #[test]
    fn test_collect_free() {
        let data = create_template();
        let grid = Grid::new(data.clone());
        let mut collected = grid.collect_free_indices(0);
        let mut free_values: Vec<usize> = vec![0, 3];

        free_values.sort();
        collected.sort();

        assert_eq!(free_values, collected);
    }

}