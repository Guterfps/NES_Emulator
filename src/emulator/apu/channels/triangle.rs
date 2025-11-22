use super::length_counter::*;
use super::linear_counter::*;
use super::timer::*;

pub struct Triangle {
    linear_counter_load: LinearCounter,
    timer: TimerLow,
    length_counter: LengthCounter,
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            linear_counter_load: LinearCounter::new(),
            timer: TimerLow::new(),
            length_counter: LengthCounter::new(),
        }
    }
}
