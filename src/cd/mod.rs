use std::env;
use std::path::Path;
use crate::internal;

fn absolute_path(dir : String) -> i32 {

    let path = Path::new(dir.as_str());
    
    match env::set_current_dir(&path) {
        Err(e) => {println!("{:?}", e); return 3; }
        _ => (),
    };

    let current_dir = match internal::current_dir() {
        Ok(f) => f,
        Err(e) => { println!("{}", e); return 1;}
    };
    println!("{}", current_dir);

    return 0;
}

fn relative_path(dir : String) -> i32 {


    let mut current_dir = match internal::current_dir() {
        Ok(f) => f,
        Err(e) => { println!("{}", e); return 1;}
    };
    current_dir.push_str("/");
    current_dir.push_str(dir.as_str());

    return absolute_path(current_dir);
}

pub fn run(mut args: Vec<String>) -> i32 {

    let new_dir = args.remove(0);

    if new_dir.as_bytes()[0] ==  b'/' {
        return absolute_path(new_dir);
    }
    else {
        return relative_path(new_dir);
    }
}
