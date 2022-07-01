use std::env;


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
    println!("{:?}", arguments);

    match prog_name.as_str() {
        "test" => println!("Yes"),
        "hello" => println!("No"),
        _   => println!("No argument found suitable")

    }

    return 1;
}

