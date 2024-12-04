use std::fmt;
use crate::utils;
use ndarray::{self, s, Array2, ArrayView2};
use rand::{seq::{IteratorRandom, SliceRandom}, thread_rng};


#[derive(Clone, Debug)]
pub struct Grid {
    pub matrix: ndarray::Array2<u8>
}

impl fmt::Display for Grid {

    // fn fmt_1(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    //     write!(formatter,"{}", self.matrix.to_string())
    // }

    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.matrix {

        }

        write!(formatter,"{}", self.matrix.to_string())
    }
}

impl Grid {
    pub fn new(matrix: ndarray::Array2<u8>) -> Self {
        Self {matrix: matrix}
    }

    pub fn get_indices(subgrid_index: usize) -> (usize, usize) {
        match subgrid_index {
            0 => (0, 0),
            1 => (0, 3),
            2 => (0, 6),
            3 => (3, 0),
            4 => (3, 3),
            5 => (3, 6),
            6 => (6, 0),
            7 => (6, 3),
            _other => (6, 6)
        }
    }
}

impl Grid {

    pub fn generate_neighbor() {
        let mut rng = thread_rng();
        let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        v.shuffle(&mut rng);
    }

    pub fn collect_fixed_indices(&self, subgrid_index: usize) -> Vec<usize> {
        let (row, col) = self::Grid::get_indices(subgrid_index);
        let subgrid_slice = s![row..row + 3, col..col + 3];

        self.matrix
            .slice(subgrid_slice)
            .to_owned() // Ensure contiguity
            .into_shape((9,)) // Flatten into 1D
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(index, &value)| if value != 0 { Some(index) } else { None }) // Collect non-zero indices
            .collect()
    }

    pub fn collect_free_indices(&self, subgrid_index: usize) -> Vec<usize> {
        let mut free: Vec<usize> = vec![0, 1, 2, 3, 4,  5, 6, 7, 8];
        let fixed = self.collect_fixed_indices(subgrid_index);
        free.retain(|value| !fixed.contains(value));

        free
    }
}


impl Grid {

    pub fn insert_subgrid(&mut self, subgrid: &Array2::<u8>, index: usize) {
        let valid_dim: (usize, usize) = (3, 3);
        assert_eq!(subgrid.dim(), valid_dim, "Wrong dimensions of subgrid");

        let (row_start, col_start) = self::Grid::get_indices(index);
        self.matrix.slice_mut(s![row_start..row_start + 3, col_start..col_start + 3]).assign(subgrid);

        println!("{:?}", self);
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

    pub fn check_row(&self, index: usize) -> usize {
        let mut bitmask: u8 = 0;
        let row = self.matrix.row(index);

        //TODO: AND every number to create bitmask
        // 1 := number missing
        for elem in row.iter() {
            bitmask = bitmask & (*elem - 1);
        }
        // at the end check for amounts of ones in total number
        // let count = bitmask.iter().rev().tak

        return 0;


        // return total_elements - unique_elements.len();
    }

    // pub fn check_col(&self, index: usize) -> usize {
    //     let total_elements = 9;
    //     let row = self.matrix.column(index);
    //     let unique_elements: Vec<&u8> = row.iter().unique().collect();

    //     return total_elements - unique_elements.len();
    // }

    pub fn check_subgrid(&self, index: usize) -> usize {
        0
    }

}

#[cfg(test)]
mod grid_tests;