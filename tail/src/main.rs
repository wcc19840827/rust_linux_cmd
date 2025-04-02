/**
 * cat 的实现涉及格式化输出、文件读取以及迭代器相关方法的使用，还用到了 Trait Object 来组合多个函数的调用。
 * 标准库很实在、函数式跟抽象能力很不错，但上手难度还是比较高
 */

extern crate clap;
use clap::Parser;
use std::{io::{self, BufReader, BufRead, ErrorKind, Error}, fs::File, path::Path};

//clap3.0.0版本用 `help`代替了`about`

// 整体和 head 一致
#[derive(Debug, Parser)]
#[clap(version = "0.1", about = "tail - output the last part of files", author = "Franck <franckcl@icloud.com>")]
struct Opts {
    // 新增检查函数，-f 只能接受 name 或 descriptor
    #[clap(parse(try_from_str = parse_follow))]
    #[clap(required = false, short = 'f', long = "follow", name = "{name|descriptor}", help = "output appended data as the file grows;\nan absent option argument means 'descriptor'")]
    follow: Option<Vec<String>>,
    // 这里也新增检查函数，把 head 偷懒的 +K 补上
    #[clap(parse(try_from_str = parse_lines))]
    #[clap(required = false, short = 'n', long = "lines", default_value = "10", name = "[-K]", help = "output the last K lines, instead of the last 10; or use -n +K to output starting with the Kth")]
    // 这里类型是与检查函数返回一致的（也就是输入都是 &str 但是返回的可以定义为其他类型）
    lines: Option<Vec<i32>>,
    #[clap(long = "retry", help = "keep trying to open a file if it is inaccessible")]
    retry: bool,
    #[clap(short = 'v', long = "verbose", help = "always output headers giving file names")]
    verbose: bool,
    #[clap(required = false, short = 's', long = "sleep-interval", name = "[-N]", help = "with -f, sleep for approximately N seconds (default 1.0) between iterations; with inotify and --pid=P, check process P at least once every N seconds")]
    sleep: Option<i32>,
    // 支持多参数 Vec<String>
    #[clap(name = "FILE")]
    input: Vec<String>,
}

// 检查函数 接收 &str 返回 String
fn parse_follow(s: &str) -> io::Result<String> {
    if s.eq("name") || s.eq("descriptor") {
        Ok(s.into())
    } else {
        Err(Error::new(ErrorKind::InvalidData, "\nValid arguments are:\n  - 'descriptor'\n  - 'name'"))
    }
}

// 检查函数 接收 &str 返回 i32
fn parse_lines(s: &str) -> io::Result<i32> {
    if s.starts_with("+") {
        if let Ok(i) = &s[1..].parse::<i32>() {
            return Ok(*i);
        }
    } else {
        if let Ok(i) = s.parse::<i32>() {
            // 转成负数 后面直接用 head 返回 Lines 的实现
            return Ok(-i);
        }
    }
    // 这里直接用 std 的 Error 了，没用 anyhow 是因为尽量用标准库
    Err(Error::new(ErrorKind::InvalidData, format!("tail: {}: invalid number of lines", s)))
}

fn main() {
    //读取命令行参数
    let cmd = Opts::from_args();
    // println!("{:?}", cmd);

    do_tail(cmd);
}

//----------------
//文件操作与 INPUT 处理
//----------------

fn do_tail(cmd : Opts) {
    // 复用 head 的实现
    if cmd.input.len() >= 1 && cmd.input[0] != "-" {
        //INPUT为 输入文件
        do_show(cmd);
    } else {
        //INPUT为 - 或空
        let stdin = io::stdin(); //变量默认是不可改变
        loop {
            let mut buffer = String::new();
            match stdin.read_line(&mut buffer) {
                Ok(_) => {
                    if buffer.is_empty() {
                        break;
                    }
                    print!("{}", buffer);
                }
                Err(e) => {
                    // Err(e)
                    println!("Err:{:?}", e);
                }
            }
        }
    }
}

// 复用 head 的实现
// -n lines 处理 返回裁剪后的迭代器 这里有 i32 转换 usize 所以是一个 Result<String>
fn fn_lines<P>(file: P, each: i32) -> io::Result<Box<dyn Iterator<Item=io::Result<String>>>> where P: AsRef<Path> {
    let f = File::open(file)?;
    let len_f = f.try_clone()?;
    // clone 一份用于获取总行数，这里不能 clone BufReader 不然会导致 buffer 失效无法获取文件内容
    let lines = BufReader::new(f).lines();

    match each {
        // 正数从开头跳过n行
        n if n > 0 => {
            Ok(Box::new(lines.skip(n as usize)))
        }
        // 负数从结尾去除
        n if n < 0 => {
            let len = BufReader::new(len_f).lines().count();//FIXME:这样写会有问题
            Ok(Box::new(lines.skip(len - n.abs() as usize)))
        }

        // 零返回None
        _ => {
            let len = BufReader::new(len_f).lines().count();
            //TODO: take 换 skip
            Ok(Box::new(lines.skip(len)))
        }
    }
}

//----------------
//输出逻辑处理
//----------------

fn do_show(cmd : Opts) {
    match cmd.follow {
        Some(_) => {
            loop {
                //FIXME: 偷懒只拿第一个 file，下周有空改成 channel MPSC 实现多文件的 -f
                match fn_lines(&cmd.input[0], cmd.lines.as_ref().unwrap()[0]) {
                    Ok(mut lines) => {
                        loop {
                            let line = lines.next();
                            match line {
                                Some(s) => {
                                    println!("{}", s.unwrap());
                                }
                                None => {
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
        None => {
            for file in &cmd.input {
                // 偷懒 这部分有判断不会出错 直接 unwrap 拿 index 了
                if let Ok(lines) = fn_lines(file, cmd.lines.as_ref().unwrap()[0]) {
                    for line in lines {
                        if let Ok(line) = line {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
    }
}
