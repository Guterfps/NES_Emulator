use super::Reg;
use super::envelope::*;
use super::length_counter::*;
use super::sweep::*;
use super::timer::*;

pub struct Pulse {
    envelope: Envelope,
    sweep: Sweep,
    timer_low: TimerLow,
    length_counter: LengthCounter,

    timer_period: u16,
    timer_counter: u16,
    duty_sequence_index: u8,
}

const HIGH_TIMER_OFFSET: usize = 8;
const SILENT_PERIOD: u16 = 8;
const ENVELOPE_DUTY_MASK_OFFSET: usize = 6;

const DUTY_TABLE_SIZE: usize = 4;
const SEQUENCE_SIZE: usize = 8;

const WRITE_ALL_MASK: u8 = 0xFF;

const TIMER_MASK: u16 = 0x7FF;

const DUTY_TABLE: [[u8; SEQUENCE_SIZE]; DUTY_TABLE_SIZE] = [
    [0, 1, 0, 0, 0, 0, 0, 0], // 12.5%
    [0, 1, 1, 0, 0, 0, 0, 0], // 25%
    [0, 1, 1, 1, 1, 0, 0, 0], // 50%
    [1, 0, 0, 1, 1, 1, 1, 1], // 25% Negated
];

impl Pulse {
    pub fn new() -> Self {
        Self {
            envelope: Envelope::new(),
            sweep: Sweep::new(),
            timer_low: TimerLow::new(),
            length_counter: LengthCounter::new(),
            timer_period: 0,
            timer_counter: 0,
            duty_sequence_index: 0,
        }
    }

    fn update_timer_period(&mut self) {
        let low = self.timer_low.read(TIMER_LOW_MASK) as u16;
        let high = self.length_counter.read(TIMER_HIGH_MASK) as u16;

        self.timer_period = low | (high << HIGH_TIMER_OFFSET)
    }

    pub fn get_timer_period(&self) -> u16 {
        self.timer_period
    }

    pub fn set_timer_period(&mut self, period: u16) {
        self.timer_period = period & TIMER_MASK;
    }

    pub fn step_timer(&mut self) {
        if self.timer_counter > 0 {
            self.timer_counter -= 1;
        } else {
            self.timer_counter = self.timer_period;
            self.duty_sequence_index =
                self.duty_sequence_index.wrapping_sub(1) & (SEQUENCE_SIZE - 1) as u8;
        }
    }

    pub fn output(&self) -> u8 {
        let mut res = 0;
        if self.length_counter.is_active()
            && (self.timer_period >= SILENT_PERIOD)
            && !self.sweep.is_muted()
        {
            let duty_mode = self.envelope.read(DUTY_MASK) >> ENVELOPE_DUTY_MASK_OFFSET;
            let bit = DUTY_TABLE[duty_mode as usize][self.duty_sequence_index as usize];

            if bit != 0 {
                res = self.envelope.output_volume()
            }
        }

        res
    }

    pub fn envelope_tick(&mut self) {
        self.envelope.tick();
    }

    pub fn length_counter_tick(&mut self) {
        self.length_counter.tick();
    }

    pub fn sweep_tick(&mut self, current_period: u16, is_pulse_1: bool) -> Option<u16> {
        self.sweep.tick(current_period, is_pulse_1)
    }

    pub fn envelope_write_all(&mut self, val: u8) {
        self.envelope.write(WRITE_ALL_MASK, val);
    }

    pub fn length_counter_halt(&mut self, halt: bool) {
        self.length_counter.halt = halt;
    }

    pub fn sweep_write_all(&mut self, val: u8) {
        self.sweep.write(WRITE_ALL_MASK, val);
    }

    pub fn timer_low_write_all(&mut self, val: u8) {
        self.timer_low.write(WRITE_ALL_MASK, val);
        self.update_timer_period();
    }

    pub fn length_counter_write_all(&mut self, val: u8) {
        self.length_counter.write(WRITE_ALL_MASK, val);
        self.update_timer_period();
        self.duty_sequence_index = 0;
    }

    pub fn reset_phase_envelope(&mut self) {
        self.envelope.restart();
        self.duty_sequence_index = 0;
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
