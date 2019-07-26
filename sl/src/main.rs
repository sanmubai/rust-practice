use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let file = File::open(config.filename).expect("cannot open file ");
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
//        for c in line.chars() {
//            print!("{}", c);
//        }
        if line.contains(&config.query) {
            println!("{}",line);
        }
    }
}
