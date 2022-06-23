use crate::{vector4::Vector4, vector2::Vector2};
use image;
use crate::math::srgb_to_linear;



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
            srgb_to_linear(pixel[0] as f32 / 255.0),
            srgb_to_linear(pixel[1] as f32 / 255.0),
            srgb_to_linear(pixel[2] as f32 / 255.0),
            srgb_to_linear(pixel[3] as f32 / 255.0),
        )
    }

    pub fn sample(&self, uv: Vector2) -> Vector4 {
        let u = uv.x;
        let v = uv.y;
        let u = u - u.floor();
        let v = v - v.floor();
        let c = (u * (self.width - 1) as f32) as u32;
        let r = (v * (self.height - 1) as f32) as u32;
        self.get_pixel(c, r)
    }


}

impl Clone for Texture {
    fn clone(&self) -> Texture {
        Texture::new(self.image.clone(), self.width, self.height)
    }
}
