use std::fmt;

#[derive(Debug)]
struct Grid {
    grid: [[u8; 9]; 9]
}

impl fmt::Display for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let col_separator = format!("{}", "| ");
        let row_separator = format!("{}{}", "-".repeat(25), "\n");

        let mut output = String::from("").to_owned();
        let mut triple_row_counter = 0;

        for row in self.grid.iter() {
            let mut row_buffer = String::from("").to_owned();

            if triple_row_counter % 3 == 0 {
                row_buffer.push_str(&row_separator);
            }

            for index in 0..row.len() {
                let column_triplet = index % 3 == 0;
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

fn main() {
    let grid = Grid{grid: [[0;9]; 9]};
    println!("{}", grid);
}
