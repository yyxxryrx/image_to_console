use image_to_console_core::{ResizeMode, AutoResizeOption, CustomResizeOption};

#[test]
fn test_resize_mode_default() {
    let default_mode = ResizeMode::default();
    match default_mode {
        ResizeMode::Auto(option) => {
            assert!(option.width);
            assert!(option.height);
        }
        _ => panic!("Expected Auto resize mode as default"),
    }
}

#[test]
fn test_resize_mode_creation() {
    let auto_mode = ResizeMode::Auto(AutoResizeOption {
        width: false,
        height: true,
    });
    
    match auto_mode {
        ResizeMode::Auto(option) => {
            assert!(!option.width);
            assert!(option.height);
        }
        _ => panic!("Expected Auto resize mode"),
    }
    
    let custom_mode = ResizeMode::Custom(CustomResizeOption {
        width: Some(100),
        height: None,
    });
    
    match custom_mode {
        ResizeMode::Custom(option) => {
            assert_eq!(option.width, Some(100));
            assert_eq!(option.height, None);
        }
        _ => panic!("Expected Custom resize mode"),
    }
    
    let none_mode = ResizeMode::None;
    match none_mode {
        ResizeMode::None => (),
        _ => panic!("Expected None resize mode"),
    }
}