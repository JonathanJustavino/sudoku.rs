use rand::prelude::*;
use rand::thread_rng;

use crate::game_grid::Grid;

#[derive(Debug)]
pub struct Solver {
    pub cache: Vec<Vec<(u8, Vec<u8>)>>,
    pub grid: Grid,
}

impl Solver {
    pub fn new(grid: Grid) -> Solver {
        Solver {
            grid: grid,
            cache: vec![vec![(0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]); 9]; 9],
        }
    }

    fn validate_guess(self, position: (&u8, &u8), choice: u8) {
        let value = self.grid.matrix[*position.0 as usize][*position.1 as usize];
        // self.guess(x, y, );
    }

    fn guess(&mut self, x: &u8, y: &u8, values: &mut[u8; 9]) -> u8 {
        let mut rng = thread_rng();
        let choice = values.choose(&mut rng).unwrap();
        self.cache_guess(*x, *y, *choice);
        return *choice;
    }

    fn cache_guess(&mut self, x: u8, y: u8, value: u8) {
        // let cell_cache: (u8, Vec<u8>) = self.cache[x as usize][y as usize];
        // self.cache.push((x, y, value))
    }
}


use std::fmt;

impl fmt::Display for Solver {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let col_separator = format!("{}", "| ");
        let row_separator = format!("{}{}", "-".repeat(25), "\n");
        let mut output = String::from("").to_owned();

        for (row_index, row) in self.grid.matrix.iter().enumerate() {
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
mod solver_tests;