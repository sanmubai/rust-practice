use std::error::Error;
use std::fs;

pub struct Config{
    pub query:String,
    pub filename:String,
}
impl Config{
    pub fn new(args:&[String])->Config{
        if args.len()<3{
            panic!("not enough arguments");
        }
        let query=args[1].clone();
        let filename=args[2].clone();
        Config{query,filename}
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.filename)?;
    println!("content \n{}",contents);
    Ok(())
}