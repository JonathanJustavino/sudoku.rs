mod game_grid;
mod solver;
mod annealing;

use crate::game_grid::Grid;
use crate::solver::Solver;
// use crate::annealing;

fn main() {
    // let grid = Grid{matrix: [[0;9]; 9]};
    // let solver: Solver = Solver::new(grid);

    // println!("{}", solver.grid);
    // println!("{:?}", solver);

    annealing::generate_solution();


}
