use crate::cp;
use crate::rm;
mod arguments;
use crate::internal;

fn handle_recursive(conf: arguments::Config) -> i32 {
    let ret = cp::run(vec!("-r".to_string(), conf.infile.as_str().to_string(), conf.outfile.as_str().to_string()));
    if ret != 0 {
        return 1;
    }
    return rm::run(vec!("-r".to_string(), conf.infile.as_str().to_string()));
    return 0;
}

pub fn run(arg: Vec<String>) -> i32 {
    
    let conf: arguments::Config = arguments::parse_args(arg);
    
    if internal::is_dir(&conf.infile) {
        return handle_recursive(conf);
    }
    
    let ret = cp::run(vec!(conf.infile.as_str().to_string(), conf.outfile.as_str().to_string()));
    if (ret != 0) {
        return 1;
    }
    return rm::run(vec!(conf.infile, conf.outfile));

}
