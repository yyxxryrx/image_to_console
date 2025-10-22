# The Python binding of the core library

## Example

This is a simple example to show how to use this library in Python

```Python
import image_to_console_python as itccp

img = itccp.open("path/image.png")
print(img.display())
# Or you can set display mode
# print(img.display(itccp.DisplayMode.Kitty))
```
