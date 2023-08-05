use std::fs;
use std::path::PathBuf;

fn load_test_file(file_path: &str) -> String {
    let file = PathBuf::from(file_path);
    fs::read_to_string(file).unwrap()
}

fn load_test_files(dir_path: &str) -> Vec<String> {
    let mut files = Vec::new();
    let dir = PathBuf::from(dir_path);
    for entry in fs::read_dir(dir).unwrap() {
        let file = entry.unwrap();
        if file.path().extension().map_or(false, |ext| ext == "txt") {
            let contents = fs::read_to_string(file.path()).unwrap();
            files.push(contents);
        }
    }
    files
}
