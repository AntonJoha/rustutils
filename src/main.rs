use std::env;
mod cat;

fn convert_args(mut arg: Vec<String>)  -> Vec<String> {
    arg.remove(0);
    return arg;
}

fn main() {
    let mut args:  Vec<String> = env::args().collect();
    
    #[cfg(debug_assertions)]
    let args = convert_args(args);

    
    callfunc(args);
}

fn callfunc(arg: Vec<String>) -> i32 {


    let prog_name = &arg[0];
    let arguments = &arg[1..];
    println!("{}", prog_name);

    match prog_name.as_str() {
        "cat" => cat::run(arguments.to_vec()),
        "hello" => 10,
        _   => 11

    };

    return 1;
}

