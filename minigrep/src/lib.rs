use std::error::Error;
use std::{env, fs, result};

// Result中的()代表啥也不需要返回
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename).expect("sth wrong!");
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else{
        search_in_sensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}", line);
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    // 此处对字符串向量的借用是不可修改的，参数仅指向该向量，但不获得其所有权
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("pls input enough params!");
        }
        // 拿到副本的所有权，来实例化Config结构体
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }

    results
} 

pub fn search_in_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
} 