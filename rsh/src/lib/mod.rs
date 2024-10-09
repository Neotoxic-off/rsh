mod io;

pub struct Io;

impl Io {
    pub fn is_directory(path: &String) -> bool {
        return io::is_directory(path);
    }
    
    pub fn is_file(path: &String) -> bool {
        return io::is_file(path);
    }
    
    pub fn get_directory(directory: &String, recursive: bool) -> Vec<String> {
        return io::get_directory(directory, recursive);
    }
    
    pub fn open_file(path: &str) -> std::io::Result<Vec<u8>> {
        return io::open_file(path);
    }
}
