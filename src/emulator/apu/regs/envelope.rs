use super::Reg;

pub struct Envelope {
    data: u8,
    start_flag: bool,
    decay_level: u8,
    divider: u8,
}

pub const DUTY_MASK: u8 = 0b1100_0000;
pub const ENVELOPE_COUNTER_HALT_MASK: u8 = 0b0010_0000;
pub const CONSTANST_VOLUME_MASK: u8 = 0b0001_0000;
pub const VOLUME_MASK: u8 = 0b000_1111;

const DECAY_LEVEL_LOAD: u8 = 15;

impl Envelope {
    pub fn new() -> Self {
        Self {
            data: 0,
            start_flag: false,
            decay_level: 0,
            divider: 0,
        }
    }

    pub fn restart(&mut self) {
        self.start_flag = true;
    }

    pub fn output_volume(&self) -> u8 {
        if (self.data & CONSTANST_VOLUME_MASK) != 0 {
            self.data & VOLUME_MASK
        } else {
            self.decay_level
        }
    }

    pub fn tick(&mut self) {
        if self.start_flag {
            self.start_flag = false;
            self.decay_level = DECAY_LEVEL_LOAD;
            self.divider = self.data & VOLUME_MASK;
        } else if self.divider > 0 {
            self.divider -= 1;
        } else {
            self.divider = self.data & VOLUME_MASK;

            if self.decay_level > 0 {
                self.decay_level -= 1;
            } else if (self.data & ENVELOPE_COUNTER_HALT_MASK) != 0 {
                self.decay_level = DECAY_LEVEL_LOAD;
            }
        }
    }
}

impl Reg for Envelope {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
