use super::length_counter::*;
use super::linear_counter::*;
use super::timer::*;

pub struct Triangle {
    linear_counter_load: LinearCounter,
    timer: TimerLow,
    length_counter: LengthCounter,
}
