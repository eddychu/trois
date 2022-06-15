use crate::vector3::Vector3;
use crate::vector4::Vector4;
use image::{save_buffer, ColorType};
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub channels: u32,
    pub pixels: Vec<u8>,
    pub depths: Vec<f64>,
}

impl Canvas {
    pub fn new(width: u32, height: u32, channels: u32) -> Canvas {
        Canvas {
            width: width,
            height: height,
            channels: channels,
            pixels: vec![0; (width * height * channels) as usize],
            depths: vec![1.0; (width * height) as usize],
        }
    }

    pub fn set_depth(&mut self, x: u32, y: u32, depth: f64) {
        let index = (y * self.width + x) as usize;
        self.depths[index] = depth;
    }

    pub fn get_depth(&self, x: u32, y: u32) -> f64 {
        self.depths[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Vector4) {
        let index = (y * self.width + x) * self.channels;
        self.pixels[index as usize] = (color.x * 255.0) as u8;
        self.pixels[index as usize + 1] = (color.y * 255.0) as u8;
        self.pixels[index as usize + 2] = (color.z * 255.0) as u8;
        self.pixels[index as usize + 3] = (color.w * 255.0) as u8;
    }

    pub fn set_pixel_rgb(&mut self, x: u32, y: u32, color: Vector3) {
        let index = (y * self.width + x) * self.channels;
        self.pixels[index as usize] = (color.x * 255.0) as u8;
        self.pixels[index as usize + 1] = (color.y * 255.0) as u8;
        self.pixels[index as usize + 2] = (color.z * 255.0) as u8;
        self.pixels[index as usize + 3] = (1.0 * 255.0) as u8;
    }

    pub fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: Vector3) {
        let ipart = |x: f64| x.floor() as u32;
        let round = |x: f64| x.round() as u32;
        let fpart = |x: f64| x - ipart(x) as f64;
        let rfpart = |x: f64| 1.0 - fpart(x);
        let diff_y = (y1 as f64 - y0 as f64).abs();
        let diff_x = (x1 as f64 - x0 as f64).abs();
        let steep = diff_y > diff_x;
        let mut x0 = x0;
        let mut y0 = y0;
        let mut x1 = x1;
        let mut y1 = y1;

        if steep {
            (x0, y0) = (y0, x0);
            (x1, y1) = (y1, x1);
        }
        if x0 > x1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let mut gradient = 0.0f64;
        if dx == 0.0 {
            gradient = 1.0;
        } else {
            gradient = dy as f64 / dx as f64;
        }
        let mut x_end = round(x0);
        let mut y_end = y0 + gradient * (x_end as f64 - x0);
        let mut x_gap = rfpart(x0 + 0.5);
        let x_pxl1 = x_end;
        let y_pxl1 = ipart(y_end);
        if steep {
            self.set_pixel_rgb(y_pxl1, x_pxl1, color.clone() * x_gap * rfpart(y_end));
            self.set_pixel_rgb(y_pxl1 + 1, x_pxl1, color.clone() * x_gap * fpart(y_end));
        } else {
            self.set_pixel_rgb(x_pxl1, y_pxl1, color.clone() * x_gap * rfpart(y_end));
            self.set_pixel_rgb(x_pxl1, y_pxl1 + 1, color.clone() * x_gap * fpart(y_end));
        }

        let mut intery = y_end + gradient;
        x_end = round(x1);
        y_end = y1 + gradient * (x_end as f64 - x1);
        x_gap = fpart(x1 + 0.5);
        let x_pxl2 = x_end;
        let y_pxl2 = ipart(y_end);
        if steep {
            self.set_pixel_rgb(y_pxl2, x_pxl2, color.clone() * x_gap * rfpart(y_end));
            self.set_pixel_rgb(y_pxl2 + 1, x_pxl2, color.clone() * x_gap * fpart(y_end))
        } else {
            self.set_pixel_rgb(x_pxl2, y_pxl2, color.clone() * x_gap * rfpart(y_end));
            self.set_pixel_rgb(x_pxl2, y_pxl2 + 1, color.clone() * x_gap * fpart(y_end))
        }

        if steep {
            for x in (x_pxl1 + 1)..x_pxl2 {
                self.set_pixel_rgb(ipart(intery), x, color.clone() * rfpart(intery));
                self.set_pixel_rgb(ipart(intery) + 1, x, color.clone() * fpart(intery));
                intery += gradient;
            }
        } else {
            for x in (x_pxl1 + 1)..x_pxl2 {
                self.set_pixel_rgb(x, ipart(intery), color.clone() * rfpart(intery));
                self.set_pixel_rgb(x, ipart(intery) + 1, color.clone() * fpart(intery));
                intery += gradient;
            }
        }
    }

    pub fn save_image(&self, filename: &str) {
        save_buffer(
            filename,
            &self.pixels,
            self.width,
            self.height,
            ColorType::Rgba8,
        )
        .unwrap();
    }
}
