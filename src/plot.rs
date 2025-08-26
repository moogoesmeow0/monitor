use image::{GenericImageView, ImageBuffer, ImageReader, Pixel, Rgba, RgbaImage};
use rayon::prelude::*;
use std::time::Instant;

use crate::math::flatten;
use crate::shared::SharedState;
use crate::util::{CAM_ANGLE, CAM_HEIGHT, DATA_PATH, Error, FOV, VIEW_HEIGHT, VIEW_WIDTH, read};

pub fn plot(state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
    let width = 1920;
    let height = 1080;

    let start = Instant::now();
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    println!("Image creation: {:?}", start.elapsed());

    let start = Instant::now();
    let points = read()?;
    println!("Reading data: {:?}", start.elapsed());

    let start = Instant::now();
    let world_coords = flatten(CAM_HEIGHT, CAM_ANGLE, VIEW_WIDTH, VIEW_HEIGHT, FOV, &points);
    println!("Flattening: {:?}", start.elapsed());

    let start = Instant::now();
    //bg((255, 255, 255, 255), &mut img);
    img_bg(&mut img, "room.png")?;

    println!("Background fill: {:?}", start.elapsed());

    grid(&mut img, 30, state)?;

    draw_points(&mut img, &world_coords)?;

    let start = Instant::now();
    save(&img, "output.png")?;
    println!("Saving: {:?}", start.elapsed());

    println!("done");

    Ok(())
}

pub fn bg(color: (u8, u8, u8, u8), img: &mut RgbaImage) {
    let rgba = Rgba([color.0, color.1, color.2, color.3]);
    img.par_pixels_mut().for_each(|pixel: &mut Rgba<u8>| {
        *pixel = rgba;
    });
}

pub fn img_bg(img: &mut RgbaImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bg_img = ImageReader::open(path)?.decode()?;

    if bg_img.width() != img.width() || bg_img.height() != img.height() {
        return Err(Box::new(Error::ImageSizeError(format!(
            "Background image size does not match: expected {}x{}, got {}x{}",
            img.width(),
            img.height(),
            bg_img.width(),
            bg_img.height()
        ))));
    }

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let bg_pixel = bg_img.get_pixel(x, y);
        *pixel = Rgba([bg_pixel[0], bg_pixel[1], bg_pixel[2], bg_pixel[3]]);
    });

    Ok(())
}

pub fn save(img: &RgbaImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    img.save(path)?;
    Ok(())
}

pub fn grid(
    img: &mut RgbaImage,
    cell_size: u32,
    state: SharedState,
) -> Result<(), Box<dyn std::error::Error>> {

    let width = img.width();
    let height = img.height();

    for x in (0..width).step_by(cell_size as usize) {
        for y in 0..height {
            img.put_pixel(x, y, Rgba([0, 255, 0, 255])); // green vertical lines
        }
    }

    for y in (0..height).step_by(cell_size as usize) {
        for x in 0..width {
            img.put_pixel(x, y, Rgba([0, 255, 0, 255])); // green horizontal lines
        }
    }

    Ok(())
}

pub fn draw_points(
    img: &mut RgbaImage,
    points: &Vec<(f64, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = img.width();
    let height = img.height();

    for &(x, y) in points {
        // Camera is now at center bottom of image
        let pixel_x = ((x + 1.0) * (width as f64 / 2.0)) as u32;
        let pixel_y = (height as f64 - (y * (height as f64 / 2.0))) as u32;

        if pixel_x < width && pixel_y < height {
            img.put_pixel(pixel_x, pixel_y, Rgba([255, 0, 0, 255])); 
            //circle(img, 5.0, (pixel_x, pixel_y), (255, 0, 0, 255)); 
        }
    }

    Ok(())
}

fn circle(img: &mut RgbaImage, radius: f32, center: (u32, u32), color: (u8, u8, u8, u8)) {
    let rgba = Rgba([color.0, color.1, color.2, color.3]);
    let (cx, cy) = center;

    for y in 0..img.height() {
        for x in 0..img.width() {
            let dx = x as f32 - cx as f32;
            let dy = y as f32 - cy as f32;
            if (dx * dx + dy * dy) <= (radius * radius) {
                img.put_pixel(x, y, rgba);
            }
        }
    }
}
