use crate::emulator::apu::regs::Reg;

use super::envelope::*;
use super::length_counter::*;
use super::linear_feedback::*;

pub struct Noise {
    pub envelope: Envelope,
    pub linear_feedback: LinearFeedback,
    pub length_counter: LengthCounter,

    current_timer: u16,
    shift_register: u16,
}

const NUM_OF_NOISE_PERIODS: usize = 16;

const NOISE_PERIOD_TABLE: [u16; NUM_OF_NOISE_PERIODS] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

const BIT_0_MASK: u16 = 1;
const BIT_14_OFFSET: usize = 14;

impl Noise {
    pub fn new() -> Self {
        Self {
            envelope: Envelope::new(),
            linear_feedback: LinearFeedback::new(),
            length_counter: LengthCounter::new(),
            current_timer: 0,
            shift_register: 1,
        }
    }

    pub fn step_timer(&mut self) {
        if self.current_timer > 0 {
            self.current_timer -= 1;
        } else {
            let index = self.linear_feedback.read(NOISE_PERIOD_MASK) as usize;
            self.current_timer = NOISE_PERIOD_TABLE[index];

            let mode = self.linear_feedback.read(NOISE_MODE_MASK) != 0;
            let tap_bit = if mode { 6 } else { 1 };

            let bit_0 = self.shift_register & BIT_0_MASK;
            let bit_tap = (self.shift_register >> tap_bit) & BIT_0_MASK;
            let feadback = bit_0 ^ bit_tap;

            self.shift_register >>= 1;
            self.shift_register |= feadback << BIT_14_OFFSET;
        }
    }

    pub fn output(&self) -> u8 {
        let mut res = 0;

        if self.length_counter.is_active() && ((self.shift_register & BIT_0_MASK) != 0) {
            res = self.envelope.output_volume();
        }

        res
    }
}
