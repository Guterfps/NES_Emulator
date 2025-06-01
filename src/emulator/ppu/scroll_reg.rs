pub struct ScrollReg {
    x: u8,
    y: u8,
}

impl ScrollReg {
    pub fn new() -> Self {
        ScrollReg { x: 0, y: 0 }
    }

    pub fn write(&mut self, value: u8, latch: bool) {
        if !latch {
            self.x = value;
        } else {
            self.y = value;
        }
    }

    pub fn scroll_x(&self) -> u8 {
        self.x
    }

    pub fn scroll_y(&self) -> u8 {
        self.y
    }
}
