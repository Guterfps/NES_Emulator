use super::envelope::*;
use super::length_counter::*;
use super::linear_feedback::*;

pub struct Noise {
    envelope: Envelope,
    noise_mod_period: LinearFeedback,
    length_counter: LengthCounter,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            envelope: Envelope::new(),
            noise_mod_period: LinearFeedback::new(),
            length_counter: LengthCounter::new(),
        }
    }
}
