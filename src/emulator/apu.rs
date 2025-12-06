mod channels;
mod regs;

use channels::dmc::Dmc;
use channels::noise::Noise;
use channels::pulse::Pulse;
use channels::triangle::Triangle;
use regs::Reg;
use regs::envelope::ENVELOPE_COUNTER_HALT_MASK;
use regs::frame_counter::*;
use regs::load_counter::LOAD_COUNTER_MASK;
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

const PULSES_ADDR_MASK: u16 = 0b111;
const NUM_OF_PULSE_REGS: u16 = 4;
const REG_WRITE_ALL_MASK: u8 = 0xFF;

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
            self.noise.step_timer();
            self.dmc.step_timer();
        }

        self.global_cycle += 1;
    }

    fn step_quarter_frame(&mut self) {
        for pulse in self.pulses.iter_mut() {
            pulse.envelope_tick();
        }
        self.triangle.linear_counter_tick();
        self.noise.envelope.tick();
    }

    fn step_half_frame(&mut self) {
        self.step_quarter_frame();

        for pulse in self.pulses.iter_mut() {
            pulse.length_counter_tick();
        }
        self.triangle.length_counter_tick();
        self.noise.length_counter.tick();

        // self.pulses[0].sweep.tick(); // TODO: Implement Sweep logic later
        // self.pulses[1].sweep.tick();
    }

    pub fn get_audio_sample(&self) -> f32 {
        let p1 = self.pulses[0].output();
        let p2 = self.pulses[1].output();
        let tri = self.triangle.output();
        let noise = self.noise.output();
        let dmc = self.dmc.output();

        // Siplefied Mixer formula (Approximation)
        let pulse_out = 0.00752 * (p1 + p2) as f32;
        let tnd_out = (0.00851 * tri as f32) + (0.00494 * noise as f32) + (0.00335 * dmc as f32);

        pulse_out + tnd_out
    }

    pub fn write_register(&mut self, addr: u16, data: u8) {
        match addr {
            // pulses
            0x4000..=0x4007 => self.write_pulses(addr, data),
            // triangle
            0x4008 => self.triangle.linear_counter_write_all(data),
            0x400A => self.triangle.timer_low_write_all(data),
            0x400B => {
                self.triangle.length_counter_write_all(data);
                self.triangle.linear_counter_reload();
            }
            // Noise
            0x400C => {
                self.noise.envelope.write(REG_WRITE_ALL_MASK, data);
                self.noise.length_counter.halt = (data & ENVELOPE_COUNTER_HALT_MASK) != 0;
            }
            0x400E => self.noise.linear_feedback.write(REG_WRITE_ALL_MASK, data),
            0x400F => {
                self.noise.length_counter.write(REG_WRITE_ALL_MASK, data);
                self.noise.envelope.restart();
            }
            // DMC
            0x4010 => self.dmc.freq.write(REG_WRITE_ALL_MASK, data),
            0x4011 => self.dmc.direct_load.write(LOAD_COUNTER_MASK, data),
            0x4012 => self.dmc.write_sample_addr(data),
            0x4013 => self.dmc.write_sample_len(data),
            // status
            0x4015 => {
                self.status.write(REG_WRITE_ALL_MASK, data);
                self.pulses[0].length_counter_enable((data & PULSE_1_CHANNEL_MASK) != 0);
                self.pulses[1].length_counter_enable((data & PULSE_2_CHANNEL_MASK) != 0);
                self.triangle
                    .length_counter_enable((data & TRIANGLE_MASK) != 0);

                if !self.pulses[0].is_length_counter_enabled() {
                    self.pulses[0].length_counter_reset();
                }
                if !self.pulses[1].is_length_counter_enabled() {
                    self.pulses[1].length_counter_reset();
                }
                if !self.triangle.is_length_counter_enabled() {
                    self.triangle.length_counter_reset();
                }

                self.noise.length_counter.enabled = (data & NOISE_MASK) != 0;
                if !self.noise.length_counter.enabled {
                    self.noise.length_counter.counter = 0;
                }

                self.dmc.enable((data & ENABLE_DMC_MASK) != 0);
            }
            // frame counter
            0x4017 => self.frame_counter.write(REG_WRITE_ALL_MASK, data),

            _ => println!("Not a valid APU write address! ({:x})", addr),
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let mut status = 0;

        if self.pulses[0].get_length_counter() > 0 {
            status |= PULSE_1_CHANNEL_MASK;
        }
        if self.pulses[1].get_length_counter() > 0 {
            status |= PULSE_2_CHANNEL_MASK;
        }
        if self.triangle.get_length_counter() > 0 {
            status |= TRIANGLE_MASK;
        }
        if self.noise.length_counter.counter > 0 {
            status |= NOISE_MASK;
        }
        if self.dmc.remaining_bytes() > 0 {
            status |= ENABLE_DMC_MASK;
        }

        // TODO: handle Frame Interrupt logic

        status
    }

    fn write_pulses(&mut self, addr: u16, data: u8) {
        let mask_addr = addr & PULSES_ADDR_MASK;
        let iner_reg_addr = mask_addr % NUM_OF_PULSE_REGS;
        let pulse_index = mask_addr / NUM_OF_PULSE_REGS;

        match iner_reg_addr {
            0 => {
                self.pulses[pulse_index as usize].envelope_write_all(data);
                self.pulses[pulse_index as usize]
                    .length_counter_halt((data & ENVELOPE_COUNTER_HALT_MASK) != 0);
            }
            1 => self.pulses[pulse_index as usize].sweep_write_all(data),
            2 => self.pulses[pulse_index as usize].timer_low_write_all(data),
            3 => {
                self.pulses[pulse_index as usize].length_counter_write_all(data);
                self.pulses[pulse_index as usize].reset_phase_envelope();
            }
            _ => {
                panic!("Not a valid pulse register address! ({:x})", addr)
            }
        }
    }

    pub fn needs_dmc_sample(&self) -> bool {
        self.dmc.needs_sample()
    }

    pub fn get_dmc_addr(&self) -> u16 {
        self.dmc.get_current_addr()
    }

    pub fn set_dmc_sample(&mut self, val: u8) {
        self.dmc.set_sample_buffer(val);
    }
}
