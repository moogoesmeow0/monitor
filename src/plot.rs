use image::{ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;
use std::time::Instant;

use crate::math::flatten;
use crate::util::{CAM_ANGLE, CAM_HEIGHT, DATA_PATH, FOV, VIEW_HEIGHT, VIEW_WIDTH, read};

pub fn plot() -> Result<(), Box<dyn std::error::Error>> {
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
    bg((255, 255, 255, 255), &mut img);
    println!("Background fill: {:?}", start.elapsed());

    grid(&mut img, width, height, 30)?;

    draw_points(&mut img, &world_coords, width, height)?;

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


pub fn save(img: &RgbaImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    img.save(path)?;
    Ok(())
}

pub fn grid(
    img: &mut RgbaImage,
    width: u32,
    height: u32,
    cell_size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
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
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    for &(x, y) in points {
        let pixel_x = ((x + 1.0) * (width as f64 / 2.0)) as u32;
        let pixel_y = ((1.0 - y) * (height as f64 / 2.0)) as u32;

        if pixel_x < width && pixel_y < height {
            img.put_pixel(pixel_x, pixel_y, Rgba([255, 0, 0, 255])); // red points
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