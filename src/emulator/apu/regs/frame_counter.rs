use super::Reg;

pub struct FrameCounter {
    data: u8,
    cycle_count: usize,
    step_index: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FrameAction {
    None,
    QuarterFrame,
    HalfFrame,
}

pub const MODE_MASK: u8 = 0b1000_0000;
pub const IRQ_FLAG_MASK: u8 = 0b0100_0000;

const APU_STEP: usize = 7457; // NTSC roughly 7457 cycles

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            data: 0,
            cycle_count: 0,
            step_index: 0,
        }
    }

    pub fn step(&mut self) -> FrameAction {
        self.cycle_count += 1;
        let mut res = FrameAction::None;

        if self.cycle_count >= APU_STEP {
            self.cycle_count = 0;
            self.step_index += 1;

            let mode = (self.data & MODE_MASK) == MODE_MASK;

            res = match self.step_index {
                1 => FrameAction::QuarterFrame,
                2 => FrameAction::HalfFrame,
                3 => FrameAction::QuarterFrame,
                4 => {
                    if !mode {
                        self.step_index = 0;
                        FrameAction::HalfFrame
                    } else {
                        FrameAction::None
                    }
                }
                5 => {
                    if mode {
                        self.step_index = 0;
                        FrameAction::HalfFrame
                    } else {
                        FrameAction::None
                    }
                }
                _ => FrameAction::None,
            }
        }

        res
    }

    fn reset_internal_state(&mut self) {
        self.cycle_count = 0;
        self.step_index = 0;
    }
}

impl Reg for FrameCounter {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }

    fn write(&mut self, mask: u8, val: u8) {
        self.data = (self.data & !mask) | (val & mask);

        self.reset_internal_state();
    }
}
