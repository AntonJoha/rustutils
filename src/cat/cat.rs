use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn open_file(path: String) -> Result<File, String> {
    match File::open(path) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string())
    }
}


pub fn cat(args: Vec<String>) -> i32 {

    for f in args {
        let file = match open_file(f) {
            Ok(f) => BufReader::new(f),
            Err(e) => { println!("{}", e); return -1 }
        };
        for line in file.lines() {
            println!("{}", line.unwrap());
        }
    }
    return 0;
}
