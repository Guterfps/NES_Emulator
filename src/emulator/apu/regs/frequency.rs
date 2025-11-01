use super::Reg;

pub struct Frequency {
    data: u8,
}

pub const IRQ_ENABLE_MASK: u8 = 0b1000_0000;
pub const LOOP_MASK: u8 = 0b0100_0000;
pub const FREQUENCY: u8 = 0b0000_1111;

impl Reg for Frequency {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
