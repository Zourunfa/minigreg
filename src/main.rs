use std::env; //collect

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args); //没发处理非法的unicode字符  // env::args_os()这个可以处理
}
