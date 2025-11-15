mod channels;
mod regs;

use channels::dmc::Dmc;
use channels::noise::Noise;
use channels::pulse::Pulse;
use channels::triangle::Triangle;
use regs::frame_counter::*;
use regs::status::*;

const NUM_OF_PULSE_CHANNELS: usize = 2;

pub struct Apu {
    pulses: [Pulse; NUM_OF_PULSE_CHANNELS],
    triangle: Triangle,
    noise: Noise,
    dmc: Dmc,
    status: Status,
    frame_counter: FrameCounter,
}
