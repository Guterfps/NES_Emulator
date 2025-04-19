pub struct ScrollReg {
    x: u8,
    y: u8,
}

pub enum Mode {
    X,
    Y,
}

impl ScrollReg {
    pub fn new() -> Self {
        ScrollReg { x: 0, y: 0 }
    }

    pub fn update(&mut self, mode: Mode, value: u8) {
        match mode {
            Mode::X => self.x = value,
            Mode::Y => self.y = value,
        }
    }
}
