use std::fs::File;
use std::env;
use std::path::Path;

pub fn current_dir() -> Result<String, String> {
    match env::current_dir() {
        Ok(f) => { match f.to_str() {
                Some(s) => Ok(s.to_string()),
                None => { Err("Couldn't get current working dir".to_string())}
            }
        },
        Err(e) => Err(e.to_string())
    }
}

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
