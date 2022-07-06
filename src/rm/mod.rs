use std::fs;

pub fn run(args: Vec<String>) -> i32 {

    for line in args {
        match fs::remove_file(line) {
            Ok(_) => (),
            Err(e) => {println!("{}", e); return 1;}
        }
    };
    return 0;
}
