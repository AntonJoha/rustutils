use crate::internal;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use crate::mkdir;
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
            return handle_recursive(conf);       
        }
        else {
            println!("{} is a directory", conf.infile);
            return -2;
        }
    }
    else {
        return copy_file(conf); 
    }
}


fn handle_recursive(conf: arguments::Config) -> i32 {
    mkdir::run(vec!(conf.outfile.as_str().to_string()));
    let files = internal::files_in_dir(&conf.infile);
    for i in files {
        let mut newin: String = String::new();
        newin.push_str(conf.infile.as_str());
        newin.push_str("/");
        newin.push_str(i.as_str());
        let mut newout: String = String::new();
        newout.push_str(conf.outfile.as_str());
        newout.push_str("/");
        newout.push_str(i.as_str());
        run(vec!("-r".to_string(), newin, newout));
    }
    return 0;
}
