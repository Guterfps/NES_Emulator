mod memory;
mod status;

use memory::*;
use status::*;

pub struct CPU6502 {
    status_reg: StatusReg,
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    indx_reg_x: u8,
    indx_reg_y: u8,
    memory: Memory,
}

enum RegName {
    X,
    Y,
    A,
}

const PC_START_ADDR: u16 = 0x8000;
const NON_MASKABLE_INTER_HNDLER_ADDR: u16 = 0xFFFA;
const RESET_LOCATION: u16 = 0xFFFC;
const BRK_INTR_HANDLER_ADDR: u16 = 0xFFFE;
const ZERO_PAGE: u16 = 0x0000; // 0x0000 - 0x00FF
const STACK_ADDR: u16 = 0x0100; // 0x0100 - 0x01FF
const PAGE_SIZE: usize = 0xFF;

impl CPU6502 {
    pub fn new() -> Self {
        CPU6502 {
            status_reg: StatusReg::new(),
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            indx_reg_x: 0,
            indx_reg_y: 0,
            memory: Memory::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.mem[PC_START_ADDR as usize..(PC_START_ADDR as usize + program.len())]
            .copy_from_slice(&program[..]);
        self.memory.mem_write_u16(RESET_LOCATION, PC_START_ADDR);
    }

    pub fn run(&mut self) {
        let mut break_status: bool = false;

        while !break_status {
            let op_code = self.get_next_byte_in_program();

            break_status = self.op_code_instraction(op_code);
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.indx_reg_x = 0;
        self.indx_reg_y = 0;
        self.status_reg = StatusReg::new();
        self.program_counter = self.memory.mem_read_u16(RESET_LOCATION);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    // load and store ops
    fn lda(&mut self) {
        self.load_reg(RegName::A);
    }

    fn ldx(&mut self) {
        self.load_reg(RegName::X);
    }

    fn ldy(&mut self) {
        self.load_reg(RegName::Y);
    }

    fn load_reg(&mut self, reg_name: RegName) {
        let param = self.get_next_byte_in_program();

        let reg = match reg_name {
            RegName::X => &mut self.indx_reg_x,
            RegName::Y => &mut self.indx_reg_y,
            RegName::A => &mut self.accumulator,
        };

        *reg = param;

        Self::update_zero_flag(*reg, &mut self.status_reg);
        Self::update_negative_flag(*reg, &mut self.status_reg);
    }

    // transfer ops
    fn tax(&mut self) {
        self.indx_reg_x = self.accumulator;

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    // Increment ops
    fn inx(&mut self) {
        self.indx_reg_x = self.indx_reg_x.wrapping_add(1);

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    fn update_zero_flag(reg: u8, status: &mut StatusReg) {
        if reg == 0 {
            status.set_flag(ZERO_FLAG);
        } else {
            status.unset_flag(ZERO_FLAG);
        }
    }

    fn update_negative_flag(reg: u8, status: &mut StatusReg) {
        if is_negative(reg) {
            status.set_flag(NEGATIVE_FLAG);
        } else {
            status.unset_flag(NEGATIVE_FLAG);
        }
    }

    fn get_next_byte_in_program(&mut self) -> u8 {
        let byte = self.memory.mem_read(self.program_counter);
        self.program_counter += 1;

        byte
    }

    fn op_code_instraction(&mut self, op_code: u8) -> bool {
        let mut is_break = false;

        match op_code {
            0xA9 => self.lda(),
            0xA2 => self.ldx(),
            0xA0 => self.ldy(),
            0xAA => self.tax(),
            0xE8 => self.inx(),
            0x00 => is_break = true,
            _ => todo!(),
        };

        is_break
    }
}

fn is_negative(val: u8) -> bool {
    const NEGATIVE_BIT: u8 = 0b1000_0000;
    (val & NEGATIVE_BIT) != 0
}

#[cfg(test)]
mod test;
