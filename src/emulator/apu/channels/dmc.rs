use crate::emulator::apu::regs::Reg;

use super::frequency::*;
use super::load_counter::*;

pub struct Dmc {
    pub freq: Frequency,
    pub direct_load: LoadCounter,
    sample_addr: u8,
    sample_len: u8,

    current_addr: u16,
    bytes_remaining: u16,
    sample_buffer: Option<u8>,

    current_timer: u16,
    shift_reg: u8,
    bits_remaining: u8,
    silence_flag: bool,

    irq_active: bool,
}

const NUM_OF_RATES: usize = 16;

const DMC_RATE_TABLE: [u16; NUM_OF_RATES] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

const START_ADDR: u16 = 0xC000;
const SHIFT_MUL_64: usize = 6;
const SHIFT_MUL_16: usize = 4;
const BIT_0_MASK: u8 = 1;
const MAX_OUTPUT_LEVEL: u8 = 125;
const MIN_OUTPUT_LEVEL: u8 = 2;
const RESET_BITS_REMAINING: u8 = 8;
const LOOP_CPU_SPACE: u16 = 0x8000;

impl Dmc {
    pub fn new() -> Self {
        Self {
            freq: Frequency::new(),
            direct_load: LoadCounter::new(),
            sample_addr: 0,
            sample_len: 0,
            current_addr: START_ADDR,
            bytes_remaining: 0,
            sample_buffer: None,
            current_timer: 0,
            shift_reg: 0,
            bits_remaining: 8,
            silence_flag: true,
            irq_active: false,
        }
    }

    pub fn enable(&mut self, enabled: bool) {
        if !enabled {
            self.bytes_remaining = 0;
        } else if self.bytes_remaining == 0 {
            self.restart_sample();
        }
        self.clear_interrupt();
    }

    pub fn restart_sample(&mut self) {
        self.current_addr = START_ADDR + ((self.sample_addr as u16) << SHIFT_MUL_64);
        self.bytes_remaining = ((self.sample_len as u16) << SHIFT_MUL_16) + 1;
    }

    pub fn step_timer(&mut self) {
        if self.current_timer > 0 {
            self.current_timer -= 1;
        } else {
            let index = self.freq.read(FREQUENCY) as usize;
            self.current_timer = DMC_RATE_TABLE[index];
            self.step_output_unit();
        }
    }

    fn step_output_unit(&mut self) {
        if !self.silence_flag {
            let bit_0 = self.shift_reg & BIT_0_MASK;
            let mut level = self.direct_load.read(LOAD_COUNTER_MASK);

            if bit_0 == 1 {
                if level <= MAX_OUTPUT_LEVEL {
                    level += 2;
                }
            } else if level >= MIN_OUTPUT_LEVEL {
                level -= 2;
            }

            self.direct_load.write(LOAD_COUNTER_MASK, level);
            self.shift_reg >>= 1;
        }

        self.bits_remaining -= 1;

        if self.bits_remaining == 0 {
            self.bits_remaining = RESET_BITS_REMAINING;

            if let Some(sample) = self.sample_buffer {
                self.shift_reg = sample;
                self.silence_flag = false;
                self.sample_buffer = None;
            } else {
                self.silence_flag = true;
            }
        }
    }

    pub fn needs_sample(&self) -> bool {
        self.sample_buffer.is_none() && (self.bytes_remaining > 0)
    }

    pub fn set_sample_buffer(&mut self, val: u8) {
        self.sample_buffer = Some(val);
        self.current_addr = self.current_addr.wrapping_add(1);
        if self.current_addr == 0 {
            self.current_addr = LOOP_CPU_SPACE;
        }

        self.bytes_remaining -= 1;
        if self.bytes_remaining == 0 {
            if self.freq.read(LOOP_MASK) != 0 {
                self.restart_sample();
            } else if self.freq.read(IRQ_ENABLE_MASK) != 0 {
                self.irq_active = true;
            }
        }
    }

    pub fn clear_interrupt(&mut self) {
        self.irq_active = false;
    }

    pub fn output(&self) -> u8 {
        self.direct_load.read(LOAD_COUNTER_MASK)
    }

    pub fn write_sample_addr(&mut self, addr: u8) {
        self.sample_addr = addr;
    }

    pub fn write_sample_len(&mut self, len: u8) {
        self.sample_len = len;
    }

    pub fn get_current_addr(&self) -> u16 {
        self.current_addr
    }

    pub fn remaining_bytes(&self) -> u16 {
        self.bytes_remaining
    }
}
