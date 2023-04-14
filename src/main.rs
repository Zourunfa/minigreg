use std::env; //collect
use std::fs;

// 二进制程序关注点分离的指导性原则
// 将程序拆分为main.rs和lib.rs 将业务逻辑放入lib.rs
// 当命令行解析逻辑较少的时候，将它放在main.rs
// 当命令行逻辑变复杂是，需要将main.rs的逻辑抽离到lib.rs

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents); //没发处理非法的unicode字符  // env::args_os()这个可以处理
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        // 返回一个元组
        Config { query, filename }
    }
}
