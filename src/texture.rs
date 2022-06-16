use crate::vector4::Vector4;
use image;
pub struct Texture {
    pub image: image::RgbaImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(image: image::RgbaImage, width: u32, height: u32) -> Texture {
        Texture {
            image,
            width,
            height,
        }
    }

    pub fn load(path: &str) -> Texture {
        let image = image::open(path).unwrap();
        let width = image.width();
        let height = image.height();
        Texture::new(image.into_rgba8(), width, height)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Vector4 {
        let pixel = self.image.get_pixel(x, y);
        Vector4::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
            pixel[3] as f64 / 255.0,
        )
    }
}
