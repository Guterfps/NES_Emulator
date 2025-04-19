use super::{MIRRORS_ADDR, control_reg::AddressInc};

pub struct AddressReg {
    value: (u8, u8),
    latch: bool,
}

const MIRRORS_MASK: u16 = 0b1111_1111_1111_1111;

impl AddressReg {
    pub fn new() -> Self {
        AddressReg {
            value: (0, 0),
            latch: (false),
        }
    }

    fn set(&mut self, data: u16) {
        [self.value.0, self.value.1] = data.to_be_bytes()
    }

    pub fn get(&self) -> u16 {
        u16::from_be_bytes([self.value.0, self.value.1])
    }

    pub fn update(&mut self, data: u8) {
        if !self.latch {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        self.mirror_down_addr();
        self.toggle_latch();
    }

    pub fn increment(&mut self, inc: AddressInc) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc as u8);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }

        self.mirror_down_addr();
    }

    fn mirror_down_addr(&mut self) {
        let value = self.get();
        if value >= MIRRORS_ADDR {
            self.set(value & MIRRORS_MASK);
        }
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }

    pub fn toggle_latch(&mut self) {
        self.latch = !self.latch;
    }

    pub fn get_latch(&self) -> bool {
        self.latch
    }
}
