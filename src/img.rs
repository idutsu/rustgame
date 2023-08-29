use image::DynamicImage;
use std::fmt;

pub struct Image {
    pub filename: String,
}

#[derive(Debug)]
pub enum ImageError {
    LoadError(image::ImageError),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageError::LoadError(err) => write!(f, "Failed to load image: {}", err),
        }
    }
}

impl Image {
    pub fn new(&self) -> Result<DynamicImage, ImageError> {
        let assets_dir = String::from("./assets/");
        let path = assets_dir + &self.filename;
        image::open(&path).map_err(|err| ImageError::LoadError(err))
    }
}