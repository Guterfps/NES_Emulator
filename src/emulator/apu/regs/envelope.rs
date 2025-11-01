use super::Reg;

pub struct Envelope {
    data: u8,
}

pub const DUTY_MASK: u8 = 0b1100_0000;
pub const ENVELOPE_COUNTER_HALT_MASK: u8 = 0b0010_0000;
pub const CONSTANST_VOLUME_MASK: u8 = 0b0001_0000;
pub const VOLUME_MASK: u8 = 0b000_1111;

impl Reg for Envelope {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
