/**
 * cat 的实现涉及格式化输出、文件读取以及迭代器相关方法的使用，还用到了 Trait Object 来组合多个函数的调用。
 * 标准库很实在、函数式跟抽象能力很不错，但上手难度还是比较高
 */

extern crate clap;
use clap::Parser;
use std::{io::{self, Lines, BufReader, BufRead}, fs::File, path::Path, ops::Add};

//clap3.0.0版本用 `help`代替了`about`

#[derive(Parser, Debug, Clone)]
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

    do_cat(cmd);
}

fn do_cat(cmd : Opts) {
    let copy_cmd = cmd.clone();

    match cmd.file {
        //(1)读取文件
        Some(f) => {
            //读取文件所有行
            if let Ok(lines) = open_read_lines(f) {
                // (1)直接输出: 循环输出文件的每行
                // for line in lines {
                //     if let Ok(s) = line {
                //         println!("{}", s);
                //     }
                // }

                //(2) 带输出逻辑：行号、空行合并、TAB 替换、行尾 $
                do_show(copy_cmd, lines);
            }
        }
        
        //(2)手动输入内容,直到输入`EOF`为止
        // 处理 cat << EOF > FILE 
        None => {
            // 打开标准输入
            let stdin = io::stdin();
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
                        println!("Err:{:?}", e);
                        // Err(e)
                    }
                }
            }
        }
    }
}

fn do_show(cmd : Opts, lines: Lines<BufReader<File>>) {
    // 创建 handle_shows 的 Vec<handle> (用于保存函数对象)
    let mut handle: Vec<fn(Box<dyn Iterator<Item=String>>) -> Box<dyn Iterator<Item=String>>> = Vec::new();

    // 如果 -E 压入 show_ends 函数
    if cmd.show_ends {
        handle.push(show_ends);
    }
    // 如果 -T 压入 show_tabs 函数
    if cmd.show_tabs {
        handle.push(show_tabs);
    }
    // 如果 -s 压入 squeeze_blank 函数
    if cmd.squeeze_blank {
        handle.push(squeeze_blank);
    }
    if cmd.number_non_blank {
        // 如果 -b 压入 number_non_blank 函数
        handle.push(number_non_blank);
    } else {
        if cmd.number {
            // 如果 -n 压入 number 函数
            handle.push(number);
        }
    }

    //FLAG-Ryan: Box是智能指针
    for line in handle_shows(Box::new(lines.filter(|x| x.is_ok()).map(|x| x.unwrap())), handle) {
        println!("{}", line);
    }
}

//----------------
//实现打开文件与读取文件逻辑
//----------------

// 定义一个函数接收一个 实现 AsRef<Path>的泛型 参数，返回一个 BufRead 的 Lines 迭代器
fn open_read_lines<P>(file: P) -> io::Result<Lines<BufReader<File>>> where P: AsRef<Path> {
    let f = File::open(file)?;
    // 使用 BufReader 缓冲区加速大文件反复读取速度
    let lines = BufReader::new(f).lines();
    Ok(lines)
}

//----------------
//实现输出逻辑：行号、空行合并、TAB 替换、行尾 $
//----------------

// TAB 替换函数，接收、输出都使用 Trait Object 简化
fn show_tabs(lines: Box<dyn Iterator<Item=String>>) -> Box<dyn Iterator<Item=String>> {
    Box::new(lines.map(|x| x.replace("\t", "^I")))
}
// 行尾$ 函数，接收、输出都使用 Trait Object 简化
fn show_ends(lines: Box<dyn Iterator<Item=String>>) -> Box<dyn Iterator<Item=String>> {
    Box::new(lines.map(|x| x.add("$")))
}
// 行号非空，接收、输出都使用 Trait Object 简化
fn number_non_blank(lines: Box<dyn Iterator<Item=String>>) -> Box<dyn Iterator<Item=String>> {
    Box::new(lines.enumerate().map(|(x, y)| if y.is_empty() || y.eq("$") { y } else { format!("    {}    {}", x + 1, y) }))
}
// 行号为空,接收、输出都使用 Trait Object 简化
fn number(lines: Box<dyn Iterator<Item=String>>) -> Box<dyn Iterator<Item=String>> {
    Box::new(lines.enumerate().map(|(x, y)| format!("    {}    {}", x + 1, y)))
}
// 合并空行，接收、输出都使用 Trait Object 简化
fn squeeze_blank(lines: Box<dyn Iterator<Item=String>>) -> Box<dyn Iterator<Item=String>> {
    Box::new(lines.filter(|x| !x.is_empty() && !x.eq("$")))
}
// 处理函数，接收一个 Vec 循环调用 Next 函数 对迭代器进行处理
fn handle_shows(lines: Box<dyn Iterator<Item=String>>, handle: Vec<fn(Box<dyn Iterator<Item=String>>) -> Box<dyn Iterator<Item=String>>>) -> Box<dyn Iterator<Item=String>> {
    let mut lines = lines;
    for f in handle { //handle是各种要执行的处理函数
        lines = f(lines)
    }
    lines
}