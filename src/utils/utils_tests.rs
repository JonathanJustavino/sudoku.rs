#[cfg(test)]
mod tests {
    use crate::utils::{self, cast_to_array};
    use ndarray::{array, Array2};
    use std::any::Any;
    use std::env::current_dir;
    use std::path::{Path, PathBuf};

    fn create_template() -> Array2<u8> {
        let data: Array2<u8> = array![
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

        data
    }

    fn create_testing_path() -> PathBuf {
        let path_str = Path::new("static/off_by_one.txt");
        let cur_dir = current_dir();
        let base_path = match cur_dir {
            Ok(path_buf) => path_buf,
            Err(error) => panic!("Error trying to read from file: {error:?}"),
        };

        let base_path = base_path.as_path();

        base_path.join(path_str)
    }

    #[test]
    fn test_file_reading() {
        let sudoku_template_path_buf = create_testing_path();
        let mut file_read = utils::read_from_file(&sudoku_template_path_buf);
        let content = r#"
            089541620
            042693780
            653287419
            964125378
            821739546
            375864291
            217956834
            496378152
            538412967"#;

        let mut content = content.to_string();
        content = content.replace('\n', "");
        content = content.replace(' ', "");
        file_read = file_read.replace('\n', "");

        assert!(Path::is_file(sudoku_template_path_buf.as_path()));
        assert_eq!(file_read, content);
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
