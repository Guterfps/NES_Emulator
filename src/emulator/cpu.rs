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

#[derive(Debug)]
enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
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
            let op_code = self.memory.mem_read(self.program_counter);
            self.program_counter += 1;

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
    fn lda(&mut self, mode: &AddressingMode) {
        self.load_reg(RegName::A, mode);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        self.load_reg(RegName::X, mode);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        self.load_reg(RegName::Y, mode);
    }

    fn load_reg(&mut self, reg_name: RegName, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let val = self.memory.mem_read(addr);

        let reg = match reg_name {
            RegName::X => &mut self.indx_reg_x,
            RegName::Y => &mut self.indx_reg_y,
            RegName::A => &mut self.accumulator,
        };

        *reg = val;

        Self::update_zero_flag(*reg, &mut self.status_reg);
        Self::update_negative_flag(*reg, &mut self.status_reg);

        self.move_pc(mode);
    }

    // transfer ops
    fn tax(&mut self, mode: &AddressingMode) {
        self.indx_reg_x = self.accumulator;

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);

        self.move_pc(mode);
    }

    // Increment ops
    fn inx(&mut self, mode: &AddressingMode) {
        self.indx_reg_x = self.indx_reg_x.wrapping_add(1);

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);

        self.move_pc(mode);
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

    fn move_pc(&mut self, mode: &AddressingMode) {
        use AddressingMode::*;

        let bytes_to_move = match mode {
            Implicit | Accumulator => 0,
            Immediate | ZeroPage | ZeroPageX | ZeroPageY | IndexedIndirect | IndirectIndexed => 1,
            Relative | Absolute | AbsoluteX | AbsoluteY | Indirect => 2,
        };

        self.program_counter += bytes_to_move;
    }

    fn op_code_instraction(&mut self, op_code: u8) -> bool {
        let mut is_break = false;

        match op_code {
            0xA9 => self.lda(&AddressingMode::Immediate),
            0xA2 => self.ldx(&AddressingMode::Immediate),
            0xA0 => self.ldy(&AddressingMode::Immediate),
            0xAA => self.tax(&AddressingMode::Implicit),
            0xE8 => self.inx(&AddressingMode::Implicit),
            0x00 => is_break = true,
            _ => todo!(),
        };

        is_break
    }

    fn get_operand_addr(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implicit => todo!(),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Immediate => self.immediate_addr(),
            AddressingMode::ZeroPage => self.zero_page_addr(),
            AddressingMode::ZeroPageX => self.zero_page_x_addr(),
            AddressingMode::ZeroPageY => self.zero_page_y_addr(),
            AddressingMode::Relative => self.relative_addr(),
            AddressingMode::Absolute => self.absolute_addr(),
            AddressingMode::AbsoluteX => self.absolute_x_addr(),
            AddressingMode::AbsoluteY => self.absolute_y_addr(),
            AddressingMode::Indirect => self.relative_addr(),
            AddressingMode::IndexedIndirect => self.indexed_indirect_addr(),
            AddressingMode::IndirectIndexed => self.indirect_indexed_addr(),
        }
    }

    fn immediate_addr(&self) -> u16 {
        self.program_counter as u16
    }

    fn zero_page_addr(&self) -> u16 {
        let addr = self.memory.mem_read(self.program_counter);
        addr as u16
    }

    fn zero_page_x_addr(&self) -> u16 {
        let param = self.memory.mem_read(self.program_counter);
        let addr = self.indx_reg_x.wrapping_add(param);
        addr as u16
    }

    fn zero_page_y_addr(&self) -> u16 {
        let param = self.memory.mem_read(self.program_counter);
        let addr = self.indx_reg_y.wrapping_add(param);
        addr as u16
    }

    fn relative_addr(&self) -> u16 {
        let offset = self.memory.mem_read(self.program_counter);
        let addr = self.program_counter as i16 + offset as i16;
        addr as u16
    }

    fn absolute_addr(&mut self) -> u16 {
        self.memory.mem_read_u16(self.program_counter)
    }

    fn absolute_x_addr(&mut self) -> u16 {
        let param = self.memory.mem_read_u16(self.program_counter);
        let addr = self.indx_reg_x as u16 + param;
        addr
    }

    fn absolute_y_addr(&mut self) -> u16 {
        let param = self.memory.mem_read_u16(self.program_counter);
        let addr = self.indx_reg_y as u16 + param;
        addr
    }

    fn indirect_addr(&mut self) -> u16 {
        self.memory.mem_read_u16(self.program_counter)
    }

    fn indexed_indirect_addr(&self) -> u16 {
        let param = self.memory.mem_read(self.program_counter);
        let peek1 = self
            .memory
            .mem_read(self.indx_reg_x.wrapping_add(param) as u16);
        let peek2 = self
            .memory
            .mem_read(self.indx_reg_x.wrapping_add(param).wrapping_add(1) as u16)
            as u16;
        let addr = peek1 as u16 + (peek2 << 8);

        addr
    }

    fn indirect_indexed_addr(&self) -> u16 {
        let param = self.memory.mem_read(self.program_counter);
        let peek1 = self.memory.mem_read(param as u16);
        let peek2 = self.memory.mem_read(param.wrapping_add(1) as u16) as u16;
        let addr = peek1 as u16 + (peek2 << 8) + self.indx_reg_y as u16;

        addr
    }
}

fn is_negative(val: u8) -> bool {
    const NEGATIVE_BIT: u8 = 0b1000_0000;
    (val & NEGATIVE_BIT) != 0
}

#[cfg(test)]
mod test;
