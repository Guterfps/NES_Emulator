use super::Reg;

pub struct LinearCounter {
    data: u8,
}

pub const CONTROL_MASK: u8 = 0b1000_0000;
pub const LOAD_MASK: u8 = 0b0111_1111;

impl Reg for LinearCounter {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
