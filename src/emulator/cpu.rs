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
    IndexedIndirectX,
    IndirectIndexedY,
}

const PC_START_ADDR: u16 = 0x8000;
const NON_MASKABLE_INTER_HNDLER_ADDR: u16 = 0xFFFA;
const RESET_LOCATION: u16 = 0xFFFC;
const BRK_INTR_HANDLER_ADDR: u16 = 0xFFFE;
const ZERO_PAGE: u16 = 0x0000; // 0x0000 - 0x00FF
const STACK_ADDR: u16 = 0x0100; // 0x0100 - 0x01FF
const PAGE_SIZE: u8 = 0xFF;

impl CPU6502 {
    pub fn new() -> Self {
        CPU6502 {
            status_reg: StatusReg::new(),
            program_counter: 0,
            stack_pointer: PAGE_SIZE,
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
        self.stack_pointer = PAGE_SIZE;
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

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        self.memory.mem_write(addr, self.accumulator);

        self.move_pc(mode);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        self.memory.mem_write(addr, self.indx_reg_x);

        self.move_pc(mode);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        self.memory.mem_write(addr, self.indx_reg_y);

        self.move_pc(mode);
    }

    // transfer ops
    fn tax(&mut self, mode: &AddressingMode) {
        self.indx_reg_x = self.accumulator;

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);

        self.move_pc(mode);
    }

    fn tay(&mut self, mode: &AddressingMode) {
        self.indx_reg_y = self.accumulator;

        Self::update_zero_flag(self.indx_reg_y, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_y, &mut self.status_reg);

        self.move_pc(mode);
    }

    fn txa(&mut self, mode: &AddressingMode) {
        self.accumulator = self.indx_reg_x;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);

