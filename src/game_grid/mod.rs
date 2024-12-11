use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Grid {
    pub matrix: [[u8; 9]; 9],
}

impl Grid {
    pub fn _new(matrix: [[u8; 9]; 9]) -> Self {
        Self { matrix }
    }
}

impl Grid {
    pub fn _check_value(&self, input: &u8, position: (&u8, &u8)) -> bool {
        let row_idx = usize::from(*position.0);
        let col_idx = usize::from(*position.1);
        let valid = true;

        match (*input, col_idx, row_idx) {
            (0..=9, 0..=9, 0..=9) => true,
            _ => {
                println!("Invalid input");
                return !valid;
            }
        };

        if 0 < self.matrix[row_idx][col_idx] {
            return !valid;
        }

        if !self._check_row(input, position) {
            return !valid;
        }

        if !self._check_col(input, position) {
            return !valid;
        }

        if !self._check_subgrid(input, position) {
            return !valid;
        }

        valid
    }

    fn _check_row(&self, input: &u8, position: (&u8, &u8)) -> bool {
        let row_idx: usize = usize::from(*position.0);
        let row = self.matrix[row_idx];

        !row.contains(input)
    }

    fn _check_col(&self, input: &u8, position: (&u8, &u8)) -> bool {
        let col_idx: usize = usize::from(*position.1);
        let mut contained = false;
        for row in self.matrix {
            contained = contained || *input == row[col_idx];
        }
        !contained
    }

    pub fn _get_subgrid_start_index(&self, position: (&u8, &u8)) -> (usize, usize) {
        let row = position.0;
        let col = position.1;

        let start_row: u8 = match row {
            0..=2 => 0,
            3..=5 => 3,
            _ => 6,
        };

        let start_col: u8 = match col {
            0..=2 => 0,
            3..=5 => 3,
            _ => 6,
        };

        (usize::from(start_row), usize::from(start_col))
    }

    fn _check_subgrid(&self, input: &u8, position: (&u8, &u8)) -> bool {
        let (row_idx, col_idx) = self._get_subgrid_start_index(position);
        let mut contained = false;

        for index in col_idx..col_idx + 3 {
            let first_row = self.matrix[row_idx][index];
            let second_row = self.matrix[row_idx + 1][index];
            let third_row = self.matrix[row_idx + 2][index];

            contained = contained || *input == first_row;
            contained = contained || *input == second_row;
            contained = contained || *input == third_row;
        }
        !contained
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let col_separator = "| ".to_string();
        let row_separator = format!("{}{}", "-".repeat(25), "\n");
        let mut output = String::from("").to_owned();

        for (row_index, row) in self.matrix.iter().enumerate() {
            if row_index % 3 == 0 {
                output.push_str(&row_separator);
            }

            for (col_index, number) in row.iter().enumerate() {
                if col_index % 3 == 0 {
                    output.push_str(&col_separator);
                }
                output.push_str(&format!("{} ", &number.to_string()));
            }

            output.push_str("|\n");
        }

        output.push_str(&row_separator);
        let print = output.to_string();

        write!(formatter, "{}", print)
    }
}

#[cfg(test)]
mod game_grid_tests;
