use std::fs::File;

pub fn open_file(path: String) -> Result<File, String> {
    match File::open(path) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string())
    }
}

pub fn create_file(path: String) -> Result<File, String> {
    match File::create(path) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string())
    }
}
