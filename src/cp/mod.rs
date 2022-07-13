use crate::internal;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

mod arguments;


fn copy_file(conf: arguments::Config) -> i32 {
        
    let i = match internal::open_file(conf.infile) {
        Ok(f) => f,
        Err(e) => {println!("{}", e); return 1; }
    };
    let mut out = match internal::create_file(conf.outfile) {
        Ok(f) => f,
        Err(e) => {println!("{}", e); return 2; }
    };
    let buf = BufReader::new(i);

    for line in buf.lines() {
        write!(out, "{}\n", line.unwrap());
    }
    return 0;
}

pub fn run(mut args: Vec<String>) -> i32 {
    
    
    let conf : arguments::Config = arguments::parse_args(args);


    if internal::is_dir(&conf.infile) {
        if conf.recursive {
            println!("TODO IMPLEMENT MKDIR");
            return 0;
        }
        else {
            println!("{} is a directory", conf.infile);
            return -2;
        }
    }
    else {
        return copy_file(conf); 
    }
    return 0;
}
