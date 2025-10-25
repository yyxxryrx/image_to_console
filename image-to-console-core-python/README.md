# The Python binding of the core library

> with use `pyo3`

## Example

This is a simple example to show how to use this library in Python

```Python
import image_to_console_core_python as itccp

img = itccp.open("path/image.png")
print(img.display())
# Or you can set display mode
# e.g. Kitty mode
# print(img.display(itccp.DisplayMode.Kitty))
```

## Dependencies

| Crate                                 | Version | License          | Purpose                    |
|---------------------------------------|---------|------------------|----------------------------|
| [pyo3](https://crates.io/crates/pyo3) | 0.27.1  | MIT / Apache-2.0 | Python bindings (optional) |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.