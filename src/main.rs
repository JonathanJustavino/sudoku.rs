mod annealing;
mod game_grid;
mod grid;
mod utils;

use std::path::PathBuf;

use clap::Parser;
use dotenv::dotenv;

use crate::grid::Grid;
//TODO: https://docs.rs/heapless/latest/heapless/struct.Vec.html use heapless vectors

#[derive(Parser, Debug)]
#[command(version, about="A sudoku solver using simmulated annealing", long_about = None)]
struct Cli {
    /// Name of the grid to solve (located under /static)
    #[arg(short, long, default_value = "easy.txt")]
    grid_name: String,

    /// value of cooling ratio (typically in the range [0.99,..,0.85])
    #[arg(short, long, default_value_t = 0.99)]
    cooling_ration: f32,

    /// List available grids
    #[arg(short, long)]
    list: bool,
}

fn main() {
    dotenv().ok();

    let args = Cli::parse();

    let grid_names = utils::list_dir().unwrap_or_default();
    if grid_names.is_empty() {
        panic!("Available grids are empty");
    }

    if args.list {
        println!("Available grids");
        for name in grid_names.iter() {
            let file_name = name.file_name().unwrap().to_str().unwrap();
            let (print_name, _) = file_name.split_at(file_name.len() - 4);
            println!("{}", print_name);
        }
        return;
    }

    let cooling_ratio = args.cooling_ration;
    let mut grid_name = PathBuf::from(args.grid_name);

    if cooling_ratio.is_nan() {
        panic!("Cooling ratio is not a number!");
    }

    grid_name.set_extension("txt");
    let found = grid_names
        .into_iter()
        .find(|name| *name.file_name().unwrap() == grid_name);

    println!("selected gridname {:?}", grid_name);

    if found.is_none() {
        panic!("Selected grid name is not available");
    }

    let mut grid = Grid::from_file(&grid_name.to_string_lossy());
    println!("{}", grid);
    annealing::anneal(&mut grid, cooling_ratio);
}
