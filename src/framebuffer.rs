pub struct FrameBuffer {
    width: u32,
    height: u32,
    colors: Vec<u32>,
    depth: Vec<f32>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        FrameBuffer {
            width: width,
            height: height,
            colors: vec![0xFF000000; (width * height) as usize],
            depth: vec![1.0; (width * height) as usize],
        }
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: u32) {
        let index = (y * self.width + x) as usize;
        self.colors[index] = color;
    }

    pub fn get_color(&self, x: u32, y: u32) -> u32 {
        self.colors[(y * self.width + x) as usize]
    }

    pub fn clear(&mut self, color: u32) {
        for i in 0..self.colors.len() {
            self.colors[i] = color;
        }
        for i in 0..self.depth.len() {
            self.depth[i] = 1.0;
        }
    }

    pub fn get_depth(&self, x: u32, y: u32) -> f32 {
        self.depth[(y * self.width + x) as usize]
    }

    pub fn set_depth(&mut self, x: u32, y: u32, depth: f32) {
        self.depth[(y * self.width + x) as usize] = depth;
    }

    pub fn get_colors(&self) -> &[u32] {
        &self.colors
    }
}
