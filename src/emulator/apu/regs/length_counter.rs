use super::Reg;

pub struct LengthCounter {
    data: u8,
    pub counter: u8,
    pub enabled: bool,
    pub halt: bool,
}

pub const LENGTH_LOAD_MASK: u8 = 0b1111_1000;
pub const TIMER_HIGH_MASK: u8 = 0b0000_0111;

const LENGTH_LOAD_OFFSET: usize = 3;
const LENGTH_TABLE_SIZE: usize = 32;

#[rustfmt::skip]
const LENGTH_TABLE: [u8; LENGTH_TABLE_SIZE] = [
    10,254, 20,  2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
    12, 16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
];

impl LengthCounter {
    pub fn new() -> Self {
        Self {
            data: 0,
            counter: 0,
            enabled: true,
            halt: false,
        }
    }

    pub fn tick(&mut self) {
        if !self.halt && (self.counter > 0) {
            self.counter -= 1;
        }
    }

    pub fn is_active(&self) -> bool {
        self.counter > 0
    }

    fn update_internal_state(&mut self) {
        if self.enabled {
            let index = (self.data & LENGTH_LOAD_MASK) >> LENGTH_LOAD_OFFSET;
            self.counter = LENGTH_TABLE[index as usize];
        }
    }
}

impl Reg for LengthCounter {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }

    fn write(&mut self, mask: u8, val: u8) {
        self.data = (self.data & !mask) | (val & mask);

        self.update_internal_state();
    }
}
