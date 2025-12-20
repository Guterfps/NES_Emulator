use super::Reg;

pub struct LinearFeedback {
    data: u8,
    changed: bool,
}

pub const NOISE_MODE_MASK: u8 = 0b1000_0000;
pub const NOISE_PERIOD_MASK: u8 = 0b0000_1111;

impl LinearFeedback {
    pub fn new() -> Self {
        Self {
            data: 0,
            changed: false,
        }
    }

    pub fn take_changed(&mut self) -> Option<u8> {
        if self.changed {
            self.changed = false;
            Some(self.data)
        } else {
            None
        }
    }

    pub fn get_mode(&self) -> bool {
        (self.data & NOISE_MODE_MASK) != 0
    }

    pub fn get_period_index(&self) -> usize {
        (self.data & NOISE_PERIOD_MASK) as usize
    }
}

impl Reg for LinearFeedback {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }

    fn write(&mut self, mask: u8, val: u8) {
        self.data = (self.data & !mask) | (val & mask);
        self.changed = true;
    }
}
