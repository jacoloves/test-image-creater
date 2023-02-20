use std::env;

use image::{Rgb,RgbImage};

const R:u8 = 169;
const G:u8 = 169;
const B:u8 = 169;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: test-image-creater width heigt");
        return;
    }

    let w: u32 = args[1].parse::<u32>().unwrap();
    let h: u32 = args[2].parse::<u32>().unwrap();

    let mut img = RgbImage::new(w, h);
    for x in 0..w {
        for y in 0..h {
            img.put_pixel(x, y, Rgb([R,G,B]));
        }
    }

    let image_name = format!("{}x{}.png",w,h);
    img.save(image_name).unwrap();
}
