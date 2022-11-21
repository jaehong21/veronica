pub mod path {
    use std::fs;

    pub fn exist_dir(mut path: String) -> (String, String) {
        return match fs::read_dir(&path) {
            Ok(_) => {
                let file_path_arr: Vec<char> = path.chars().collect();
                if file_path_arr[file_path_arr.len() - 1] != '/' {
                    path.push_str("/");
                }
                (path.clone(), path.clone())
            },
            Err(_) => { panic!("No file or directory found: {}", &path); },
        };
    }
}
