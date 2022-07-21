use std::env;
mod mv;
mod cat;
mod cp;
mod rm;
mod internal;
mod cd;
mod mkdir;

fn convert_args(mut arg: Vec<String>)  -> Vec<String> {
    arg.remove(0);
    return arg;
}

fn main() {
    let args:  Vec<String> = env::args().collect();
    
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
        "rm" => rm::run(arguments.to_vec()),
        "cp" => cp::run(arguments.to_vec()),
        "mv" => mv::run(arguments.to_vec()),
        "cd" => cd::run(arguments.to_vec()),
        "mkdir" => mkdir::run(arguments.to_vec()),
        _   => 11

    }

}

