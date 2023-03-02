mod game_grid;
use crate::game_grid::Grid;

fn main() {
    let grid = Grid{matrix: [[0;9]; 9]};
    println!("{}", grid);
    let is_valid = grid.check_input_validity(&5, &(7, 8));
    // println!("{}", is_valid);
}
