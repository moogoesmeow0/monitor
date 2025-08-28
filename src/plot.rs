use chrono::{DateTime, Utc};
use image::{GenericImageView, ImageBuffer, ImageReader, Pixel, Rgba, RgbaImage};
use rayon::prelude::*;
use std::time::Instant;

use crate::math::{flatten, normalize};
use crate::shared::SharedState;
use crate::util::{constants, Error, read};

pub fn plot(state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
    let width = 1920;
    let height = 1080;

    let start = Instant::now();
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    println!("Image creation: {:?}", start.elapsed());

    let start = Instant::now();
    let points = if let Ok(data) = state.read() {
        data.points.clone()
    } else {
        return Err(Box::new(Error::StateGuardError));
    };

    // let normalized_points: Vec<(f64, f64)> = normalize(&points);

    let start = Instant::now();
    //bg((255, 255, 255, 255), &mut img);
    img_bg(&mut img, "room.png")?;

    println!("Background fill: {:?}", start.elapsed());

    grid_with_heatmap(&mut img, 30, state)?;

    draw_points(&mut img, &points)?;

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
            img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
        }
    }

    for y in (0..height).step_by(cell_size as usize) {
        for x in 0..width {
            img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
        }
    }

    Ok(())
}

pub fn grid_with_heatmap(
    img: &mut RgbaImage,
    cell_size: u32,
    state: SharedState,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = img.width();
    let height = img.height();

    let data = {
        if let Ok(data) = state.read() {
            data
        } else {
            return Err(Box::new(crate::util::Error::StateGuardError));
        }
    };
    let points = &data.points;

    let grid_width = width.div_ceil(cell_size);
    let grid_height = height.div_ceil(cell_size);
    let mut heat_grid = vec![vec![0u32; grid_height as usize]; grid_width as usize];

    // count le points
    for &(world_x, world_y, _) in points {
        // Convert world coordinates to pixel coordinates, assuming camera at center bottom
        let px = ((world_x + 1.0) * (width as f64 / 2.0));
        let py = (height as f64 - (world_y * (height as f64 / 2.0)));

        if px >= 0.0 && py >= 0.0 && (px as u32) < width && (py as u32) < height {
            let grid_x = (px as u32) / cell_size;
            let grid_y = (py as u32) / cell_size;
            if (grid_x as usize) < heat_grid.len() && (grid_y as usize) < heat_grid[0].len() {
                heat_grid[grid_x as usize][grid_y as usize] += 1;
            }
        }
    }

    // find le max count
    let max_count = heat_grid
        .iter()
        .flat_map(|row| row.iter())
        .max()
        .copied()
        .unwrap_or(1);

    // draw le heatmap
    for grid_x in 0..grid_width {
        for grid_y in 0..grid_height {
            let count = heat_grid[grid_x as usize][grid_y as usize];
            if count > 0 {
                let intensity = (count as f32 / max_count as f32 * 255.0) as u8;
                let heat_color = Rgba([intensity, 0, 255 - intensity, 128]); // Red-blue gradient with transparency

                // Fill the cell with heat color
                let start_x = grid_x * cell_size;
                let start_y = grid_y * cell_size;
                let end_x = std::cmp::min(start_x + cell_size, width);
                let end_y = std::cmp::min(start_y + cell_size, height);

                for x in start_x..end_x {
                    for y in start_y..end_y {
                        let existing = img.get_pixel(x, y);
                        // Blend the heat color with existing pixel
                        let blended = Rgba([
                            ((existing[0] as u16 + heat_color[0] as u16) / 2) as u8,
                            ((existing[1] as u16 + heat_color[1] as u16) / 2) as u8,
                            ((existing[2] as u16 + heat_color[2] as u16) / 2) as u8,
                            255,
                        ]);
                        img.put_pixel(x, y, blended);
                    }
                }
            }
        }
    }

    for x in (0..width).step_by(cell_size as usize) {
        for y in 0..height {
            img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
        }
    }

    for y in (0..height).step_by(cell_size as usize) {
        for x in 0..width {
            img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
        }
    }

    Ok(())
}

pub fn draw_points(
    img: &mut RgbaImage,
    points: &Vec<(f64, f64, Option<DateTime<Utc>>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = img.width();
    let height = img.height();

    for &(x, y, _) in points {
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
