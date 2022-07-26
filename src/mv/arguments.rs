
pub struct Config {
    pub recursive: bool,
    pub infile: String,
    pub outfile: String,
}


pub fn parse_args(mut args: Vec<String>) -> Config {
    
    let mut conf: Config = Config{recursive: false, infile: String::new(),outfile: String::new()};
    
    while args.len() > 2 {
        let s = args.remove(0);

        match s.as_str() {
            "-r" => conf.recursive = true,
            _ => println!("Unknown argument given {}", s),
        };
    }
    
    conf.infile = args.remove(0);
    conf.outfile = args.remove(0);

    return conf
}
