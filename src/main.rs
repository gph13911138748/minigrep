// in main.rs
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    //解析命令行参数
    let args: Vec<String> = env::args().collect();//args是一个可变数组
    
    //重构后的参数配置函数
    //采取更加缓和的方式处理数据
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
} 

