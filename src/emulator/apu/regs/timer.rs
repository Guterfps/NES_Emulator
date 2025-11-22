use super::Reg;

pub struct TimerLow {
    data: u8,
}

pub const TIMER_LOW_MASK: u8 = 0b1111_1111;

impl TimerLow {
    pub fn new() -> Self {
        Self { data: 0 }
    }
}

impl Reg for TimerLow {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
