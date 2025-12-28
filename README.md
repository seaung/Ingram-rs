# Ingram Rust 版本使用指南

## 编译程序

### 前置要求

在编译之前，请确保已安装 Rust 工具链：

**Windows:**
```bash
# 下载并运行 Rust 安装程序
# 访问 https://rustup.rs/ 下载 rustup-init.exe
# 或者在 PowerShell 中运行：
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```

**macOS:**
```bash
# 使用 Homebrew 安装
brew install rust

# 或使用官方安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Linux:**
```bash
# 使用官方安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或使用包管理器（以 Ubuntu/Debian 为例）
sudo apt update
sudo apt install rustc cargo
```

### 编译步骤

在项目根目录下运行：

```bash
cargo build --release
```

### 编译后的可执行文件位置

**Windows:**
- 可执行文件：`target/release/ingram-rs.exe`

**macOS / Linux:**
- 可执行文件：`target/release/ingram-rs`

### 跨平台编译

如果需要在一个平台上编译其他平台的可执行文件：

**在 Windows 上编译 Linux 版本:**
```bash
# 安装 Linux 目标
rustup target add x86_64-unknown-linux-gnu

# 编译 Linux 版本
cargo build --release --target x86_64-unknown-linux-gnu
```

**在 Linux 上编译 Windows 版本:**
```bash
# 安装 Windows 目标
rustup target add x86_64-pc-windows-gnu

# 编译 Windows 版本
cargo build --release --target x86_64-pc-windows-gnu
```

**在 macOS 上编译 Linux 版本:**
```bash
# 安装 Linux 目标
rustup target add x86_64-unknown-linux-gnu

# 编译 Linux 版本
cargo build --release --target x86_64-unknown-linux-gnu
```

## 基本用法

**Windows:**
```bash
ingram-rs.exe -i <目标文件> -o <输出目录>
```

**macOS / Linux:**
```bash
./ingram-rs -i <目标文件> -o <输出目录>
```

## 命令行参数

### 必需参数

- `-i, --in-file <IN_FILE>`: 要扫描的目标文件，每行一个目标（IP 或 IP:端口）
- `-o, --out-dir <OUT_DIR>`: 结果输出目录

### 可选参数

- `-p, --ports <PORTS>`: 要检测的端口，默认为 80,81,8000,8080,8081
- `-t, --th-num <TH_NUM>`: 线程数，默认为 300
- `-T, --timeout <TIMEOUT>`: 请求超时时间（秒），默认为 3
- `-D, --disable-snapshot`: 禁用快照功能
- `--debug`: 启用调试模式
- `-h, --help`: 显示帮助信息

## 目标文件格式

创建一个文本文件（例如 `targets.txt`），每行一个目标：

```
192.168.1.1
192.168.1.2
192.168.1.3:8080
10.0.0.1
```

## 使用示例

### 1. 基本扫描

**Windows:**
```bash
ingram-rs.exe -i targets.txt -o results
```

**macOS / Linux:**
```bash
./ingram-rs -i targets.txt -o results
```

### 2. 指定端口扫描

**Windows:**
```bash
ingram-rs.exe -i targets.txt -o results -p 80,8080,8000
```

**macOS / Linux:**
```bash
./ingram-rs -i targets.txt -o results -p 80,8080,8000
```

### 3. 调整线程数和超时

**Windows:**
```bash
ingram-rs.exe -i targets.txt -o results -t 500 -T 5
```

**macOS / Linux:**
```bash
./ingram-rs -i targets.txt -o results -t 500 -T 5
```

### 4. 禁用快照功能

**Windows:**
```bash
ingram-rs.exe -i targets.txt -o results -D
```

**macOS / Linux:**
```bash
./ingram-rs -i targets.txt -o results -D
```

### 5. 启用调试模式

**Windows:**
```bash
ingram-rs.exe -i targets.txt -o results --debug
```

**macOS / Linux:**
```bash
./ingram-rs -i targets.txt -o results --debug
```

## 输出目录结构

扫描完成后，输出目录将包含以下结构：

```
results/
├── result.txt          # 扫描结果摘要
├── snapshots/          # 设备快照（如果未禁用）
│   ├── 192.168.1.1_80.jpg
│   ├── 192.168.1.2_8080.jpg
│   └── ...
└── debug.log           # 调试日志（如果启用调试模式）
```

## 支持的设备类型

当前版本支持以下设备的漏洞检测：

- **大华（Dahua）**: 弱口令漏洞
- **海康威视（Hikvision）**: 弱口令漏洞

## 功能说明

1. **端口扫描**: 检测指定端口是否开放
2. **设备指纹识别**: 通过 HTTP 响应识别设备类型
3. **漏洞验证**: 使用 POC 验证设备是否存在已知漏洞
4. **快照捕获**: 从存在漏洞的设备捕获快照（需要设备支持）
5. **并发扫描**: 使用多线程提高扫描效率
6. **进度显示**: 实时显示扫描进度和发现的设备数量

## 注意事项

1. 请确保你有权限扫描目标设备
2. 扫描大量目标时，建议适当调整线程数以避免网络拥塞
3. 快照功能需要设备支持且网络连接稳定
4. 调试模式会产生大量日志，建议仅在需要时使用
5. 请遵守相关法律法规，仅用于合法的安全测试

## 故障排除

### 编译错误

如果遇到编译错误，请确保：
- 已安装 Rust 工具链（rustup, cargo）
- 所有依赖项都能正常下载

### 运行时错误

如果程序运行时出错：
1. 检查目标文件格式是否正确
2. 确保输出目录有写入权限
3. 检查网络连接是否正常
4. 使用 `--debug` 参数查看详细日志

### 性能问题

如果扫描速度较慢：
1. 适当增加线程数（`-t` 参数）
2. 减少超时时间（`-T` 参数）
3. 减少扫描的端口数量
4. 禁用快照功能（`-D` 参数）

## 开发信息

- 项目语言: Rust
- 主要依赖: tokio, reqwest, clap, crossbeam, indicatif
- 并发模型: 基于线程池的并发扫描
- 数据管理: 使用 Arc 和 Atomic 类型实现线程安全


## 致谢
[Jorhelp/Ingram](https://github.com/jorhelp/Ingram)

我只是项目的搬运工...

---
that's all
