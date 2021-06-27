use std::env;
use std::process;
use minigrep::Config;

fn main() {

    let args:Vec<String>=env::args().collect();

    let config=Config::new(&args);
    println!("query {}",config.query);
    println!("filename {}",config.filename);

    if let Err(e)=minigrep::run(config){
        println!("Application run error {}",e);
        process::exit(1);
    }
}





