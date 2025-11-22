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
    global_cycle: usize,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            pulses: [Pulse::new(), Pulse::new()],
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: Dmc::new(),
            status: Status::new(),
            frame_counter: FrameCounter::new(),
            global_cycle: 0,
        }
    }

    pub fn tick(&mut self) {
        let action = self.frame_counter.step();

        match action {
            FrameAction::QuarterFrame => self.step_quarter_frame(),
            FrameAction::HalfFrame => self.step_half_frame(),
            FrameAction::None => {}
        }

        self.triangle.step_timer();

        if (self.global_cycle & 1) == 0 {
            for pulse in self.pulses.iter_mut() {
                pulse.step_timer();
            }
            // self.noise.step_timer();
            // self.dmc.step_timer();
        }

        self.global_cycle += 1;
    }

    fn step_quarter_frame(&mut self) {
        for pulse in self.pulses.iter_mut() {
            pulse.envelope_tick();
        }
        self.triangle.linear_counter_tick();
        // self.noise.envelope_tick();
    }

    fn step_half_frame(&mut self) {
        self.step_quarter_frame();

        for pulse in self.pulses.iter_mut() {
            pulse.length_counter_tick();
        }
        self.triangle.length_counter_tick();
        // self.noise.length_counter_tick();

        // self.pulses[0].sweep.tick(); // TODO: Implement Sweep logic later
        // self.pulses[1].sweep.tick();
    }

    // pub fn get_audio_sample

    // pub fn write_register(&mut self, addr: u16, data: u8)
}
