use std::{fmt, iter::zip, path::Path};
use crate::{annealing, utils};
use itertools::Itertools;
use ndarray::{self, s, Array1, Array2, ArrayBase, ArrayView2, Dim, Ix, SliceInfo, SliceInfoElem, ViewRepr};
use rand::{seq::SliceRandom, thread_rng};


#[derive(Clone)]
pub struct Grid {
    pub matrix: ndarray::Array2<u8>,
    pub fixed_subgrid_positions: Vec<Vec<usize>>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let col_separator = format!("{}", "| ");
        let row_separator = format!("{}{}", "-".repeat(25), "\n");
        let mut output = String::from("").to_owned();

        for (row_index, row) in self.matrix.rows().into_iter().enumerate() {
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
        output.push_str("\n");

        let mut cache_output = String::from("Fixed Positions:\n").to_owned();
        for (index, row) in self.fixed_subgrid_positions.iter().enumerate() {
            let subgrid = format!("  Subgrid-{index}: {:?}\n", row);
            cache_output.push_str(&subgrid);
        }

        output.push_str(&cache_output);
        let print = output.to_string();

        write!(formatter, "{}", print)
    }
}


impl fmt::Display for Grid {

    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let col_separator = format!("{}", "| ");
        let row_separator = format!("{}{}", "-".repeat(25), "\n");
        let mut output = String::from("").to_owned();

        for (row_index, row) in self.matrix.rows().into_iter().enumerate() {
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

impl Grid {
    pub fn new(matrix: ndarray::Array2<u8>) -> Self {
        return Self {matrix: matrix, fixed_subgrid_positions: vec![vec![]; 9]};
    }

    pub fn from_file(file_name: &str) -> Self {
        let base_path_string = std::env::var("TEMPLATE_DIR").expect("Could not load TEMPLATE_DIR!");
        let template_path = Path::new(&base_path_string);
        let file_path = template_path.join(template_path).join(file_name);
        let matrix = utils::cast_to_array(&file_path);
        let mut fixed_positions: Vec<Vec<usize>> = vec![vec![]; 9];

        for index in 0..9 {
            fixed_positions[index] = Grid::collect_fixed_indices(&matrix, index);
        }

        return Self {matrix: utils::cast_to_array(&file_path), fixed_subgrid_positions: fixed_positions};
    }
}

impl Grid {
    pub fn determine_value_pool(&self, subgrid_index: usize) -> Vec<u8> {
        let (row, col) = self::Grid::get_indices(subgrid_index);
        let grid_slice = s![row..row + 3, col..col + 3];
        // let slice = self.matrix.slice(grid_slice).to_owned().into_shape(9).unwrap();
        let slice = self.matrix.slice(grid_slice).to_owned();
        let mut pool: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        pool.retain(|item| !slice.to_owned().into_iter().contains(item));
        return pool;
    }

    pub fn collect_fixed_indices(matrix: &Array2<u8>, subgrid_index: usize) -> Vec<usize> {
        let (row, col) = self::Grid::get_indices(subgrid_index);
        let subgrid_slice = s![row..row + 3, col..col + 3];

        let filter_non_zero = |(index, value): (usize, &u8)| -> Option<usize> {
            if *value != 0 {
                return Some(index)
            } else {
                return None
            };
        };

        let indices = self::Grid::_filter_collect(matrix, subgrid_slice, filter_non_zero);
        return  indices.clone();
    }

    pub fn collect_free_indices(&self, subgrid_index: usize) -> Vec<usize> {
        let (row, col) = self::Grid::get_indices(subgrid_index);
        let subgrid_slice = s![row..row + 3, col..col + 3];

        let filter_empty = |(index, value): (usize, &u8)| -> Option<usize> {
            if *value == 0 {
                return Some(index)
            } else {
                return None
            };
        };

        let indices = self::Grid::_filter_collect(&self.matrix, subgrid_slice, filter_empty);
        return indices.clone();
    }

    fn _filter_collect<F>(matrix: &Array2<u8>, grid_slice: SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 2]>>, func: F) -> Vec<usize>
    where
        F: Fn((usize, &u8)) -> Option<usize>,
    {
        matrix
            .slice(grid_slice)
            .to_owned() // Ensure contiguity
            // .into_shape((9,)) // Flatten into 1D
            // .unwrap()
            .iter()
            .enumerate()
            .filter_map(func) // Collect non-zero indices
            .collect()
    }
}


impl Grid {

