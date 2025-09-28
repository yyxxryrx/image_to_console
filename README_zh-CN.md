# image_to_console

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Last Version](https://img.shields.io/github/v/release/yyxxryrx/image_to_console
)](https://github.com/yyxxryrx/image_to_console/releases/latest) [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/yyxxryrx/image_to_console)

一个用 Rust 编写的高性能终端图片查看工具。

[English](README.md) | 简体中文

## 功能特点

- 🖼️ **多种来源**: 支持从文件、URL、Base64 字符串和标准输入加载图片。
- 📁 **目录处理**: 支持批量处理整个目录中的图片文件。
- 🎨 **真彩支持**: 在支持的终端中显示彩色图片。
- ⚫ **灰度模式**: 支持将彩色图片转换为灰度字符艺术显示。
- ⚫ **黑色背景**: 灰度模式下支持黑色背景显示选项。
- ⚡ **并行处理**: 使用 Rayon 库进行并行计算，转换速度快。
- 📏 **自适应尺寸**: 自动缩放图片以适应终端窗口。
- ⌨️ **交互式暂停**: 可选择在显示后暂停，方便查看。
- 🖥️ **多种显示模式**: 支持全分辨率（默认）和半分辨率显示模式。
- 🖥️ **多种终端协议支持**: 支持 WezTerm、Kitty、iTerm2 和 Sixel 图片协议。
- 🎞️ **GIF 动画支持**: 支持在终端中播放 GIF 动画。
- 🎥 **视频播放支持**: 支持在终端中播放视频文件（需要启用 `video` 特性及 FFmpeg）。
- 🔊 **音频支持**: 支持为 GIF 动画添加音频轨道（需要启用 `audio_support` 特性）。
- 🗜️ **压缩支持**: 在普通协议模式下支持输出压缩。

## 支持的协议

- [x] WezTerm
- [x] Kitty
- [x] iTerm2
- [x] Sixel

## 特性说明

| 特性            | 依赖                                                 | 描述              | 是否默认启用                              |
|---------------|----------------------------------------------------|-----------------|-------------------------------------|
| reqwest       | reqwest                                            | HTTP 请求库        | <span style="color: green">✓</span> |
| audio_support | rodio                                              | 播放音频支持          | <span style="color: red">✗</span>   |
| crossterm     | crossterm                                          | 终端库             | <span style="color: green">✓</span> |
| gif_player    | gif, crossbeam-channel                             | 在终端播放 GIF 动画    | <span style="color: green">✓</span> |
| video_player  | ez-ffmpeg, video-rs, crossbeam-channel, **FFmpeg** | 在终端播放视频         | <span style="color: red">✗</span>   |
| sixel_support | quantette, nohash-hasher                           | 使用 Sixel 协议显示图像 | <span style="color: green">✓</span> |

## 安装

### 从 Release 下载

1. 访问 [GitHub Release 页面](https://github.com/yyxxryrx/image_to_console/releases)
2. 下载适用于您操作系统的预编译二进制文件
3. 解压并将二进制文件添加到系统 PATH 中

### 从源码编译

```bash
git clone https://github.com/yyxxryrx/image_to_console.git
cd image_to_console

# 编译基础版本（不包含视频支持）
# 此版本不需要 FFmpeg
cargo build --release

# 编译包含视频支持的版本
# 此版本需要 FFmpeg 库
cargo build --release --features video_player
```

编译后的二进制文件位于 `target/release/image_to_console`。

如果要构建包含视频支持的版本，需要先安装 FFmpeg 库：

- Ubuntu/Debian: `sudo apt-get install libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev`
- macOS: `brew install ffmpeg`
- Windows: 安装 FFmpeg 并确保 DLL 文件在您的 PATH 中

## 使用方法

### 基本用法

```bash
# 从文件加载图片
image_to_console file path/to/image.jpg

# 从目录加载所有图片
image_to_console directory path/to/directory

# 从 GIF 文件加载动画
image_to_console gif path/to/animation.gif

# 从 URL 加载图片
image_to_console url https://example.com/image.png

# 从 Base64 字符串加载图片
image_to_console base64 <base64-encoded-image-data>

# 从标准输入加载图片字节
cat image.jpg | image_to_console bytes

# 从文件加载视频（需要启用 video 特性）
image_to_console video path/to/video.mp4
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

# 半分辨率显示
image_to_console --half-resolution file image.jpg

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

# 使用 iTerm2 图片协议显示图片
image_to_console --protocol iterm2 file image.jpg

# 使用 Sixel 协议显示图片
image_to_console --protocol sixel file image.jpg

# 使用 Sixel 协议显示图片并设置最大颜色数
image_to_console --protocol sixel --max-colors 128 file image.jpg

# 启用压缩（仅在普通协议下可用）
image_to_console --enable-compression file image.jpg

# 自动检测终端协议（默认）
image_to_console --protocol auto file image.jpg

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

### GIF 子命令选项

> **注意**：此功能需要启用 `gif_player` 特性。

```bash
# 播放 GIF 动画（默认 10 FPS）
image_to_console gif animation.gif

# 播放 GIF 动画并设置帧率
image_to_console gif --fps 24 animation.gif

# 循环播放 GIF 动画
image_to_console gif --loop animation.gif

# 设置帧率并循环播放 GIF 动画
image_to_console gif --fps 30 --loop animation.gif

# 播放 GIF 动画并伴随音频（需要启用 audio_support 特性）
image_to_console gif --audio audio.mp3 animation.gif
```

### 视频子命令选项

> **注意**：此功能需要启用 `video_player` 特性及 FFmpeg 库。

```bash
# 播放视频文件（需要启用 video_player 特性）
image_to_console video path/to/video.mp4

# 播放视频文件并指定音频轨道（需要启用 video_player 特性）
image_to_console video --audio path/to/audio.mp3 path/to/video.mp4
```

## 显示模式说明

### 彩色模式

- **全分辨率彩色模式**（默认）：使用上下半色块显示，每个字符代表两个像素点
- **半分辨率彩色模式**（--half-resolution）：使用背景色块显示，每个字符代表一个像素点
- **WezTerm 模式**（--protocol wezterm）：使用 WezTerm 的内联图片协议直接显示原图
- **Kitty 模式**（--protocol kitty）：使用 Kitty 的内联图片协议直接显示原图
- **iTerm2 模式** (--protocol iterm2) ：使用 iTerm2 的内联图片协议直接显示原图
- **Sixel 模式** (--protocol sixel) ：使用 Sixel 协议在支持的终端中显示图像
- **半分辨率 Sixel 模式** (--protocol sixel --half-resolution) ：使用 Sixel 协议在支持的终端中显示图像，并使用全分辨率显示图片
- **自动检测模式** (--protocol auto) ：自动检测并使用最佳的终端协议（默认）

### 灰度模式

- **灰度模式**（--no-color）：将图片转换为灰度字符艺术显示
- 使用不同的 Unicode 字符（如 █、▀、▄、.、, 等）表示不同的灰度级别
- **黑色背景**（-b）：在灰度模式下使用黑色背景显示图片
- **WezTerm 灰度模式**（--protocol wezterm --no-color）：在 WezTerm 中显示灰度图
- **Kitty 灰度模式**（--protocol kitty --no-color）：在 Kitty 中显示灰度图
- **iTerm2 灰度模式** (--protocol iterm2 --no-color) ：在 iTerm2 中显示灰度图
- **自动检测灰度模式** (--protocol auto --no-color) ：自动检测并使用最佳的终端协议在灰度模式下显示

## 支持的图片格式

支持大多数常见的图片格式，包括但不限于：

- JPEG
- PNG
- GIF
- BMP
- ICO
- TIFF
- WebP

## 依赖库

| Crate                                                           | Version | License          | Purpose          |
|-----------------------------------------------------------------|---------|------------------|------------------|
| [clap](https://crates.io/crates/clap)                           | 4.5.20  | MIT / Apache-2.0 | 命令行参数解析          |
| [rayon](https://crates.io/crates/rayon)                         | 1.11.0  | MIT / Apache-2.0 | 数据并行计算           |
| [num_cpus](https://crates.io/crates/num_cpus)                   | 1.17.0  | MIT              | 获取逻辑 CPU 核心数     |
| [image](https://crates.io/crates/image)                         | 0.25.4  | MIT              | 图像编解码与处理         |
| [base64](https://crates.io/crates/base64)                       | 0.22.1  | MIT / Apache-2.0 | Base64 编解码       |
| [indicatif](https://crates.io/crates/indicatif)                 | 0.17.8  | MIT              | 终端进度条            |
| [terminal_size](https://crates.io/crates/terminal_size)         | 0.4.0   | MIT              | 检测终端尺寸           |
| [crossterm](https://crates.io/crates/crossterm)                 | 0.29.0  | MIT              | 终端控制（可选）         |
| [reqwest](https://crates.io/crates/reqwest)                     | 0.12.9  | MIT / Apache-2.0 | 阻塞式 HTTP 客户端（可选） |
| [gif](https://crates.io/crates/gif)                             | 0.13.3  | MIT              | GIF 动画解码（可选）     |
| [crossbeam-channel](https://crates.io/crates/crossbeam-channel) | 0.5.15  | MIT / Apache-2.0 | 跨线程通信（可选）        |
| [rodio](https://crates.io/crates/rodio)                         | 0.21.1  | MIT / Apache-2.0 | 音频播放（可选）         |
| [ez-ffmpeg](https://crates.io/crates/ez-ffmpeg)                 | 0.5.3   | MIT              | 视频处理（可选）         |
| [video-rs](https://crates.io/crates/video-rs)                   | 0.10.3  | MIT              | 视频处理（可选）         |
| [ndarray](https://crates.io/crates/ndarray)                     | 0.16.1  | MIT              | N维数组（可选）         |
| [quantette](https://crates.io/crates/quantette)                 | 0.3.0   | MIT              | Sixel 图像量化（可选）   |
| [nohash-hasher](https://crates.io/crates/nohash-hasher)         | 0.2.0   | MIT              | Sixel 快速哈希（可选）   |

## License

本项目采用 MIT 许可证 - 详情请见 [LICENSE](LICENSE) 文件。