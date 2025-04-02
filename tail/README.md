# tail

## 需求分析

输出文件最后的部分，默认是10行打印到标准输出，同时可以在开头输出文件名标题，如果没有指定 INPUT 列表或者 INPUT 为 - 则读取标准输入

![](https://pic2.zhimg.com/80/v2-22270ebdeddda581b0f0b3caec6747d1_720w.webp)

tail 的逻辑跟 head 差不多，核心功能有以下几个：

1. 可以操作多个文件
2. 能指定输出量
3. 重新打开文件

### 主要参数

```bash
# output the last K bytes; or use -c +K to output bytes starting with the Kth of each file
# 输出最后 K 个字节 或者 +K 从 K 个字节开始输出
-c, --bytes=K

# output appended data as the file grows; an absent option argument means 'descriptor'
# 随文件内容的追加输出数据
-f, --follow[={name|descriptor}]

# same as --follow=name --retry
# 相当于 指定 --follow 与 --retry 参数
-F

# 输出最后 K 行 或者 +K 从 K 行开始输出
-n, --lines=K

# with --follow=name, reopen a FILE which has not
# changed size after N (default 5) iterations to see if it has been unlinked or renamed (this is the usual case of rotated log files); with inotify, this option is rarely useful
# 当 --follow=name 时文件没有变更后在 N 次迭代之后检查变化
--max-unchanged-stats=N

# with -f, terminate after process ID, PID dies
# 当使用 -f 参数时通过此参数指定一个 pid 当 pid 终止后退出
--pid=PID

# never output headers giving file names
# 不输出文件标题
-q, --quiet, --silent

# keep trying to open a file if it is inaccessible
# 输入不存在时尝试重新打开文件（一般与 --max-unchanged-stats 同用）
--retry

# with -f, sleep for approximately N seconds (default 1.0) between iterations; with inotify and --pid=P, check process P at least once every N seconds
# 当使用 -f 参数时通过该参数指定 N 秒作为迭代间隔 指定 --pid 时 间隔 N 秒 检查 pid 存活
-s, --sleep-interval=N

# always output headers giving file names
# 始终输出文件标题
-v, --verbose
```

## 编译
```bash
cargo build
```

## 运行

### 命令解析
```bash
./target/debug/tail --help
```
> 连接文件并在标准输出上打印

### 用例
```bash
# 读取文件有几行
./target/debug/tail README.md
./target/debug/tail -n=5 README.md
./target/debug/tail -n=-5 README.md

## 多文件
./target/debug/tail -n=5 -v README.md Cargo.toml

```
