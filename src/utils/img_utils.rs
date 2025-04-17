use std::io::Cursor;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba};

pub fn is_in_rounded_rect(x: u32, y: u32, width: u32, height: u32, radius: f32) -> bool {
    let x = x as f32;
    let y = y as f32;
    let width = width as f32;
    let height = height as f32;
    
    if x < radius && y < radius {
        let dx = x - radius;
        let dy = y - radius;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        return distance <= radius;
    } else if x > width - radius && y < radius {
        let dx = x - (width - radius);
        let dy = y - radius;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        return distance <= radius;
    } else if x < radius && y > height - radius {
        let dx = x - radius;
        let dy = y - (height - radius);
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        return distance <= radius;
    } else if x > width - radius && y > height - radius {
        let dx = x - (width - radius);
        let dy = y - (height - radius);
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        return distance <= radius;
    }
    
    x >= 0.0 && x < width && y >= 0.0 && y < height
}

pub fn round_image(img_data: Cursor<std::borrow::Cow<'static, [u8]>>) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = ImageReader::new(img_data)
    .with_guessed_format()
    .unwrap()
    .decode()
    .unwrap();
    
    let (width, height) = img.dimensions();
    
    let mut rounded_img = ImageBuffer::new(width, height);
    
    let radius = 50.0;
    
    for (x, y, pixel) in img.pixels() {
        if is_in_rounded_rect(x, y, width, height, radius) {
            rounded_img.put_pixel(x, y, Rgba(pixel.0));
        } else {
            rounded_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    Ok(DynamicImage::ImageRgba8(rounded_img))
}