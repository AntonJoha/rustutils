use crate::internal;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};


pub fn run(mut args: Vec<String>) -> i32 {
    

    let i = match internal::open_file(args.remove(0)) {
        Ok(f) => f,
        Err(e) => {println!("{}", e); return 1; }
    };
    let mut out = match internal::create_file(args.remove(0)) {
        Ok(f) => f,
        Err(e) => {println!("{}", e); return 2; }
    };
    let buf = BufReader::new(i);

    for line in buf.lines() {
        write!(out, "{}\n", line.unwrap());
    }
    return 0;
}
