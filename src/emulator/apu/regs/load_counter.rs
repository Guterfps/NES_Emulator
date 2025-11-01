use super::Reg;

pub struct LoadCounter {
    data: u8,
}

pub const LOAD_COUNTER_MASK: u8 = 0b0111_1111;

impl Reg for LoadCounter {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
