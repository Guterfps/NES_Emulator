pub struct Frame {
    pub data: Vec<u8>,
}

impl Frame {
    const WIDTH: usize = 256 * 2;
    const HIGHT: usize = 240;
    const NUM_OF_PIXELS: usize = Frame::WIDTH * Frame::HIGHT;
    const PIXEL_SIZE: usize = 3;

    pub fn new() -> Self {
        Frame {
            data: vec![0; Frame::NUM_OF_PIXELS * Frame::PIXEL_SIZE],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let base = y * Frame::PIXEL_SIZE * Frame::WIDTH + x * Frame::PIXEL_SIZE;

        if (base + 2) < self.data.len() {
            let (r, g, b) = rgb;
            self.data[base] = r;
            self.data[base + 1] = g;
            self.data[base + 2] = b;
        }
    }
}
