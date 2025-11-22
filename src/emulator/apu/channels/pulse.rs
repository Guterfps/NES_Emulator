use super::envelope::*;
use super::length_counter::*;
use super::sweep::*;
use super::timer::*;

pub struct Pulse {
    envelope: Envelope,
    sweep: Sweep,
    timer: TimerLow,
    length_counter: LengthCounter,
}

impl Pulse {
    pub fn new() -> Self {
        Self {
            envelope: Envelope::new(),
            sweep: Sweep::new(),
            timer: TimerLow::new(),
            length_counter: LengthCounter::new(),
        }
    }
}
