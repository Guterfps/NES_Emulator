use super::Reg;

pub struct Status {
    data: u8,
}

pub const DMC_INTERRUPT_MASK: u8 = 0b1000_0000;
pub const FRAME_INTERRUPT_MASK: u8 = 0b0100_0000;
pub const ENABLE_DMC_MASK: u8 = 0b0001_0000;
pub const NOISE_MASK: u8 = 0b0000_1000;
pub const TRIANGLE_MASK: u8 = 0b0000_0100;
pub const PULSE_2_CHANNEL_MASK: u8 = 0b0000_0010;
pub const PULSE_1_CHANNEL_MASK: u8 = 0b0000_0001;

impl Reg for Status {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
