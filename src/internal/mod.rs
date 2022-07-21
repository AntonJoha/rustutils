use std::fs::File;
use std::env;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};

pub fn files_in_dir(path: &String) -> Vec<String> {

    let mut toReturn: Vec<String> = Vec::new();
    for i in match fs::read_dir(path) {
        Ok(s) => s,
        _ => return toReturn,
    }{
        toReturn.push(match i {
            Ok(r) =>  {match r.file_name().into_string() { 
                Ok(s) => s, 
                _ => String::new()
                }
            },
            _ => String::new()
        });
    }
    return toReturn;
}

pub fn is_dir(path: &String) -> bool {
    let p = Path::new(path);
    return p.is_dir();
}

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
