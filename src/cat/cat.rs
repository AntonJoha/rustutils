use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use  crate::internal;


pub fn cat(args: Vec<String>) -> i32 {

    for f in args {
        let file = match internal::open_file(f) {
            Ok(f) => BufReader::new(f),
            Err(e) => { println!("{}", e); return -1 }
        };
        for line in file.lines() {
            println!("{}", line.unwrap());
        }
    }
    return 0;
}
