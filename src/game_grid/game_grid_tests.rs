#[cfg(test)]
mod tests {
    use crate::game_grid::Grid;

    #[test]
    fn it_works() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_check_row() {
        let mut grid = Grid {
            matrix: [[0; 9]; 9],
        };
        let input: u8 = 5;
        let valid_position: (u8, u8) = (0, 4);
        let invalid_position: (u8, u8) = (1, 4);

        let valid_row: [u8; 9] = [1, 2, 3, 4, 0, 6, 7, 8, 9];
        let invalid_row: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        grid.matrix[0] = valid_row;
        grid.matrix[1] = invalid_row;
        grid.matrix[3][3] = 7;

        let should_work = grid._check_row(&input, &valid_position);
        let should_fail = grid._check_row(&input, &invalid_position);

        assert_eq!(should_work, true);
        assert_eq!(should_fail, false);
    }

    #[test]
    fn test_check_col() {
        let mut grid = Grid {
            matrix: [[0; 9]; 9],
        };
        let input: u8 = 5;
        let valid_position: (u8, u8) = (4, 5);
        let invalid_position: (u8, u8) = (1, 4);

        let mut value: u8 = 1;

        for row in 0..grid.matrix.len() {
            grid.matrix[row][4] = value;
            grid.matrix[row][5] = value;
            value += 1;
        }

        grid.matrix[4][5] = 0;

        let should_fail = grid._check_col(input, &invalid_position);
        let should_work = grid._check_col(input, &valid_position);

        assert_eq!(should_work, true);
        assert_eq!(should_fail, false);
    }

    fn fill_subgrid(start: (usize, usize), grid: &mut Grid) {
        let col_offset: u8 = 3;
        for row_index in 0..=2 {
            let col_idx = start.1 + row_index;
            grid.matrix[start.0][col_idx] = (row_index + 1) as u8;
            grid.matrix[start.0 + 1][col_idx] = (row_index + 1) as u8 + col_offset;
            grid.matrix[start.0 + 2][col_idx] = (row_index + 1) as u8 + col_offset * 2;
        }
    }

    #[test]
    fn test_check_subgrid() {
        let input: u8 = 5;
        let mut grid = Grid {
            matrix: [[0; 9]; 9],
        };
        let first_subgrid: (usize, usize) = (0, 0);
        let fifth_subgrid: (usize, usize) = (3, 3);
        fill_subgrid(first_subgrid, &mut grid);
        fill_subgrid(fifth_subgrid, &mut grid);
        grid.matrix[4][4] = 0;

        let invalid_position: (u8, u8) = (1, 1);
        let (row_idx, col_idx) = grid._get_subgrid_start_index(&invalid_position);
        let should_fail = find_in_subgrid(&grid, row_idx, col_idx, input);

        let valid_position: (u8, u8) = (4, 4);
        let (row_idx, col_idx) = grid._get_subgrid_start_index(&valid_position);
        let should_work = find_in_subgrid(&grid, row_idx, col_idx, input);

        assert_eq!(should_work, true);
        assert_eq!(should_fail, false);
    }

    fn find_in_subgrid(grid: &Grid, row_idx: usize, col_idx: usize, input: u8) -> bool {
        let mut contained = false;
        let mut first_row: u8;
        let mut second_row: u8;
        let mut third_row: u8;

        for index in col_idx..col_idx + 3 {
            first_row = grid.matrix[row_idx][index];
            second_row = grid.matrix[row_idx + 1][index];
            third_row = grid.matrix[row_idx + 2][index];

            contained = contained || input == first_row;
            contained = contained || input == second_row;
            contained = contained || input == third_row;
        }
        !contained
    }
}
