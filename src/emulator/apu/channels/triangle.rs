use super::Reg;
use super::length_counter::*;
use super::linear_counter::*;
use super::timer::*;

pub struct Triangle {
    linear_counter_load: LinearCounter,
    timer_low: TimerLow,
    length_counter: LengthCounter,

    current_timer: u16,
    sequence_index: u8,
}

const HIGH_TIMER_OFFSET: usize = 8;
const SEQUENCE_TABLE_SIZE: usize = 32;

#[rustfmt::skip]
const TRIANGLE_SEQUENCE: [u8; SEQUENCE_TABLE_SIZE] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
];

impl Triangle {
    pub fn new() -> Self {
        Self {
            linear_counter_load: LinearCounter::new(),
            timer_low: TimerLow::new(),
            length_counter: LengthCounter::new(),
            current_timer: 0,
            sequence_index: 0,
        }
    }

    fn get_timer_period(&self) -> u16 {
        let low = self.timer_low.read(TIMER_LOW_MASK) as u16;
        let high = self.length_counter.read(TIMER_HIGH_MASK) as u16;

        low | (high << HIGH_TIMER_OFFSET)
    }

    pub fn step_timer(&mut self) {
        if self.current_timer > 0 {
            self.current_timer -= 1;
        } else {
            self.current_timer = self.get_timer_period();

            if self.length_counter.is_active() && (self.linear_counter_load.counter > 0) {
                self.sequence_index = (self.sequence_index + 1) & (SEQUENCE_TABLE_SIZE - 1) as u8;
            }
        }
    }

    pub fn output(&self) -> u8 {
        TRIANGLE_SEQUENCE[self.sequence_index as usize]
    }

    pub fn linear_counter_tick(&mut self) {
        self.linear_counter_load.tick();
    }

    pub fn length_counter_tick(&mut self) {
        self.length_counter.tick();
    }
}
