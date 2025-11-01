mod channels;
mod regs;

use channels::pulse::Pulse;

const NUM_OF_PULSE_CHANNELS: usize = 2;

pub struct APU {
    pulses: [Pulse; NUM_OF_PULSE_CHANNELS],
}
