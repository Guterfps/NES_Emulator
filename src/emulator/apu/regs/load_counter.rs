use super::Reg;

pub struct LoadCounter {
    data: u8,
    changed: bool,
}

pub const LOAD_COUNTER_MASK: u8 = 0b0111_1111;

impl LoadCounter {
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

impl Reg for LoadCounter {
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
