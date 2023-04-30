use std::env;
use std::error::Error;
use std::fs;
// ()代表什么都不返回 dyn代表动态
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let res: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_intensitive(&config.query, &contents)
    };
    // 问号加了之后不会发生恐慌，会将错误值返回给调用者

    for line in res {
        println!("With text:\n{}", line);
    }

    // 如果操作成功将什么都不返回
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // 返回一个Result枚举，如果操作成功就返回config,若是失败返回一个字符串
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argument");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // 返回一个元组
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// 返回的字符串是从contents里面所获得的所以他们的生命周期要相同
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    res
}

pub fn search_case_intensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line)
        }
    }
    res
}

// 测试驱动开发
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "safe";
        let contents = "\
Rust safe,
false productive
Safe aaaa";

        assert_eq!(vec!["Rust safe,"], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "safe";
        let contents = "\
Rust Safe,
false productive
Duct aaaa";

        assert_eq!(vec!["Rust Safe,"], search_case_intensitive(query, contents));
    }
}
