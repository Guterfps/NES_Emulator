use super::Reg;

pub struct FrameCounter {
    data: u8,
    cycle_count: usize,
    frame_irq_active: bool,
    interrupt_inhbit: bool,
    mode: bool,
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
const STEP_1: usize = APU_STEP;
const STEP_2: usize = 2 * APU_STEP;
const STEP_3: usize = 3 * APU_STEP;
const STEP_4: usize = 4 * APU_STEP;
const STEP_4_END: usize = STEP_4 + 1;
const STEP_5: usize = 5 * APU_STEP;

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            data: 0,
            cycle_count: 0,
            frame_irq_active: false,
            interrupt_inhbit: false,
            mode: false,
        }
    }

    pub fn step(&mut self) -> FrameAction {
        self.cycle_count += 1;
        let mut res = FrameAction::None;

        if !self.mode {
            if !self.interrupt_inhbit && (self.cycle_count == STEP_4) {
                self.frame_irq_active = true;
            }
            if self.cycle_count == STEP_4_END {
                self.cycle_count = 0;
                res = FrameAction::HalfFrame;
            }
        } else if self.cycle_count == STEP_5 {
            self.cycle_count = 0;
        }

        if res != FrameAction::None {
            res = match self.cycle_count {
                STEP_1 | STEP_3 => FrameAction::QuarterFrame,
                STEP_2 => FrameAction::HalfFrame,
                _ => FrameAction::None,
            };
        }

        res
    }

    pub fn clear_irq(&mut self) {
        self.frame_irq_active = false;
    }

    pub fn is_irq_active(&self) -> bool {
        self.frame_irq_active
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

        self.mode = (self.data & MODE_MASK) != 0;
        self.interrupt_inhbit = (self.data & IRQ_FLAG_MASK) != 0;

        if self.interrupt_inhbit {
            self.frame_irq_active = false;
        }

        self.cycle_count = 0;
    }
}
