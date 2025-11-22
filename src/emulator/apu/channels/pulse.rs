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

    current_timer: u16,
    duty_sequence_index: u8,
}

const HIGH_TIMER_OFFSET: usize = 8;
const SILENT_PERIOD: u16 = 8;
const ENVELOPE_DUTY_MASK_OFFSET: usize = 6;

const DUTY_TABLE_SIZE: usize = 4;
const SEQUENCE_SIZE: usize = 8;

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
            current_timer: 0,
            duty_sequence_index: 0,
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
            self.duty_sequence_index =
                self.duty_sequence_index.wrapping_sub(1) & (SEQUENCE_SIZE - 1) as u8;
        }
    }

    pub fn output(&self) -> u8 {
        let mut res = 0;
        if self.length_counter.is_active() && (self.get_timer_period() >= SILENT_PERIOD) {
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
}
