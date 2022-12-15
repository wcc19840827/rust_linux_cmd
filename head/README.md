# head

## 需求分析
head 的逻辑跟 cat 差不多，都是对文件内容的输出，这里实现 head 完成之前 cat 没有添加的多文件功能（毕竟 head 的介绍直接说出来了）核心功能有以下几个：

1. 可以操作多个文件
2. 能指定输出量（按行或按字节）
3. 同类型参数覆盖 最后的生效

## 编译
```bash
cargo build
```

## 运行

### 命令解析
```bash
./target/debug/head --help
```
> 连接文件并在标准输出上打印

### 主要参数
```bash
# print the first K bytes of each file; with the leading '-', print all but the last K bytes of each file
# 打印每个文件的前 K 个字节；以'-'开头，打印除每个文件的最后 K 字节外的所有字节
-c, --bytes=[-]K

# print the first K lines instead of the first 10; with the leading '-', print all but the last K lines of each file
# 打印前 K 行而不是前 10 行；以'-'开头，打印除每个文件的最后 K 行以外的所有行 (目前只实现这个)
-n, --lines=[-]K

# never print headers giving file names
# 多个 INPUT 时不显示标题
-q, --quiet, --silent

# always print headers giving file names
# 多个 INPUT 时显示标题
-v, --verbose

```

### 用例
```bash
# 读取文件有几行
./target/debug/head README.md
./target/debug/head -n=5 README.md

## 多文件
./target/debug/head -n=5 -v README.md Cargo.toml

```
