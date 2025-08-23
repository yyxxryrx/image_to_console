# image_to_console

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个用 Rust 编写的高性能终端图片查看工具。

## 功能特点

- 🖼️ **多种来源**: 支持从文件、URL 和 Base64 字符串加载图片。
- 🎨 **真彩支持**: 在支持的终端中显示彩色图片。
- ⚡ **并行处理**: 使用 Rayon 库进行并行计算，转换速度快。
- 📏 **自适应尺寸**: 自动缩放图片以适应终端窗口。
- ⌨️ **交互式暂停**: 可选择在显示后暂停，方便查看。
