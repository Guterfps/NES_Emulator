use super::Reg;

pub struct LinearCounter {
    data: u8,
    pub counter: u8,
    pub reload_flag: bool,
}

pub const CONTROL_MASK: u8 = 0b1000_0000;
pub const RELOAD_VALUE_MASK: u8 = 0b0111_1111;

impl LinearCounter {
    pub fn new() -> Self {
        Self {
            data: 0,
            counter: 0,
            reload_flag: false,
        }
    }

    pub fn tick(&mut self) {
        if self.reload_flag {
            self.counter = self.data & RELOAD_VALUE_MASK;
        } else if self.counter > 0 {
            self.counter -= 1;
        }

        if (self.data & CONTROL_MASK) == 0 {
            self.reload_flag = false;
        }
    }

    pub fn set_reload(&mut self) {
        self.reload_flag = true;
    }
}

impl Reg for LinearCounter {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }

    fn write(&mut self, mask: u8, val: u8) {
        self.data = (self.data & !mask) | (val & mask);
        self.set_reload();
    }
}
