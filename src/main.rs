use std::env;

use image::{Rgb, RgbImage};

const R: u8 = 169;
const G: u8 = 169;
const B: u8 = 169;
const PROCESS_FAILED: i32 = -1;
const PNG_MODE: i32 = 0;
const JPEG_MODE: i32 = 1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let process_status: i32 = check_args(args.clone());

    if process_status == PROCESS_FAILED {
        return;
    }

    let w: u32;
    let h: u32;

    if args.len() < 4 {
        if args[1].clone().parse::<i32>().is_err() || args[2].clone().parse::<i32>().is_err() {
            println!("Usage: test-image-creater -j|-p width height");
            return;
        }

        w = args[1].parse::<u32>().unwrap();
        h = args[2].parse::<u32>().unwrap();
    } else {
        if args[2].clone().parse::<i32>().is_err() || args[3].clone().parse::<i32>().is_err() {
            println!("Usage: test-image-creater -j|-p width height");
            return;
        }

        w = args[2].parse::<u32>().unwrap();
        h = args[3].parse::<u32>().unwrap();
    }

    if image_create(process_status, w, h) {
        if process_status == PNG_MODE {
            println!("{}x{}.png create success!", w, h);
        } else {
            println!("{}x{}.jpg create success!", w, h);
        }
    } else {
        println!("image create failed")
    }
}

fn check_args(args: Vec<String>) -> i32 {
    if args.len() < 3 {
        println!("Usage: test-image-creater -j|-p width height");
        return PROCESS_FAILED;
    }

    let option = &args[0];

    match option.as_str() {
        "-j" => JPEG_MODE,
        "-p" => PNG_MODE,
        _ => PNG_MODE,
    }
}

fn image_create(mode: i32, width: u32, height: u32) -> bool {
    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, Rgb([R, G, B]));
        }
    }

    let mut image_name: String = "".to_string();

    if mode == PNG_MODE {
        image_name = format!("{}x{}.png", width, height);
    } else if mode == JPEG_MODE {
        image_name = format!("{}x{}.jpg", width, height);
    }

    if img.save(image_name).is_ok() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, process::Command};

    use super::*;

    #[test]
    fn test_check_args() {
        let args1: Vec<String> = vec!["-j".to_string(), "800".to_string(), "600".to_string()];
        assert_eq!(check_args(args1), JPEG_MODE);

        let args2: Vec<String> = vec!["-p".to_string(), "1024".to_string(), "768".to_string()];
        assert_eq!(check_args(args2), PNG_MODE);

        let args3: Vec<String> = vec!["-p".to_string()];
        assert_eq!(check_args(args3), PROCESS_FAILED);

        let args4: Vec<String> = vec!["-x".to_string(), "800".to_string(), "600".to_string()];
        assert_eq!(check_args(args4), PNG_MODE);

        let args5: Vec<String> = vec!["-x".to_string(), "test".to_string(), "600".to_string()];
        assert_eq!(check_args(args5), PNG_MODE);

        let args6: Vec<String> = vec!["-x".to_string(), "800".to_string(), "test".to_string()];
        assert_eq!(check_args(args6), PNG_MODE);

        let args7: Vec<String> = vec!["-x".to_string(), "test".to_string(), "test".to_string()];
        assert_eq!(check_args(args7), PNG_MODE);
    }

    #[test]
    fn test_image_create_png() {
        let width = 800;
        let height = 600;
        let result = image_create(PNG_MODE, width, height);
        assert!(result);

        let image_name = format!("{}x{}.png", width, height);
        let path = Path::new(&image_name);
        assert!(path.exists());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_image_create_jpeg() {
        let width = 1024;
        let height = 768;
        let result = image_create(JPEG_MODE, width, height);
        assert!(result);

        let image_name = format!("{}x{}.jpg", width, height);
        let path = Path::new(&image_name);
        assert!(path.exists());

        std::fs::remove_file(path).unwrap();
    }
}
