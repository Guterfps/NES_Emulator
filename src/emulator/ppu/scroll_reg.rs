pub struct ScrollReg {
    x: u8,
    y: u8,
    latch: bool,
}

impl ScrollReg {
    pub fn new() -> Self {
        ScrollReg {
            x: 0,
            y: 0,
            latch: false,
        }
    }

    pub fn write(&mut self, value: u8) {
        if !self.latch {
            self.x = value;
        } else {
            self.y = value;
        }

        self.togle_latch();
    }

    fn togle_latch(&mut self) {
        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }

    pub fn scroll_x(&self) -> u8 {
        self.x
    }

    pub fn scroll_y(&self) -> u8 {
        self.y
    }
}
