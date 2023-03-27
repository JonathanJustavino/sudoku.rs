mod game_grid;
use crate::game_grid::Grid;

fn main() {
    let grid = Grid{matrix: [[0;9]; 9]};
    println!("{}", grid);
}
