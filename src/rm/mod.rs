use std::fs;
use crate::internal;
mod arguments;



fn remove(name: String) -> i32 {
    match fs::remove_file(name) {
            Ok(_) => (),
            Err(e) => {println!("{}", e); return 1;}
    }
    return 0;
}

fn handle_recursive(conf: arguments::Config) -> i32 {
    fs::remove_dir_all(conf.file);
    return 0;

    let files = internal::files_in_dir(&conf.file);
    for i in files {
        let mut newfile: String = String::new();
        newfile.push_str(conf.file.as_str());
        newfile.push_str("/");
        newfile.push_str(i.as_str());
        run(vec!("-r".to_string(), newfile));
    }
    remove(conf.file);
    return 0;
}

pub fn run(args: Vec<String>) -> i32 {
    
    let conf: arguments::Config = arguments::parse_args(args);
    
    if internal::is_dir(&conf.file) {
        if conf.recursive {
            return handle_recursive(conf);
        }
        else {
            println!("Error can't delete directory without -r flag");
            return -1;
        }
    }
    else {
        return remove(conf.file);
    }
}