        self.move_pc(mode);
    }

    fn tya(&mut self, mode: &AddressingMode) {
        self.accumulator = self.indx_reg_y;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);

        self.move_pc(mode);
    }

    // stack ops
    fn tsx(&mut self, mode: &AddressingMode) {
        self.indx_reg_x = self.stack_pointer;

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);

        self.move_pc(mode);
    }

    fn txs(&mut self, mode: &AddressingMode) {
        self.stack_pointer = self.indx_reg_x;
        self.move_pc(mode);
    }

    fn pha(&mut self, mode: &AddressingMode) {
        self.push_stack(self.accumulator);
        self.move_pc(mode);
    }

    fn php(&mut self, mode: &AddressingMode) {
        self.push_stack(self.status_reg.status);
        self.move_pc(mode);
    }

    fn pla(&mut self, mode: &AddressingMode) {
        self.accumulator = self.pop_stack();

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
        self.move_pc(mode);
    }

    fn plp(&mut self, mode: &AddressingMode) {
        self.status_reg.status = self.pop_stack();
        self.move_pc(mode);
    }

    fn push_stack(&mut self, val: u8) {
        let mem_addr = STACK_ADDR + self.stack_pointer as u16;
        self.memory.mem_write(mem_addr, val);
        self.stack_pointer -= 1;
    }

    fn pop_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        let mem_addr = STACK_ADDR + self.stack_pointer as u16;
        self.memory.mem_read(mem_addr)
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
            Immediate | ZeroPage | ZeroPageX | ZeroPageY | IndexedIndirectX | IndirectIndexedY => 1,
            Relative | Absolute | AbsoluteX | AbsoluteY | Indirect => 2,
        };

        self.program_counter += bytes_to_move;
    }

    fn op_code_instraction(&mut self, op_code: u8) -> bool {
        let mut is_break = false;
        let addres_mode = Self::get_opcode_address_mode(op_code);

        match op_code {
            // load and store ops
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&addres_mode),
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&addres_mode),
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&addres_mode),
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&addres_mode),
            0x86 | 0x96 | 0x8E => self.stx(&addres_mode),
            0x84 | 0x94 | 0x8C => self.sty(&addres_mode),
            // transfer ops
            0xAA => self.tax(&addres_mode),
            0xA8 => self.tay(&addres_mode),
            0x8A => self.txa(&addres_mode),
            0x98 => self.tya(&addres_mode),
            // stack ops
            0xBA => self.tsx(&addres_mode),
            0x9A => self.txs(&addres_mode),
            0x48 => self.pha(&addres_mode),
            0x08 => self.php(&addres_mode),
            0x68 => self.pla(&addres_mode),
            0x28 => self.plp(&addres_mode),

            0xE8 => self.inx(&addres_mode),
            0x00 => is_break = true,
            _ => todo!(),
        };

        is_break
    }

    fn get_opcode_address_mode(opcode: u8) -> AddressingMode {
        match opcode {
            0x00 | 0x18 | 0xD8 | 0xB8 | 0x58 | 0xCA | 0x88 | 0xE8 | 0xC8 | 0xEA | 0x48 | 0x08
            | 0x68 | 0x28 | 0x40 | 0x60 | 0x38 | 0xF8 | 0x78 | 0xAA | 0xA8 | 0xBA | 0x8A | 0x9A
            | 0x98 => AddressingMode::Implicit,
            0x0A | 0x4A | 0x2A | 0x6A => AddressingMode::Accumulator,
            0xA9 | 0xA2 | 0xA0 | 0x09 | 0xE9 | 0x69 | 0x29 | 0xC9 | 0xE0 | 0xC0 | 0x49 => {
                AddressingMode::Immediate
            }
            0x65 | 0x25 | 0x06 | 0x24 | 0xC5 | 0xE4 | 0xC4 | 0xC6 | 0x45 | 0xE6 | 0xA5 | 0xA6
            | 0xA4 | 0x46 | 0x05 | 0x26 | 0x66 | 0xE5 | 0x85 | 0x86 | 0x84 => {
                AddressingMode::ZeroPage
            }
            0x75 | 0x35 | 0x16 | 0xD5 | 0xD6 | 0x55 | 0xF6 | 0xB5 | 0xB4 | 0x56 | 0x15 | 0x36
            | 0x76 | 0xF5 | 0x95 | 0x94 => AddressingMode::ZeroPageX,
            0xB6 | 0x96 => AddressingMode::ZeroPageY,
            0x90 | 0xB0 | 0xF0 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70 => AddressingMode::Relative,
            0x6D | 0x2D | 0x0E | 0x2C | 0xCD | 0xEC | 0xCC | 0xCE | 0x4D | 0xEE | 0x4C | 0x20
            | 0xAD | 0xAE | 0xAC | 0x4E | 0x0D | 0x2E | 0x6E | 0xED | 0x8D | 0x8E | 0x8C => {
                AddressingMode::Absolute
            }
            0x7D | 0x3D | 0x1E | 0xDD | 0xDE | 0x5D | 0xFE | 0xBD | 0xBC | 0x5E | 0x1D | 0x3E
            | 0x7E | 0xFD | 0x9D => AddressingMode::AbsoluteX,
            0x79 | 0x39 | 0xD9 | 0x59 | 0xB9 | 0xBE | 0x19 | 0xF9 | 0x99 => {
                AddressingMode::AbsoluteY
            }
            0x6C => AddressingMode::Indirect,
            0x61 | 0x21 | 0xC1 | 0x41 | 0xA1 | 0x01 | 0xE1 | 0x81 => {
                AddressingMode::IndexedIndirectX
            }
            0x71 | 0x31 | 0xD1 | 0x51 | 0xB1 | 0x11 | 0xF1 | 0x91 => {
                AddressingMode::IndirectIndexedY
            }
            _ => panic!("invalid opcode: {}", opcode),
        }
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
            AddressingMode::IndexedIndirectX => self.indexed_indirect_addr(),
            AddressingMode::IndirectIndexedY => self.indirect_indexed_addr(),
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
