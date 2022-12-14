extern crate clap;
use clap::Parser;
use std::fmt::Write;

#[derive(Parser, Debug)]
// 定义命令 版本、介绍、作者 等信息
#[clap(
    version = "0.1",
    about = "echo - display a line of text",
    author = "Ryan <wcc19840827@126.com>"
)]
// 定义命令 结构
struct Opts {
    // 定义 -n 参数的短写法以及说明 这里使用 flag 类型为 bool
    #[clap(short = 'n', help = "do not output the trailing newline")]
    flag_newline: bool,

    // 定义 -e 参数的短写法以及说明 这里使用 flag 类型为 bool
    #[clap(short = 'e', help = "enable interpretation of backslash escapes")]
    flag_enable_escapes: bool,

    // 定义 -E 参数的短写法以及说明 这里使用 flag 类型为 bool
    #[clap(
        short = 'E',
        help = "disable interpretation of backslash escapes (default)"
    )]
    flag_disable_escapes: bool,

    // 定义 INPUT 参数 这里使用 Option<String> 因为 echo 可以不接收任何INPUT参数，接收的参数为字符串文本
    #[clap(name = "STRING")]
    text: Option<String>,
}

fn main() {
    //读取命令行参数
    let opt = Opts::from_args();

    let tmp = opt.text;
    match &tmp {
        Some(text) => fn_output(&recognize(&text, opt.flag_enable_escapes), !opt.flag_newline), //println!("{:#?}", recognize(&text)),
        None => println!("Hello, world!"),          //panic!(), //"".to_string(),
    };
}

//------------------------
//编写标记替换以及标记处理的逻辑
//------------------------

// 这里定义一个函数来进行处理，接收一个 &str 类型，返回一个 String类型
fn recognize(s: &str, enable_escapes: bool) -> String {
    if  !enable_escapes {
        return s.to_string();
    }

    // 为了兼顾一些内存占用和性能，这里转换为字符向量在原地进行处理
    let mut chars = s.chars().collect::<Vec<char>>();

    let nil = char::default();//站位的标记
    let index_max = chars.len() - 1;
    
    // 通过下标循环，依次进行标记的匹配和处理，其实这里使用正则或者宏或者解析器会更好，因为尽量使用标准库以及宏我不会
    // 所以写的很啰嗦，但是逻辑比较容易理解
    for i in 0..chars.len() {
        if (i + 1) <= (index_max) {
            // 通过match匹配字符标记，这里用元组是因为所有args捕获进来的字符串文本Rust都会作为原始操作符r##来处理
            // 所以一些本身可以记为一个char的标准转义都被作为两个char来记录。所以要匹配/ + ？这样
            match (chars[i], chars[i + 1]) {
                ('\\', '0') => {
                    todo!()
                }

                ('\\', 'a') => {
                    chars[i] = nil;
                    chars[i + 1] = nil;
                }
                
                // \b 情况比较特殊，其他标记是向后匹配和处理，它是向前
                ('\\', 'b') => {
                    if i >= 1 && (i + 1) < index_max {
                        match chars[i - 1] {
                            '\n' | '\r' | '\t' => {}
                            'f' | 'v' => {
                                if i == 1 {
                                    chars[i - 1] = nil;
                                }
                            }
                            _ => {
                                chars[i - 1] = nil;
                            }
                        }
                    }
                    chars[i] = nil;
                    chars[i + 1] = nil;
                }
                
                ('\\', 'c') => {
                    return chars[0..i].iter().filter(|x| **x != nil).collect();
                }
                
                //删除标志之后的字符
                // \e 标记的逻辑处理，因为不是替换, 所以需要再匹配到 \e 之后再匹配余下的标记，然后根据顺序强弱
                // 分别处理，这里因为\0 和 \x 先不做实现所以置空了
                ('\\', 'e') => {
                    chars[i] = nil;
                    chars[i + 1] = nil;
                    if (i + 2) == index_max {
                        chars[i + 2] = nil;
                    }
                    if (i + 3) <= index_max {
                        match (chars[i + 2], chars[i + 3]) {
                            ('\\', 'a')
                            | ('\\', 'b')
                            | ('\\', 'e')
                            | ('\\', 'f')
                            | ('\\', 'n')
                            | ('\\', 'r')
                            | ('\\', 't')
                            | ('\\', 'v')
                            | ('\\', '\\') => {
                                chars[i + 2] = nil;
                                chars[i + 3] = nil;
                            }
                            ('\\', '0') => {
                                todo!()
                            }
                            ('\\', 'x') => {
                                todo!()
                            }
                            _ => {
                                chars[i + 2] = nil;
                            }
                        }
                    }
                }
                ('\\', 'f') => {}
                ('\\', 'n') => {
                    chars[i] = nil;
                    chars[i + 1] = '\n';
                }
                ('\\', 'r') => {
                    chars[i] = nil;
                    chars[i + 1] = '\r';
                }
                ('\\', 't') => {
                    chars[i] = nil;
                    chars[i + 1] = '\t';
                }
                // 想过相同按照 \f 标记
                ('\\', 'v') => {
                    chars[i + 1] = 'f';
                }
                ('\\', 'x') => {
                    todo!()
                }
                // 这里因为 \b 向前匹配的逻辑 \\ 的标记替换顺序跟其他相反
                ('\\', '\\') => {
                    chars[i] = '\\';
                    chars[i + 1] = nil;
                }
                _ => {}
            }
        }
    }

    // 替换与处理完成后将使用char::default()站位的标记去掉，转换成 String 返回
    chars.iter().filter(|x| **x != nil).collect()
}

//------------------------
//格式化输出
//------------------------

fn fn_output(output: &str, newline: bool) {
    let mut width = 0;
    let mut result: String = String::new();
    // 通过split将包含 \f 标记的文本分割
    let vec_output = output
        .split("\\f")
        .filter(|x| *x != "")
        .collect::<Vec<&str>>();
    // 根据标记分割的字符长度格式化输出，这里为了 -n 判断省事 用write!写字符串
    for i in 0..vec_output.len() {
        width += vec_output[i].len();

        //是最后一项
        if i == vec_output.len() - 1 {
            match write!(
                &mut result,
                "{line:>width$}",
                line = vec_output[i],
                width = width
            ) {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        } else {
            match writeln!(
                &mut result,
                "{line:>width$}",
                line = vec_output[i],
                width = width
            ) {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
    }
    
    if newline {
        print!("{}", result);
    } else {
        println!("{}", result); //不输出换行符
    }
}
