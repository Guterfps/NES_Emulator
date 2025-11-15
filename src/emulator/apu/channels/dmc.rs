use super::frequency::*;
use super::load_counter::*;

pub struct Dmc {
    freq: Frequency,
    load_counter: LoadCounter,
    sample_addr: u8,
    sample_len: u8,
}
