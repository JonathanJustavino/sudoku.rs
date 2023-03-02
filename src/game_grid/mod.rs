use std::fmt;

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
        return row.contains(&input);
    }

    fn check_col(&self, input: u8, position: &(u8, u8)) -> bool {
        let col_idx: usize = usize::from(position.1);
        let mut contained = false;
        for row in self.matrix {
            contained = contained || input == row[col_idx];
        }
        contained
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
        let mut triple_row_counter: u8 = 0;

        for row in self.matrix.iter() {
            let mut row_buffer = String::from("").to_owned();

            if triple_row_counter % 3 == 0 {
                row_buffer.push_str(&row_separator);
            }

            for index in 0..row.len() {
                let column_triplet: bool = index % 3 == 0;
                if column_triplet {
                    row_buffer.push_str(&col_separator);
                }

                let number = format!("{} ", row[index]);
                row_buffer.push_str(&number);
            }

            row_buffer.push_str("|\n");
            output.push_str(&row_buffer);
            triple_row_counter += 1 % 3;
        }

        output.push_str(&row_separator);
        let print = output.to_string();
        write!(formatter, "{}", print)
    }
}