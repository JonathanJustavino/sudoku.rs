#[cfg(test)]
mod tests {
    use crate::grid::Grid;
    use ndarray::Array2;

    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_insert_subgrid() {
        let matrix: Array2<u8> = Array2::<u8>::zeros((9, 9));
        let mut grid = Grid::new(matrix);

        for i in 0..9 {
            let mut sub_grid: Array2<u8> = Array2::<u8>::ones((3, 3));
            sub_grid += i;
            grid.set_subgrid(&sub_grid, i as usize);
        }

        println!("{}", grid);
    }

    #[test]
    fn test_collect_fixed() {
        let grid = Grid::from_file("off_by_one.txt");
        let mut collected = Grid::collect_fixed_indices(&grid.matrix, 0);
        let mut fixed_values: Vec<usize> = vec![1, 2, 4, 5, 6, 7, 8];

        fixed_values.sort();
        collected.sort();

        assert_eq!(fixed_values, collected);
    }

    #[test]
    fn test_collect_free() {
        let grid = Grid::from_file("off_by_one.txt");
        let mut collected = grid.collect_free_indices(0);
        let mut free_values: Vec<usize> = vec![0, 3];

        free_values.sort();
        collected.sort();

        assert_eq!(free_values, collected);
    }
}
