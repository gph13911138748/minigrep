use std::error::Error;
use std::fs;
#[allow(unused)]

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(args:&[String]) -> Result<Config, & 'static str> {
        //从命令行得到参数并进行配置
        //如果是运行多次的项目，建议不用clone，clone影响性能，可以选择更好的方式
        if args.len()<3 {
            return Err("参数不足");
        }
        Ok(Config { query: args[1].clone(), file_path: args[2].clone() })
    }
}

pub fn run(config:&Config)->Result<(), Box<dyn Error>>{
    //这里用Box是为了后面统一处理错误，先进行传播
    let contents = fs::read_to_string(&config.file_path)?;
    println!("with my love\n {}", contents);
    Ok(())
}


#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
    
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
    
        results
    }
}



