use image as img;
use image::Pixel;
use image::{ImageBuffer, RgbaImage};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let load_start = Instant::now();
    const NEW_FRAME_MIN: u64 = 4;

    // const BUNDLED_IMG_1_BYTES: &[u8] = include_bytes!("./assets/img1.png");
    // let bundled_img_1 = img::load_from_memory(BUNDLED_IMG_1_BYTES).unwrap().into_rgb8();

    let bundled_img_1 = image::io::Reader::open("./assets/img1.png").unwrap().decode().unwrap().to_rgb8();

    // const BUNDLED_IMG_2_BYTES: &[u8] = include_bytes!("./assets/img2.png");
    // let bundled_img_2 = img::load_from_memory(BUNDLED_IMG_2_BYTES).unwrap().into_rgb8();

    let bundled_img_2 = image::io::Reader::open("./assets/img2.png").unwrap().decode().unwrap().to_rgb8();
    let load_duration = load_start.elapsed().as_millis();

    let (width, height) = bundled_img_1.dimensions();
    let mut out: RgbaImage = ImageBuffer::new(width, height);

    let img_start = Instant::now();
    for pixel in out.enumerate_pixels_mut() {
        let img1_first_pixel = pixel.2.to_rgb();
        // let img1_pixel_rgb = img1_first_pixel.to_rgb();

        let img2_first_pixel = bundled_img_2.get_pixel(pixel.0, pixel.1).to_rgb();
        // let img2_pixel_rgb = img2_first_pixel.to_rgb();

        *pixel.2 = image::Rgba([
            underflow_sub(img1_first_pixel[0],img2_first_pixel[0]),
            underflow_sub(img1_first_pixel[1],img2_first_pixel[1]),
            underflow_sub(img1_first_pixel[2],img2_first_pixel[2]),
            255]);
    }

    let mut new_pixels = Vec::new();

    for pixel in out.enumerate_pixels(){
        let rgba_pix = pixel.2.0;
        new_pixels.push(rgba_pix[0] as u64 + rgba_pix[1] as u64 + rgba_pix[2] as u64);
    }

    let sum_from_pix: u64 = new_pixels.iter().sum();
    let img_duration = img_start.elapsed().as_millis();

    if sum_from_pix > NEW_FRAME_MIN {
        println!("Not a new frame!")
    } else {
        println!("A new frame!")
    }

    println!("Took {}ms to process frame\nTook {}ms to load images", img_duration, load_duration);

    // out.save("out.png").unwrap();
}

fn underflow_sub(num1: u8, num2: u8) -> u8 {
    let (outcome, flowing) = num1.overflowing_sub(num2);
    return if flowing == true {
        0
    } else {
        outcome
    }
}