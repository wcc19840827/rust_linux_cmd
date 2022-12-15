/**
 * cat 的实现涉及格式化输出、文件读取以及迭代器相关方法的使用，还用到了 Trait Object 来组合多个函数的调用。
 * 标准库很实在、函数式跟抽象能力很不错，但上手难度还是比较高
 */

extern crate clap;
use clap::Parser;
use std::{io::{self, BufReader, BufRead}, fs::File, path::Path};

//clap3.0.0版本用 `help`代替了`about`

#[derive(Debug, Parser)]
#[clap(version = "0.1", about = "head - output the first part of files", author = "Franck <franckcl@icloud.com>")]
struct Opts {
    // 涉及保存参数值以及重复使用参数 这里使用 Option Arguments ，参数指定为 Option<Vec<i32>> 同时添加 required 和 multiple_occurrences
    #[clap(required = false, short = 'c', long = "bytes", name = "[-]K", multiple_occurrences = true, help = "print the first K bytes of each file; with the leading '-', print all but the last K bytes of each file")]
    c_bytes: Option<Vec<i32>>,
    // 除了与 -c 相同的属性之外 -n 的默认值是 10 这里指定默认值
    #[clap(required = false, short = 'n', long = "lines", default_value = "10", name = "[-N]", multiple_occurrences = true, help = "print the first N lines instead of the first 10; with the leading '-', print all but the last N lines of each file")]
    n_lines: Option<Vec<i32>>,
    #[clap(short = 'q', long = "quiet", multiple_occurrences = true, help = "never print headers giving file names")]
    q_quiet: bool,
    #[clap(short = 'v', long = "verbose", multiple_occurrences = true, help = "always print headers giving file names")]
    v_verbose: bool,
    // 支持多参数 Vec<String>
    #[clap(name = "FILE")]
    input: Vec<String>,
}

fn main() {
    //读取命令行参数
    let cmd = Opts::from_args();
    println!("{:?}", cmd);

    // do_head(cmd);
}

fn do_head(cmd : Opts) {
    // let copy_cmd = cmd.clone();

    // 逻辑跟 cat 那个实现一样，只不过判断的条件不同，这里判断 INPUT 的长度跟第一个元素是不是 `-`
    if cmd.input.len() >= 1 && cmd.input[0] != "-" {
        //INPUT为 输入文件
        do_show(cmd);//copy_cmd
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

//----------------
//输出逻辑处理
//----------------

fn do_show(cmd : Opts) {
    // 获取命令行参数 执行输出
    if let Some(n) = cmd.n_lines {
        for f in cmd.input {
            // 输出文件名
            if cmd.v_verbose {
                println!("==> {} <==", f);
            }
            if n.len() != 0 {
                // 获取最后一个 -n 参数
                let read_lines = n_lines(f, n[n.len() - 1]);
                match read_lines {
                    Ok(lines) => {
                        // 输出文件内容 这里其实直接 unwrap 也是可以的 i32 abs as uniz
                        for line in lines {
                            println!("{}", line.unwrap_or("".into()));
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
    }
}

// -n lines 处理 返回裁剪后的迭代器 这里有 i32 转换 usize 所以是一个 Result<String>
fn n_lines<P>(file: P, each: i32) -> io::Result<Box<dyn Iterator<Item=io::Result<String>>>> where P: AsRef<Path> {
    // io::Result<Lines<BufReader<File>>> where P: AsRef<Path>
    let f = File::open(file)?;
    let len_f = f.try_clone()?;
    // clone 一份用于获取总行数，这里不能 clone BufReader 不然会导致 buffer 失效无法获取文件内容
    let lines = BufReader::new(f).lines();
    match each {
        // 正数从开头获取
        n if n > 0 => {
            Ok(Box::new(lines.take(n as usize)))
        }
        // 负数从结尾去除
        n if n < 0 => {
            let len = BufReader::new(len_f).lines().count();
            Ok(Box::new(lines.take(len - n.abs() as usize)))
        }
        // 零返回None
        _ => {
            let len = BufReader::new(len_f).lines().count();
            Ok(Box::new(lines.skip(len)))
        }
    }
}
