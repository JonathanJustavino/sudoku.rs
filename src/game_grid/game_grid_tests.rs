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
        let mut grid = Grid {matrix: [[0; 9]; 9]};
        let input: u8 = 5;
        let valid_position: (u8, u8) = (0, 4);
        let invalid_position: (u8, u8) = (1, 4);

        let valid_row: [u8; 9] = [1, 2, 3, 4, 0, 6, 7, 8, 9];
        let invalid_row: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        grid.matrix[0] = valid_row;
        grid.matrix[1] = invalid_row;
        grid.matrix[3][3] = 7;

        let check_valid_row = grid.check_row(&input, &valid_position);
        let check_invalid_row = grid.check_row(&input, &invalid_position);

        assert_eq!(check_valid_row, true);
        assert_eq!(check_invalid_row, false);
    }

    #[test]
    fn test_check_col() {
        let mut grid = Grid {matrix: [[0; 9]; 9]};
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

        let check_invalid_col = grid.check_col(input, &invalid_position);
        let check_valid_col = grid.check_col(input, &valid_position);

        assert_eq!(check_valid_col, true);
        assert_eq!(check_invalid_col, false);
    }
}