
pub struct Config{
    pub recursive: bool,
    pub file: String,
}


pub fn parse_args(mut args: Vec<String>) -> Config {

    let mut conf: Config = Config{
            recursive: false,
            file: String::new()};

    while args.len() > 1 {
        let s = args.remove(0);

        match s.as_str() {
            "-r" => conf.recursive = true,
            _ => println!("Unknown argument given {}", s),
        };
    }
    
    conf.file = args.remove(0);
    return conf;

}
