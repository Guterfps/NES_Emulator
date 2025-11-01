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
