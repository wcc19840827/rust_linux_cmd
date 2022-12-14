extern crate clap;
use clap::Parser;
use std::io;

//clap3.0.0版本用 `help`代替了`about`

#[derive(Parser, Debug)]
#[clap(version = "0.1", about = "cat - concatenate files and print on the standard output", author = "Franck <franckcl@icloud.com>")]
struct Opts {
    #[clap(short = 'A', long = "--show-all", help = "equivalent to -vET")]
    show_all: bool,
    #[clap(short = 'b', long = "--number-nonblank", help = "number nonempty output lines, overrides -n")]
    number_non_blank: bool,
    #[clap(short = 'e', help = "equivalent to -vE")]
    ve: bool,
    #[clap(short = 'E', long = "--show-ends", help = "display $ at end of each line")]
    show_ends: bool,
    #[clap(short = 'n', long = "--number", help = "number all output lines")]
    number: bool,
    #[clap(short = 's', long = "--squeeze-blank", help = "suppress repeated empty output lines")]
    squeeze_blank: bool,
    #[clap(short = 't', help = "equivalent to -vT")]
    vt: bool,
    #[clap(short = 'T', long = "--show-tabs", help = "display TAB characters as ^I")]
    show_tabs: bool,
    #[clap(short = 'u', help = "(ignored)")]
    ignored: bool,
    #[clap(short = 'v', long = "--show-nonprinting", help = "use ^ and M- notation, except for LFD and TAB")]
    show_non_printing: bool,
    // 这里其实应该使用 Vec<String> 来存储多个参数，因为是核心功能的简单实现而且多文件或者通配符的情况难免要引入
    // 外部的crate，毕竟这个自己来实现可能会比较多，所以这里简单来只处理一个文件
    #[clap(name = "FILE")]
    file: Option<String>,
}

fn main() {
    //读取命令行参数
    let cmd = Opts::from_args();
    // println!("{:?}", cmd);

    match cmd.file {
        Some(f) => {
            todo!();
        }
        
        // 手动输入内容,直接输入`EOF`为止
        // 处理 cat << EOF > FILE 
        None => {
            // 打开标准输入
            let mut stdin = io::stdin();
            // 循环处理输入的字符
            loop {
                let mut buffer = String::new();
                match stdin.read_line(&mut buffer) {
                    Ok(_) => {
                        // 无输入之后返回
                        if buffer.is_empty() {
                            break;
                        }
                        // 这里要用 print! 不能用 println! 不然会出现空行的情况
                        print!("{}", buffer);
                    }
                    Err(e) => {
                        println!("Err:{}", e);
                        // Err(e)
                    }
                }
            }
        }
    }
}

// fu doCat(file: Option<String>) {

// }