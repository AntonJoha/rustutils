
use crate::cp;
use crate::rm;

pub fn run(arg: Vec<String>) -> i32 {
    
    let ret = cp::run(arg[0..].to_vec());
    if (ret != 0) {
        return 1;
    }
    return rm::run(vec!(arg[0].clone()));

}
