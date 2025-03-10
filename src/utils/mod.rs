use ndarray::Array2;
// use std::collections::HashSet;
// use std::hash::Hash;
use std::{fs, io, path::PathBuf};

// pub fn has_unique_elements<T>(iter: T) -> bool
// where
//     T: IntoIterator,
//     T::Item: Eq + Hash,
// {
//     let mut uniq = HashSet::new();
//     iter.into_iter().all(move |x| uniq.insert(x))
// }

pub fn compute_mean(values: &[f32]) -> f32 {
    let sum: f32 = values.iter().sum();
    let count = values.len() as f32;
    sum / count
}

pub fn compute_standard_deviation(values: &[f32]) -> Option<f32> {
    if values.is_empty() {
        return None;
    }

    let mean = compute_mean(values);

    let sum_of_squares: f32 = values.iter().map(|&x| (x - mean).powi(2)).sum();

    let variance = sum_of_squares / (values.len() as f32);
    Some(variance.sqrt())
}

pub fn read_from_file(path: &PathBuf) -> String {
    let read_op = fs::read_to_string(path);

    match read_op {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    }
}

pub fn cast_to_array(from_file: &PathBuf) -> Array2<u8> {
    let content = read_from_file(from_file).replace('\n', "");

    let tmp: Vec<u8> = content
        .chars()
        .map(|char| {
            if char.is_ascii_digit() {
                (char as u8) - b'0'
            } else {
                panic!("Character is not a valid digit");
            }
        })
        .collect();

    let mut arr = Array2::<u8>::default((9, 9));
    let mut row: usize = 0;
    for (index, value) in tmp.iter().enumerate() {
        if index != 0 && index % 9 == 0 {
            row += 1;
        }

        arr[[row, index % 9]] = *value;
    }

    arr
}

pub fn list_dir() -> io::Result<Vec<PathBuf>> {
    let sudoku_dir = std::env::var("TEMPLATE_DIR").expect("Could not load TEMPLATE_DIR!");
    let mut grids = fs::read_dir(sudoku_dir)?
        .map(|files| files.map(|file| file.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    grids.sort();
    Ok(grids)
}

#[cfg(test)]
mod utils_tests;
