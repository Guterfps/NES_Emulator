use super::Reg;

pub struct Frequency {
    data: u8,
    changed: bool,
}

pub const IRQ_ENABLE_MASK: u8 = 0b1000_0000;
pub const LOOP_MASK: u8 = 0b0100_0000;
pub const FREQUENCY: u8 = 0b0000_1111;

impl Frequency {
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
}

impl Reg for Frequency {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }

    fn write(&mut self, _mask: u8, val: u8) {
        self.data = val;
        self.changed = true;
    }
}
