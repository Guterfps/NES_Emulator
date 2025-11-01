pub mod envelope;
pub mod frame_counter;
pub mod frequency;
pub mod length_counter;
pub mod linear_counter;
pub mod linear_feedback;
pub mod load_counter;
pub mod status;
pub mod sweep;
pub mod timer;

pub trait Reg {
    fn data_mut(&mut self) -> &mut u8;
    fn data(&self) -> u8;

    fn read(&self, mask: u8) -> u8 {
        self.data() & mask
    }

    fn write(&mut self, mask: u8, val: u8) {
        *(self.data_mut()) = (self.data() & !mask) | (mask & val);
    }
}
