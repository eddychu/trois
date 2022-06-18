use crate::vector4::Vector4;
use image;

#[derive(Debug)]
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
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
            pixel[3] as f32 / 255.0,
        )
    }

    pub fn get_pixel_from_uv(&self, u: f32, v: f32) -> Vector4 {
        // wrap around the uv coordinates
        // let u = u.fract();
        // let v = v.fract() * self.height as f32;
        let u = u.min(1.0).max(0.0);
        let v = v.min(1.0).max(0.0);
        let x = (u * (self.width - 1) as f32) as u32;
        let y = (v * (self.height - 1) as f32) as u32;
        self.get_pixel(x, y)
    }
}
