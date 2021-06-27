use std::env;
use std::fs;

fn main() {

    let args:Vec<String>=env::args().collect();
    if args.len()<3{
        panic!("arguments no enough");
    }
    // let query=&args[1];
    // let filename=&args[2];
    let config=parse_config(&args);
    println!("query {}",config.query);
    println!("filename {}",config.filename);

    let contents=fs::read_to_string(config.filename).expect("filename error");
    println!("content \n{}",contents);

}

struct Config{
    query:String,
    filename:String,
}

fn parse_config(args:&[String])->Config{
    let query=args[1].clone();
    let filename=args[2].clone();
    Config{query,filename}
}
