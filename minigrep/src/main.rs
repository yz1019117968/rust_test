use std::env;
use std::process;
use minigrep::{Config, run};
fn main() {
    let args: Vec<String> = env::args().collect();
    // 成功执行就返回值并解包裹，否则执行闭包内函数，闭包相当于匿名函数，|err|为参数
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("ERR INFO: {}", err);
        process::exit(1);
    });
    println!("search for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = run(config) {
        println!("ERR INFO: {}", e);
    }
}