    pub fn initialize(&mut self) {
        for index in 0..9 {
            self._initialize_subgrid(index);
        }
    }

    //TODO: Reimplement with the function `exact_chunks`
    pub fn get_subgrid(&self, index: usize) -> ArrayView2<u8> {
        let (row, col) = self::Grid::get_indices(index);
        let subgrid_slice = s![row..row + 3, col..col + 3];

        self.matrix.slice(subgrid_slice)
    }

    //TODO: Reimplement with the function `exact_chunks`
    pub fn get_subgrid_mut(&mut self, index: usize) -> ArrayBase<ViewRepr<&mut u8>, Dim<[Ix; 2]>> {
        let (row, col) = self::Grid::get_indices(index);
        let subgrid_slice = s![row..row + 3, col..col + 3];

        self.matrix.slice_mut(subgrid_slice)
    }

    fn _initialize_subgrid(&mut self, subgrid_index: usize) {
        let (row, col) = self::Grid::get_indices(subgrid_index);
        let subgrid_slice = s![row..row + 3, col..col + 3];
        let free_positions = self.collect_free_indices(subgrid_index);
        let mut pool = self.determine_value_pool(subgrid_index);

        let mut grid_slice = self.matrix.slice_mut(subgrid_slice);
        let mut rng = thread_rng();
        let max_first_row_idx: usize = 2;
        let max_second_row_idx: usize = 5;

        let map_to = | pos | -> (usize, usize) {
            if pos > max_second_row_idx {
                return (2, pos % 3)
            }
            if pos > max_first_row_idx {
                return (1, pos % 3)
            }
            return (0, pos % 3)
        };

        pool.shuffle(&mut rng);

        for (pos, value) in zip(free_positions, pool) {
            let (x, y) = map_to(pos);
            grid_slice[[x, y]] = value;
        }

        let sub: Array2<u8> = grid_slice.into_owned();

        self.set_subgrid(&sub, subgrid_index);
    }

    pub fn set_subgrid(&mut self, subgrid: &Array2::<u8>, index: usize) {
        let valid_dim: (usize, usize) = (3, 3);
        assert_eq!(subgrid.dim(), valid_dim, "Wrong dimensions of subgrid");

        let (row_start, col_start) = self::Grid::get_indices(index);
        self.matrix.slice_mut(s![row_start..row_start + 3, col_start..col_start + 3]).assign(subgrid);
    }

    pub fn map_to_subgrid(index: usize) -> (usize, usize) {
        // Same but
        let (x, y) = self::Grid::map_to_grid(index);
        return (x * 3, y * 3);
    }

    pub fn map_to_grid(index: usize) -> (usize, usize) {
        // Floor x to map in range [0..2] per row
        // Mode y to map in ragne [0..2] per column
        return (index / 3, index % 3)
    }


    pub fn get_indices(subgrid_index: usize) -> (usize, usize) {
        let tuple = self::Grid::map_to_subgrid(subgrid_index);
        return tuple
    }
}


impl Grid {
    pub fn count_missing(&self, bitmask: u8) -> u8 {
        let mut zeros: u8 = 0;
        for position in 0..9 {
            if bitmask >> position as u8 == 0 {
                zeros += 1;
            }
        }

        return zeros;
    }

    fn _count_duplicates(&self, array: &Array1<u8>) -> usize {
        let mut seen = [false; 9]; // Fixed-size array to track if a value has been seen
        let mut duplicates = 0;

        for &value in array.iter() {
            let zero_based_value = value - 1;
            if seen[zero_based_value as usize] {
                duplicates += 1;
            } else {
                seen[zero_based_value as usize] = true;
            }
        }

        duplicates
    }

    pub fn check_row(&self, index: usize) -> usize {
        let row = self.matrix.row(index).to_owned();
        let collisions = self._count_duplicates(&row);

        return collisions;
    }

    pub fn check_col(&self, index: usize) -> usize {
        let col = self.matrix.column(index).to_owned();
        let collisions = self._count_duplicates(&col);

        return collisions;
    }

    pub fn check_subgrid(&self, index: usize) -> usize {
        0
    }
}

#[cfg(test)]
mod grid_tests;