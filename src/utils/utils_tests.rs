#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::env::current_dir;
    use std::path::{Path, PathBuf};
    use ndarray::{array, Array2, Axis};
    use crate::utils::{self, cast_to_array};

    fn create_template() -> Array2<u8> {
       let data = array![
            [0, 8, 9, 5, 4, 1, 6, 2, 0],
            [0, 4, 2, 6, 9, 3, 7, 8, 0],
            [6, 5, 3, 2, 8, 7, 4, 1, 9],
            [9, 6, 4, 1, 2, 5, 3, 7, 8],
            [8, 2, 1, 7, 3, 9, 5, 4, 6],
            [3, 7, 5, 8, 6, 4, 2, 9, 1],
            [2, 1, 7, 9, 5, 6, 8, 3, 4],
            [4, 9, 6, 3, 7, 8, 1, 5, 2],
            [5, 3, 8, 4, 1, 2, 9, 6, 7],
        ];

        return data;
    }

    fn create_testing_path() -> PathBuf {
        let path_str = Path::new("static/off_by_one.txt");
        let cur_dir = current_dir();
        let base_path = match cur_dir {
            Ok(path_buf) => path_buf,
            Err(error) => panic!("Error trying to read from file: {error:?}"),
        };

        let base_path = base_path.as_path();
        let sudoku_template_path_buf = base_path.join(path_str);
        return sudoku_template_path_buf;
    }

    #[test]
    fn test_file_reading() {
        let sudoku_template_path_buf = create_testing_path();
        let contents = utils::read_from_file(&sudoku_template_path_buf);

        assert!(Path::is_file(sudoku_template_path_buf.as_path()));
    }

    #[test]
    fn test_cast_to_array() {
        let sudoku_template_path_buf = create_testing_path();
        let data = create_template();
        let arr = cast_to_array(&sudoku_template_path_buf);

        assert!(data.type_id() == arr.type_id());
        assert!(data == arr);
    }

}