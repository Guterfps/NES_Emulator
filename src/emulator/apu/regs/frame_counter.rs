use super::Reg;

pub struct FrameCounter {
    data: u8,
}

pub const MODE_MASK: u8 = 0b1000_0000;
pub const IRQ_FLAG_MASK: u8 = 0b0100_0000;

impl Reg for FrameCounter {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
