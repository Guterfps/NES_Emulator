use crate::emulator::apu::regs::Reg;

use super::frequency::*;
use super::load_counter::*;

pub struct Dmc {
    pub freq: Frequency,
    pub direct_load: LoadCounter,
    value: u8,
    enabled: bool,

    sample_addr: u16,
    sample_len: u16,

    current_addr: u16,
    bytes_remaining: u16,
    sample_buffer: Option<u8>,

    timer_period: u16,
    timer_counter: u16,

    shift_reg: u8,
    bits_remaining: u8,
    silence_flag: bool,

    irq_active: bool,
    loop_flag: bool,
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
            value: 0,
            enabled: false,
            sample_addr: START_ADDR,
            sample_len: 1,
            current_addr: START_ADDR,
            bytes_remaining: 0,
            sample_buffer: None,
            timer_period: DMC_RATE_TABLE[0],
            timer_counter: 0,
            shift_reg: 0,
            bits_remaining: 8,
            silence_flag: true,
            irq_active: false,
            loop_flag: false,
        }
    }

    pub fn restart_sample(&mut self) {
        self.current_addr = self.sample_addr;
        self.bytes_remaining = self.sample_len;
    }

    pub fn step_timer(&mut self) {
        if let Some(val) = self.freq.take_changed() {
            self.irq_active = (val & IRQ_ENABLE_MASK) != 0;
            self.loop_flag = (val & LOOP_MASK) != 0;
            let index = (val & FREQUENCY) as usize;
            self.timer_period = DMC_RATE_TABLE[index];
        }

        if let Some(val) = self.direct_load.take_changed() {
            self.value = val & LOAD_COUNTER_MASK;
        }

        if self.timer_counter > 0 {
            self.timer_counter -= 1;
        } else {
            self.timer_counter = self.timer_period;
            self.step_output_unit();
        }
    }

    fn step_output_unit(&mut self) {
        if !self.silence_flag {
            let bit_0 = self.shift_reg & BIT_0_MASK;

            if bit_0 == 1 {
                if self.value <= MAX_OUTPUT_LEVEL {
                    self.value += 2;
                }
            } else if self.value >= MIN_OUTPUT_LEVEL {
                self.value -= 2;
            }
        }

        self.shift_reg >>= 1;
        self.bits_remaining -= 1;

        if self.bits_remaining == 0 {
            self.bits_remaining = RESET_BITS_REMAINING;

            if let Some(sample) = self.sample_buffer.take() {
                self.shift_reg = sample;
                self.silence_flag = false;
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

        if self.bytes_remaining > 0 {
            self.bytes_remaining -= 1;
        }

        if self.bytes_remaining == 0 {
            if self.loop_flag {
                self.restart_sample();
            } else if self.irq_active {
                // TODO
            }
        }
    }

    pub fn clear_interrupt(&mut self) {
        self.irq_active = false;
    }

    pub fn output(&self) -> u8 {
        self.value
    }

    pub fn enable(&mut self, enable: bool) {
        self.enabled = enable;
        if !enable {
            self.bytes_remaining = 0;
        } else if self.bytes_remaining == 0 {
            self.restart_sample();
        }
    }

    pub fn write_sample_addr(&mut self, addr: u8) {
        self.sample_addr = START_ADDR | ((addr as u16) << SHIFT_MUL_64);
    }

    pub fn write_sample_len(&mut self, len: u8) {
        self.sample_len = ((len as u16) << SHIFT_MUL_16) + 1;
    }

    pub fn get_current_addr(&self) -> u16 {
        self.current_addr
    }

    pub fn remaining_bytes(&self) -> u16 {
        self.bytes_remaining
    }

    pub fn is_irq_active(&self) -> bool {
        self.irq_active
    }
}
