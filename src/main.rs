mod annealing;
mod game_grid;
mod grid;
mod solver;
mod utils;

// use crate::game_grid::Grid;
use crate::grid::Grid;

//TODO: https://docs.rs/heapless/latest/heapless/struct.Vec.html use heapless vectors

fn main() {
    let grid = Grid::from_file("solved.txt");
    println!("{}", grid);
}
