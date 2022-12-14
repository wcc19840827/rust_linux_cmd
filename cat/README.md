# cat

## 编译
```bash
cargo build
```

## 运行

### 命令解析
```bash
./target/debug/cat --help
```
> 在终端上输出一行文本

### 主要参数
```bash
#不输出文本末尾的换行符
-n	 	do not output the trailing newline

#使用文本中的转义标记
-e 	enable interpretation of backslash escapes

#不使用文本中的转义标记（默认）
-E 	disable interpretation of backslash escapes (default)
```

### 用例
```bash
clear; cargo build; ./target/debug/cat > ./test << EOF

```
