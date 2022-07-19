use std::env;
use std::path::Path;
use crate::internal;
use std::fs;


pub fn run(args: Vec<String>) -> i32 {
    
    for f in args {
        fs::create_dir(f);
    }
    
    return 0;
}
