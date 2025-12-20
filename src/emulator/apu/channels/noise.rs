use super::envelope::*;
use super::length_counter::*;
use super::linear_feedback::*;

pub struct Noise {
    pub envelope: Envelope,
    pub linear_feedback: LinearFeedback,
    pub length_counter: LengthCounter,

    timer_counter: u16,
    timer_period: u16,
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
            timer_counter: 0,
            timer_period: 0,
            shift_register: 1,
        }
    }

    pub fn step_timer(&mut self) {
        if let Some(_val) = self.linear_feedback.take_changed() {
            let index = self.linear_feedback.get_period_index();
            self.timer_period = NOISE_PERIOD_TABLE[index];
        }

        if self.timer_counter > 0 {
            self.timer_counter -= 1;
        } else {
            self.timer_counter = self.timer_period;

            let mode = self.linear_feedback.get_mode();
            let tap_bit = if mode { 6 } else { 1 };

            let bit_0 = self.shift_register & BIT_0_MASK;
            let bit_tap = (self.shift_register >> tap_bit) & BIT_0_MASK;
            let feedback = bit_0 ^ bit_tap;

            self.shift_register >>= 1;
            self.shift_register |= feedback << BIT_14_OFFSET;
        }
    }

    pub fn output(&self) -> u8 {
        let mut res = 0;

        if self.length_counter.is_active() && ((self.shift_register & BIT_0_MASK) == 0) {
            res = self.envelope.output_volume();
        }

        res
    }
}
