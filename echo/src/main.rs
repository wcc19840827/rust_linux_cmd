extern crate clap;
use clap::Parser;

fn main() {
    println!("Hello, world!");
}


#[derive(Parser, Debug)]
// 定义命令 版本、介绍、作者 等信息
#[clap(version = "0.1", about = "echo - display a line of text", author = "Franck <franckcl@icloud.com>")]
// 定义命令 结构
struct Opts {
    // 定义 -n 参数的短写法以及说明 这里使用 flag 类型为 bool
    #[clap(short = 'n', help = "do not output the trailing newline")]
    flag_newline: bool,
    // 定义 -e 参数的短写法以及说明 这里使用 flag 类型为 bool
    #[clap(short = 'e', help = "enable interpretation of backslash escapes")]
    flag_enable_escapes: bool,
    // 定义 -E 参数的短写法以及说明 这里使用 flag 类型为 bool
    #[clap(short = 'E', help = "disable interpretation of backslash escapes (default)")]
    flag_disable_escapes: bool,
    // 定义 INPUT 参数 这里使用 Option<String> 因为 echo 可以不接收任何INPUT参数，接收的参数为字符串文本
    #[clap(name = "STRING")]
    text: Option<String>,
}