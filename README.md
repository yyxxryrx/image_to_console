# image_to_console

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Last Version](https://img.shields.io/github/v/release/yyxxryrx/image_to_console
)](https://github.com/yyxxryrx/image_to_console/releases/latest)

一个用 Rust 编写的高性能终端图片查看工具。

## 功能特点

- 🖼️ **多种来源**: 支持从文件、URL 和 Base64 字符串加载图片。
- 📁 **目录处理**: 支持批量处理整个目录中的图片文件。
- 🎨 **真彩支持**: 在支持的终端中显示彩色图片。
- ⚫ **灰度模式**: 支持将彩色图片转换为灰度字符艺术显示。
- ⚫ **黑色背景**: 灰度模式下支持黑色背景显示选项。
- ⚡ **并行处理**: 使用 Rayon 库进行并行计算，转换速度快。
- 📏 **自适应尺寸**: 自动缩放图片以适应终端窗口。
- ⌨️ **交互式暂停**: 可选择在显示后暂停，方便查看。
- 🖥️ **多种显示模式**: 支持全分辨率和半分辨率显示模式。
- 🖥️ **多种终端协议支持**: 支持 WezTerm、Kitty 图片协议。

## 支持的协议
- [x] WezTerm
- [x] Kitty
- [x] iTerm2
- [ ] Sixel

## 安装

### 从 Release 下载

1. 访问 [GitHub Release 页面](https://github.com/yyxxryrx/image_to_console/releases)
2. 下载适用于您操作系统的预编译二进制文件
3. 解压并将二进制文件添加到系统 PATH 中


### 从源码编译

```bash
git clone https://github.com/yyxxryrx/image_to_console.git
cd image_to_console
cargo build --release
```

编译后的二进制文件位于 `target/release/image_to_console`。

## 使用方法

### 基本用法

```bash
# 从文件加载图片
image_to_console file path/to/image.jpg

# 从目录加载所有图片
image_to_console directory path/to/directory

# 从 URL 加载图片
image_to_console url https://example.com/image.png

# 从 Base64 字符串加载图片
image_to_console base64 <base64-encoded-image-data>
```

### 命令行选项

```bash
# 显示帮助信息
image_to_console --help

# 居中显示图片
image_to_console -c file image.jpg

# 显示后暂停等待用户输入
image_to_console --pause file image.jpg

# 显示处理时间
image_to_console -t file image.jpg

# 全分辨率显示
image_to_console -f file image.jpg

# 灰度模式显示
image_to_console --no-color file image.jpg

# 灰度模式下使用黑色背景
image_to_console --no-color -b file image.jpg

# 禁用宽度缩放
image_to_console --without-resize-width file image.jpg

# 禁用高度缩放
image_to_console --without-resize-height file image.jpg

# 保存输出到文件
image_to_console -o output.txt file image.jpg

# 禁用图片打印（仅显示信息）
image_to_console --disable-print file image.jpg

# 禁用信息显示
image_to_console --disable-info file image.jpg

# 目录模式下一次性读取所有图片
image_to_console --read-all directory path/to/directory

# 使用 WezTerm 图片协议显示图片
image_to_console --protocol wezterm file image.jpg

# 使用 Kitty 图片协议显示图片
image_to_console --protocol kitty file image.jpg
```

### 文件子命令选项

```bash
# 隐藏文件名显示
image_to_console file --hide-filename image.jpg

# 显示文件名（默认）
image_to_console file image.jpg
```

### 目录子命令选项

```bash
# 处理目录中的所有图片
image_to_console directory path/to/directory

# 处理目录中的所有图片并一次性读取
image_to_console --read-all directory path/to/directory
```

## 显示模式说明

### 彩色模式
- **半分辨率彩色模式**（默认）：使用背景色块显示，每个字符代表一个像素点
- **全分辨率彩色模式**（-f）：使用上下半色块显示，每个字符代表两个像素点
- **WezTerm 模式**（--protocol wezterm）：使用 WezTerm 的内联图片协议直接显示原图
- **Kitty 模式**（--protocol kitty）：使用 Kitty 的内联图片协议直接显示原图

### 灰度模式
- **灰度模式**（--no-color）：将图片转换为灰度字符艺术显示
- 使用不同的 Unicode 字符（如 █、▀、▄、.、, 等）表示不同的灰度级别
- **黑色背景**（-b）：在灰度模式下使用黑色背景显示图片
- **WezTerm 灰度模式**（--protocol wezterm --no-color）：在 WezTerm 中显示灰度图
- **Kitty 灰度模式**（--protocol kitty --no-color）：在 Kitty 中显示灰度图

## 支持的图片格式

支持大多数常见的图片格式，包括但不限于：
- JPEG
- PNG
- GIF
- BMP
- ICO
- TIFF
- WebP

## Dependencies

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| [clap](https://crates.io/crates/clap) | 4.5.20 | MIT / Apache-2.0 | 命令行参数解析 |
| [rayon](https://crates.io/crates/rayon) | 1.11.0 | MIT / Apache-2.0 | 数据并行计算 |
| [num_cpus](https://crates.io/crates/num_cpus) | 1.17.0 | MIT | 获取逻辑 CPU 核心数 |
| [image](https://crates.io/crates/image) | 0.25.4 | MIT | 图像编解码与处理 |
| [base64](https://crates.io/crates/base64) | 0.22.1 | MIT / Apache-2.0 | Base64 编解码 |
| [indicatif](https://crates.io/crates/indicatif) | 0.17.8 | MIT | 终端进度条 |
| [terminal_size](https://crates.io/crates/terminal_size) | 0.4.0 | MIT | 检测终端尺寸 |
| [reqwest](https://crates.io/crates/reqwest) | 0.12.9 | MIT / Apache-2.0 | 阻塞式 HTTP 客户端 |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.