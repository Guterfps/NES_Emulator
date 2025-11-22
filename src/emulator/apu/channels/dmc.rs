use super::frequency::*;
use super::load_counter::*;

pub struct Dmc {
    freq: Frequency,
    load_counter: LoadCounter,
    sample_addr: u8,
    sample_len: u8,
}

impl Dmc {
    pub fn new() -> Self {
        Self {
            freq: Frequency::new(),
            load_counter: LoadCounter::new(),
            sample_addr: 0,
            sample_len: 0,
        }
    }
}
