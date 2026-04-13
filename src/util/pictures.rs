use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{GenericImageView, ImageReader};
use viuer::Config as ViuerConfig;

use super::constants::LOGO_LENGTH;
use crate::error::FetchInfoError;

const MAX_PICTURE_SIZE: f64 = 44.0;

pub fn display(path: impl AsRef<Path>) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => FetchInfoError::error_exit(format!(
            "An error occurred while reading the image: {error}"
        )),
    };
    let image_reader = match ImageReader::new(BufReader::new(file)).with_guessed_format() {
        Ok(r) => r,
        Err(error) => FetchInfoError::error_exit(format!(
            "An error occurred while guessing the image format: {error}"
        )),
    };
    let image = match image_reader.decode() {
        Ok(i) => i,
        Err(error) => FetchInfoError::error_exit(format!(
            "An error occurred while decoding the image: {error}"
        )),
    };

    let (width, height) = image.dimensions();
    let (width, height) = (f64::from(width), f64::from(height));

    let ratio = (width / MAX_PICTURE_SIZE)
        .max(height / MAX_PICTURE_SIZE)
        .max(1.0);
    let new_width = (width / ratio) as u32;

    let config: ViuerConfig = ViuerConfig {
        x: ((LOGO_LENGTH as u32 - new_width) / 2) as u16,
        width: Some(new_width),
        absolute_offset: false,
        ..ViuerConfig::default()
    };
    if let Err(error) = viuer::print(&image, &config) {
        FetchInfoError::error_exit(format!(
            "An error occurred while printing the image: {error}",
        ))
    }
    println!();
}
