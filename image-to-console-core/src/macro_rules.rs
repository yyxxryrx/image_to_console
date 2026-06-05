
#[macro_export]
/// A macro to display a single image to the terminal using default options or custom options.
///
/// This macro provides a convenient way to display images without manually setting up
/// the image processor and display protocol. It automatically detects the best terminal
/// protocol to use and processes the image accordingly.
///
/// # Arguments
///
/// * `$image` - An image of type `image::DynamicImage` to be displayed
/// * `$option` (optional) - Custom `ImageProcessorOptions` to control how the image is processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display an image with default options
/// show_image!(my_image);
///
/// // Display an image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii);
/// show_image!(my_image, options);
/// ```
#[cfg(all(not(feature = "auto_select"), feature = "processor"))]
macro_rules! show_image {
    ($image:expr) => {
        fn _show_image<T>(image: T)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let result = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image);
    };
    ($image: expr, $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image, $option);
    };
}

#[macro_export]
#[cfg(feature = "auto_select")]
/// A macro to display a single image to the terminal using default options or custom options.
///
/// This macro provides a convenient way to display images without manually setting up
/// the image processor and display protocol. It automatically detects the best terminal
/// protocol to use and processes the image accordingly.
///
/// # Arguments
///
/// * `$image` - An image of type `image::DynamicImage` to be displayed
/// * `$option` (optional) - Custom `ImageProcessorOptions` to control how the image is processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display an image with default options
/// show_image!(my_image);
///
/// // Display an image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii);
/// show_image!(my_image, options);
/// ```
macro_rules! show_image {
    ($image:expr) => {
        fn _show_image<T>(image: T)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let result = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image);
    };
    ($image: expr, $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image, $option);
    };
}

#[macro_export]
#[cfg(all(not(feature = "auto_select"), feature = "processor"))]
/// A macro to display multiple images to the terminal.
///
/// This macro allows displaying multiple images either with default options or with
/// shared custom options. It's useful when you want to display a series of images
/// with the same processing settings.
///
/// # Arguments
///
/// * `@vec $images` - A vector of images of type `Vec<image::DynamicImage>` to be displayed
/// * `$image` - One or more images of type `image::DynamicImage` to be displayed
/// * `@with_options $option` (optional) - Custom `ImageProcessorOptions` to control
///   how all images are processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display multiple images with default options
/// show_images!(image1, image2, image3);
///
/// // Display multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// show_images!(image1, image2, image3, @with_options options);
///
/// // Display images from a vector with default options
/// let image_vec = vec![image1, image2, image3];
/// show_images!(@vec image_vec);
///
/// // Display images from a vector with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// show_images!(@vec image_vec, @with_options options);
/// ```
macro_rules! show_images {
    (@vec $images:expr) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    (@vec $images:expr, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    ($($image:expr),+, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        $(
            _show_image($image, option);
        )+
    };
    ($($image:expr),+) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            println!("{}", option.create_processor(image).process().expect("Process image failed").display());
        }
        $(
            _show_image($image, option);
        )+
    };
}

#[macro_export]
#[cfg(all(feature = "auto_select", feature = "processor"))]
/// A macro to display multiple images to the terminal.
///
/// This macro allows displaying multiple images either with default options or with
/// shared custom options. It's useful when you want to display a series of images
/// with the same processing settings.
///
/// # Arguments
///
/// * `@vec $images` - A vector of images of type `Vec<image::DynamicImage>` to be displayed
/// * `$image` - One or more images of type `image::DynamicImage` to be displayed
/// * `@with_options $option` (optional) - Custom `ImageProcessorOptions` to control
///   how all images are processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display multiple images with default options
/// show_images!(image1, image2, image3);
///
/// // Display multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// show_images!(image1, image2, image3, @with_options options);
///
/// // Display images from a vector with default options
/// let image_vec = vec![image1, image2, image3];
/// show_images!(@vec image_vec);
///
/// // Display images from a vector with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// show_images!(@vec image_vec, @with_options options);
/// ```
macro_rules! show_images {
    (@vec $images:expr) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    (@vec $images:expr, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    ($($image:expr),+, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        $(
            _show_image($image, option);
        )+
    };
    ($($image:expr),+) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            println!("{}", option.create_processor(image).process().expect("Process image failed").display());
        }
        $(
            _show_image($image, option);
        )+
    };
}

