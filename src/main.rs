use std::env;
//collect
use minigrep::Config;
use std::process;

// 二进制程序关注点分离的指导性原则
// 将程序拆分为main.rs和lib.rs 将业务逻辑放入lib.rs
// 当命令行解析逻辑较少的时候，将它放在main.rs
// 当命令行逻辑变复杂是，需要将main.rs的逻辑抽离到lib.rs

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_else是如果Result的返回失败，就会返回穿进去的闭包  unwrap就是打开包装
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Probelm parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Search for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }

    // let contents =
    //     fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents); //没发处理非法的unicode字符  // env::args_os()这个可以处理
}
