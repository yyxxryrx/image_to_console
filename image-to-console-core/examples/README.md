# The examples of core library

> These examples can help you to get started

## Examples

### Simple Example

> simple-example-\*.rs

This example show how to use this library display an image in terminal in fast

The examples will use with:

1. Default Option
2. Kitty Protocol
3. Iterm Protocol

### Resize Example

This example show how to resize image

#### 1. AutoResize

> resize-example-with-AutoResize\*.rs

This example will resize image to fit the terminal size

#### 2. CustomResize

> resize-example-with-CustomResize\*.rs

This example will resize image to fit the custom size

### Option Example

> option-example-\*.rs

These example will show how to create options

#### 1. Default Option

> simple-example.rs

This example will show how to create default option

#### 2. Chained calls

> option-example-chained.rs

This example will show how to use chained calls to create option

#### 3. Builder and Chained calls

> option-example-builder.rs

This example will show how to use builder and chained calls to create option

### Protocol Example

> protocol-example-\*.rs

#### 1. Builder

> protocol-example-builder.rs

This example will show how to use builder to get `DisplayMode`

#### 2. Builder

> protocol-example-chained.rs

This example will show how to use chained calls to get `DisplayMode`

### `show_image!` and `show_images!` Example

#### 1. `show_image!`

> simple-example2.rs
>
> show-image-with-option.rs

This example show how to use `show_image!` macro

#### 2. `show_images!`

> show-images*.rs

This example show how to use `show_images!` macro

### `process_images!` Example

#### 1. One image

> process-images-example-*.rs

This example show how to process one image with use `process_images!` macro

#### 2. Multiple images

> process-images-example-*.rs

This example show how to process multiple images with use `process_images!` macro

#### 3. Nothing

> process-images-nothing.rs

This example show if you give nothing to `process_images!` macro, it will return empty `Vec`

### Converter Example

> converter-example.rs

This example show how to use `ImageConverter` without `ImageProcessor`

## CLI Example

> example-cli/

This example show how to use this library in CLI

## Image Source

| Filename    | Source                                                                                 |
|-------------|----------------------------------------------------------------------------------------|
| flower.jpg  | <https://raw.githubusercontent.com/python-pillow/Pillow/main/Tests/images/flower.jpg>  |
| flower.jpg2 | <https://raw.githubusercontent.com/python-pillow/Pillow/main/Tests/images/flower2.jpg> |
