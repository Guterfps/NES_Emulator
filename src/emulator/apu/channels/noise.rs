use super::envelope::*;
use super::length_counter::*;
use super::linear_feedback::*;

pub struct Noise {
    envelope: Envelope,
    noise_mod_period: LinearFeedback,
    length_counter: LengthCounter,
}
