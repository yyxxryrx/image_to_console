## Contributing

Contributions are welcome! Here are some ways you can help:

### Testing on Different Platforms

This library is primarily developed and tested on **Windows**. We especially welcome testing and feedback on:

- **macOS**: Kitty and iTerm2 protocol implementations
- **Linux**: Kitty protocol and various terminal emulators
- **All platforms**: Sixel protocol support in different terminals

If you encounter issues with any terminal protocol, please open an issue with:
- Your operating system and version
- Terminal emulator and version
- The protocol you're trying to use
- Expected vs actual behavior

### Code Contributions

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and add tests
4. Ensure all tests pass (`cargo test` and `cargo test --features sixel`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Reporting Issues

Found a bug? Please open an issue on [GitHub](https://github.com/yyxxryrx/image_to_console/issues) with:
- A clear description of the problem
- Steps to reproduce
- Expected behavior
- Actual behavior
- Your environment (OS, terminal, Rust version)
