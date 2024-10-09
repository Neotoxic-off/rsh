use std::path::Path;
use std::fs::{self, File};
use std::io::{self, Read};

pub fn is_directory(path: &String) -> bool {
    return Path::new(path).exists() && Path::new(path).is_dir();
}

pub fn is_file(path: &String) -> bool {
    return Path::new(path).exists() && Path::new(path).is_file();
}

pub fn get_directory(directory: &String, recursive: bool) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let entries = fs::read_dir(directory).unwrap();

    for entry in entries {
        let path: std::path::PathBuf = entry.unwrap().path();
        let path_str = path.to_string_lossy().to_string();
        let file_name = path.file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        if path.is_dir() && recursive == true {
            content.extend(get_directory(&path_str, true)); 
        } else {
            content.push(file_name);
        }
    }

    return content;
}

pub fn open_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    return Ok(contents);
}