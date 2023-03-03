use std::{fmt};

#[derive(Debug)]
pub struct Grid {
    pub matrix: [[u8; 9]; 9]
}

impl Grid {
    pub fn check_input_validity(&self, input: &u8, position: &(u8, u8)) -> bool {
        if input > &9 {
            println!("{}", "Invalid input");
            return false;
        }

        let row_idx = usize::from(position.0);
        let col_idx = usize::from(position.1);

        let value = self.matrix[row_idx][col_idx];
        match value {
            0 => true,
            _ => false,
        }
    }

    fn check_row(&self, input: &u8, position: &(u8, u8)) -> bool {
        let row_idx: usize = usize::from(position.0);
        let row = self.matrix[row_idx];
        return !row.contains(&input);
    }

    fn check_col(&self, input: u8, position: &(u8, u8)) -> bool {
        let col_idx: usize = usize::from(position.1);
        let mut contained = false;
        for row in self.matrix {
            contained = contained || input == row[col_idx];
        }
        !contained
    }

    pub fn subgrid_start_index(&self, position: &(u8, u8)) -> (usize, usize) {
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

    fn check_subgrid(&self, input: &u8, position: &(u8, u8)) -> bool {
        let (row_idx, col_idx) = self.subgrid_start_index(position);
        let mut contained = false;

        for index in row_idx..row_idx+3 {
            contained = contained || input == &self.matrix[index][col_idx];
        }
        contained
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let col_separator = format!("{}", "| ");
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