#[doc(hidden)]
#[cfg(all(not(feature = "auto_select"), feature = "processor"))]
#[macro_export]
macro_rules! __vec_process_images {
    ($images: expr, $mode:ident, $var:ident, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();
            let images: Vec<$crate::image::DynamicImage> = $images;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
    ($images: expr, $mode:ident, $var:ident, options: $options: expr, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;

            let options: $crate::processor::ImageProcessorOptions = $options;
            let images: Vec<$crate::image::DynamicImage> = $images;

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
}

#[doc(hidden)]
#[cfg(all(feature = "auto_select", feature = "processor"))]
#[macro_export]
macro_rules! __vec_process_images {
    ($images: expr, $mode:ident, $var:ident, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();
            let images: Vec<$crate::image::DynamicImage> = $images;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
    ($images: expr, $mode:ident, $var:ident, options: $options: expr, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;

            let options: $crate::processor::ImageProcessorOptions = $options;
            let images: Vec<$crate::image::DynamicImage> = $images;

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
}

#[macro_export]
#[cfg(all(not(feature = "auto_select"), feature = "processor"))]
/// A macro to process one or more images and return the processed results.
///
/// This macro provides flexible ways to process images with various options and
/// automatically handles parallel processing for better performance when dealing
/// with multiple images. It supports both single image processing and batch
/// processing with optional custom options.
///
/// For batch processing of more than 10 images, this macro automatically uses
/// parallel processing via the rayon crate to improve performance. For smaller
/// batches or single images, sequential processing is used.
///
/// # Arguments
///
/// * `()` - Returns an empty vector when called with no arguments
/// * `$image:expr` - Process a single image with default options
/// * `$image:expr, @with_options $options:expr` - Process a single image with custom options
/// * `$($image:expr),+` - Process multiple images with default options
/// * `$($image:expr),+, @with_options $options:expr` - Process multiple images with custom options
/// * `@vec $images:expr` - Process a vector of images with default options
/// * `@vec $images:expr, @with_options $options:expr` - Process a vector of images with custom options
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @map $block:block` - Process a vector of images with custom options and map operation
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with custom options and for_each operation
/// * `@vec $images:expr, @var $var:ident, @map $block:block` - Process a vector of images with default options and map operation
/// * `@vec $images:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with default options and for_each operation
/// * `@with_options $options:expr, []=> $($mut:tt)? $name:ident` - Create an empty result vector with a specified name and optional mutability
/// * `@with_options $options:expr, [$($image:expr),*]=> $($mut:tt)? $name:ident` - Process multiple images into a named variable with optional mutability
/// * `[$($image:expr),*]=> $($mut:tt)? $name:ident` - Process multiple images with default options into a named variable with optional mutability
/// * `$image:expr=> $($mut:tt)? $name:ident` - Process a single image with default options into a named variable with optional mutability
/// * `@with_options $options:expr, $image:expr=> $($mut:tt)? $name:ident` - Process a single image with custom options into a named variable with optional mutability
///
/// # Examples
///
/// ```rust,ignore
/// // Process a single image with default options
/// let result = process_images!(my_image);
///
/// // Process a single image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// let result = process_images!(my_image, @with_options options);
///
/// // Process multiple images with default options
/// let results = process_images!(image1, image2, image3);
/// // or
/// let results = process_images![image1, image2, image3];
///
/// // Process multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// let results = process_images!(image1, image2, image3, @with_options options);
///
/// // Process a vector of images
/// let image_vec = vec![image1, image2, image3];
/// let results = process_images!(@vec image_vec);
///
/// // Process a vector of images with custom options
/// let results = process_images!(@vec image_vec, @with_options options);
///
/// // Process a vector of images with custom options and map operation
/// let results = process_images!(@vec image_vec, @with_options options, @var img, @map {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
///     img
/// });
///
/// // Process a vector of images with default options and for_each operation
/// process_images!(@vec image_vec, @var img, @for_each {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
/// });
///
/// // Return an empty vector
/// let empty_results: Vec<ImageProcessorResult> = process_images!();
///
/// // Create an empty result vector with a name
/// process_images!{
///     @with_options options,
///     [] => @mut empty_result
/// }
///
/// // Process multiple images into a named variable
/// process_images! {
///     [image1, image2, image3] => mut my_results
/// }
///
/// // Process a single image into a named variable with custom options
/// process_images! {
///     @with_options options,
///     image1 => result
/// }
/// ```
///
/// # Returns
///
/// * For single image processing: `ConvertResult<ImageProcessorResult>`
/// * For multiple image processing: `Vec<ConvertResult<ImageProcessorResult>>`
/// * For empty invocation: `Vec<ConvertResult<ImageProcessorResult>>` (empty vector)
/// * For map operations: `Vec<T>` where T is the return type of the map block
macro_rules! process_images {
    () => {
        Vec::<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>::new()
    };
    ($(@with_options $options:expr,)?$([]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        $(let $($mut )?$name = $crate::process_images!();)*
    };
    (@with_options $options:expr,$([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@with_options $options:expr,$($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@vec $images:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images)}
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@map $block:block) => {
        $crate::__vec_process_images!($images, map, $var $(,options: $options)?, ty: Vec, collect: collect, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@map $block:block) => {
        $crate::__vec_process_images!($images, map, result $(,options: $options)?, ty: Vec, collect: collect, result: result, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, $var $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, result $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr,@with_options $options:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images, $options)}
    };
    ($image:expr) => {{
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, option)}
    };
    ($image:expr,@with_options $options:expr) => {{
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, $options)}
    };
    ($($image:expr),+,@with_options $options: expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let options = $options;
        let images = vec![$($image),+];
        _process_images(images, options)
    }};
    ($($image:expr),+$(,)?) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let images = vec![$(
            $image
        ),+];
        _process_images(images)
        }
    }
}

#[macro_export]
#[cfg(all(feature = "auto_select", feature = "processor"))]
/// A macro to process one or more images and return the processed results.
///
/// This macro provides flexible ways to process images with various options and
/// automatically handles parallel processing for better performance when dealing
/// with multiple images. It supports both single image processing and batch
/// processing with optional custom options.
///
/// For batch processing of more than 10 images, this macro automatically uses
/// parallel processing via the rayon crate to improve performance. For smaller
/// batches or single images, sequential processing is used.
///
/// # Arguments
///
/// * `()` - Returns an empty vector when called with no arguments
/// * `$image:expr` - Process a single image with default options
/// * `$image:expr, @with_options $options:expr` - Process a single image with custom options
/// * `$($image:expr),+` - Process multiple images with default options
/// * `$($image:expr),+, @with_options $options:expr` - Process multiple images with custom options
/// * `@vec $images:expr` - Process a vector of images with default options
/// * `@vec $images:expr, @with_options $options:expr` - Process a vector of images with custom options
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @map $block:block` - Process a vector of images with custom options and map operation
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with custom options and for_each operation
/// * `@vec $images:expr, @var $var:ident, @map $block:block` - Process a vector of images with default options and map operation
/// * `@vec $images:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with default options and for_each operation
/// * `@with_options $options:expr, []=> $($mut:tt)? $name:ident` - Create an empty result vector with a specified name and optional mutability
/// * `@with_options $options:expr, [$($image:expr),*]=> $($mut:tt)? $name:ident` - Process multiple images into a named variable with optional mutability
/// * `[$($image:expr),*]=> $($mut:tt)? $name:ident` - Process multiple images with default options into a named variable with optional mutability
/// * `$image:expr=> $($mut:tt)? $name:ident` - Process a single image with default options into a named variable with optional mutability
/// * `@with_options $options:expr, $image:expr=> $($mut:tt)? $name:ident` - Process a single image with custom options into a named variable with optional mutability
///
/// # Examples
///
/// ```rust,ignore
/// // Process a single image with default options
/// let result = process_images!(my_image);
///
/// // Process a single image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// let result = process_images!(my_image, @with_options options);
///
/// // Process multiple images with default options
/// let results = process_images!(image1, image2, image3);
/// // or
/// let results = process_images![image1, image2, image3];
///
/// // Process multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// let results = process_images!(image1, image2, image3, @with_options options);
///
/// // Process a vector of images
/// let image_vec = vec![image1, image2, image3];
/// let results = process_images!(@vec image_vec);
///
/// // Process a vector of images with custom options
/// let results = process_images!(@vec image_vec, @with_options options);
///
/// // Process a vector of images with custom options and map operation
/// let results = process_images!(@vec image_vec, @with_options options, @var img, @map {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
///     img
/// });
///
/// // Process a vector of images with default options and for_each operation
/// process_images!(@vec image_vec, @var img, @for_each {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
/// });
///
/// // Return an empty vector
/// let empty_results: Vec<ImageProcessorResult> = process_images!();
///
/// // Create an empty result vector with a name
/// process_images!{
///     @with_options options,
///     [] => @mut empty_result
/// }
///
/// // Process multiple images into a named variable
/// process_images! {
///     [image1, image2, image3] => mut my_results
/// }
///
/// // Process a single image into a named variable with custom options
/// process_images! {
///     @with_options options,
///     image1 => result
/// }
/// ```
///
/// # Returns
///
/// * For single image processing: `ConvertResult<ImageProcessorResult>`
/// * For multiple image processing: `Vec<ConvertResult<ImageProcessorResult>>`
/// * For empty invocation: `Vec<ConvertResult<ImageProcessorResult>>` (empty vector)
/// * For map operations: `Vec<T>` where T is the return type of the map block
macro_rules! process_images {
    () => {
        Vec::<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>::new()
    };
    ($(@with_options $options:expr,)?$([]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        $(let $($mut )?$name = $crate::process_images!();)*
    };
    (@with_options $options:expr,$([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@with_options $options:expr,$($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@vec $images:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images)}
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@map $block:block) => {
        $crate::__vec_process_images!($images, map, $var $(,options: $options)?, ty: Vec, collect: collect, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@map $block:block) => {
        $crate::__vec_process_images!($images, map, result $(,options: $options)?, ty: Vec, collect: collect, result: result, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, $var $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, result $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr,@with_options $options:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images, $options)}
    };
    ($image:expr) => {{
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, option)}
    };
    ($image:expr,@with_options $options:expr) => {{
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, $options)}
    };
    ($($image:expr),+,@with_options $options: expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let options = $options;
        let images = vec![$($image),+];
        _process_images(images, options)
    }};
    ($($image:expr),+$(,)?) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let images = vec![$(
            $image
        ),+];
        _process_images(images)
        }
    }
}
