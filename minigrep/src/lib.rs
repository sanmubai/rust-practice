use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}
impl Config{
    pub fn new(args:&[String])->Result<Config,&str>{
        if args.len()<3{
            return Err("not enough arguments");
        }
        let query=args[1].clone();
        let filename=args[2].clone();
        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query,filename,case_sensitive})
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.filename)?;
    println!("content \n{}",contents);
    let results=if config.case_sensitive{
        println!("case_sensitive");
        search(&config.query,&contents)
    }else{
        println!("case_insensitive");
        search_insensitive(&config.query,&contents)
    };
    for line in results{
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(query:&'a str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive<'a>(query:&'a str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    let query=query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query="test";
        let contents="\
test1
test2
lll
kkk";
        assert_eq!(vec!["test1","test2"],search(query,contents));
    }

    #[test]
    fn one_result_case_insensitive(){
        let query="test";
        let contents="\
Test1
tEst2
lll
kkk";
        assert_eq!(vec!["Test1","tEst2"],search_insensitive(query,contents));
    }
}