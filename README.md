# image_to_console

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个用 Rust 编写的高性能终端图片查看工具。

## 功能特点

- 🖼️ **多种来源**: 支持从文件、URL 和 Base64 字符串加载图片。

- 🎨 **真彩支持**: 在支持的终端中显示彩色图片。

- ⚡ **并行处理**: 使用 Rayon 库进行并行计算，转换速度快。

- 📏 **自适应尺寸**: 自动缩放图片以适应终端窗口。

- ⌨️ **交互式暂停**: 可选择在显示后暂停，方便查看。

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