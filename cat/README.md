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
> 连接文件并在标准输出上打印

### 主要参数
```bash
# 显示所有 相当于 -vET
-A, --show-all equivalent to -vET

# 显示非空行的行号 会覆盖 -n参数
-b, --number-nonblank number nonempty output lines, overrides -n

# 相当于 -vE
-e equivalent to -vE

# 行尾显示符号 $
-E, --show-ends display $ at end of each line

# 显示所有行的行号 包括空行
-n, --number number all output lines

# 临近的空行只输出一行
-s, --squeeze-blank suppress repeated empty output lines

# 相当于 -vT
-t equivalent to -vT

# 将TAB符显示为 ^|
-T, --show-tabs display TAB characters as ^I

# 忽略
-u (ignored)

# 显示 ^ 与 M- 的符号 除 LFD 与 TAB 之外
-v, --show-nonprinting use ^ and M- notation, except for LFD and TAB

```

### 用例
```bash
# 手动输入内容,直到输入`EOF`为止
clear; cargo build; ./target/debug/cat > ./test << EOF

# 读取文件内容
clear; cargo build; ./target/debug/cat ./test
clear; cargo build; ./target/debug/cat -E ./test

```
