use std::env;
use std::process;
use minigrep::Config;

fn main() {

    let config=Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("parse arguments error {}",err);
        process::exit(1);
    });

    if let Err(e)=minigrep::run(config){
        eprintln!("Application run error {}",e);
        process::exit(1);
    }
}





