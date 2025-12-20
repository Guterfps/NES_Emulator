use super::Reg;
use super::length_counter::*;
use super::linear_counter::*;
use super::timer::*;

pub struct Triangle {
    linear_counter_load: LinearCounter,
    timer_low: TimerLow,
    length_counter: LengthCounter,

    timer_period: u16,
    timer_counter: u16,
    sequence_index: u8,
}

const BIT_7_MASK: u8 = 0b1000_0000;
const HIGH_TIMER_OFFSET: usize = 8;
const SEQUENCE_TABLE_SIZE: usize = 32;

const WRITE_ALL_MASK: u8 = 0xFF;

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
            timer_period: 0,
            timer_counter: 0,
            sequence_index: 0,
        }
    }

    fn update_timer_period(&mut self) {
        let low = self.timer_low.read(TIMER_LOW_MASK) as u16;
        let high = self.length_counter.read(TIMER_HIGH_MASK) as u16;

        self.timer_period = low | (high << HIGH_TIMER_OFFSET);
    }

    pub fn step_timer(&mut self) {
        if self.timer_counter > 0 {
            self.timer_counter -= 1;
        } else {
            self.timer_counter = self.timer_period;

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

    pub fn linear_counter_write_all(&mut self, val: u8) {
        self.linear_counter_load.write(WRITE_ALL_MASK, val);
        self.length_counter.halt = (val & BIT_7_MASK) != 0;
    }

    pub fn timer_low_write_all(&mut self, val: u8) {
        self.timer_low.write(WRITE_ALL_MASK, val);
        self.update_timer_period();
    }

    pub fn length_counter_write_all(&mut self, val: u8) {
        self.length_counter.write(WRITE_ALL_MASK, val);
        self.update_timer_period();
        self.linear_counter_load.set_reload();
    }

    pub fn linear_counter_reload(&mut self) {
        self.linear_counter_load.reload_flag = true;
    }

    pub fn get_length_counter(&self) -> u8 {
        self.length_counter.counter
    }

    pub fn length_counter_enable(&mut self, enable: bool) {
        self.length_counter.enabled = enable;
    }

    pub fn is_length_counter_enabled(&self) -> bool {
        self.length_counter.enabled
    }

    pub fn length_counter_reset(&mut self) {
        self.length_counter.counter = 0;
    }
}
