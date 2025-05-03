mod opcode;
mod status;

use core::panic;

use super::bus::Bus;
use super::memory::MemAccess;
use opcode::OPCODE_TABLE;
use status::*;

pub struct CPU6502<'a> {
    status_reg: StatusReg,
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    indx_reg_x: u8,
    indx_reg_y: u8,
    page_crossed: bool,
    branch_taken: bool,
    bus: Bus<'a>,
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

const BIT_0: u8 = 0b0000_0001;
const BIT_7: u8 = 0b1000_0000;

const HI_BYTE: u16 = 0xFF00;

impl<'a> CPU6502<'a> {
    pub fn new(bus: Bus<'a>) -> Self {
        CPU6502 {
            status_reg: StatusReg::new(),
            program_counter: PC_START_ADDR,
            stack_pointer: PAGE_SIZE,
            accumulator: 0,
            indx_reg_x: 0,
            indx_reg_y: 0,
            page_crossed: false,
            branch_taken: false,
            bus,
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for (i, byte) in program.iter().enumerate() {
            self.bus.mem_write(PC_START_ADDR + i as u16, *byte);
        }
        self.bus.mem_write_u16(RESET_LOCATION, PC_START_ADDR);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU6502),
    {
        let mut break_status: bool = false;

        while !break_status {
            if let Some(_nmi) = self.bus.poll_nmi_status() {
                self.interrupt_nmi();
            }

            callback(self);

            let op_code = self.bus.mem_read(self.program_counter);
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
        self.program_counter = self.bus.mem_read_u16(RESET_LOCATION);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
    }

    pub fn mem_read(&mut self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn interrupt_nmi(&mut self) {
        self.push_stack_u16(self.program_counter);

        let mut stack_status = self.status_reg.clone();
        stack_status.unset_flag(BREAK_COMMAND);
        stack_status.set_flag(ONE_FLAG);
        self.push_stack(stack_status.status);

        self.status_reg.set_flag(INTERRUPT_DISABLE);

        let handler_addr = self.bus.mem_read_u16(NON_MASKABLE_INTER_HNDLER_ADDR);
        self.program_counter = handler_addr;
        self.bus.tick(2);
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
        let val = self.bus.mem_read(addr);

        let reg = match reg_name {
            RegName::X => &mut self.indx_reg_x,
            RegName::Y => &mut self.indx_reg_y,
            RegName::A => &mut self.accumulator,
        };

        *reg = val;

        Self::update_zero_flag(*reg, &mut self.status_reg);
        Self::update_negative_flag(*reg, &mut self.status_reg);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, self.accumulator);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, self.indx_reg_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, self.indx_reg_y);
    }

    // transfer ops
    fn tax(&mut self, _mode: &AddressingMode) {
        self.indx_reg_x = self.accumulator;

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    fn tay(&mut self, _mode: &AddressingMode) {
        self.indx_reg_y = self.accumulator;

        Self::update_zero_flag(self.indx_reg_y, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_y, &mut self.status_reg);
    }

    fn txa(&mut self, _mode: &AddressingMode) {
        self.accumulator = self.indx_reg_x;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
    }

    fn tya(&mut self, _mode: &AddressingMode) {
        self.accumulator = self.indx_reg_y;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
    }

    // stack ops
    fn tsx(&mut self, _mode: &AddressingMode) {
        self.indx_reg_x = self.stack_pointer;

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    fn txs(&mut self, _mode: &AddressingMode) {
        self.stack_pointer = self.indx_reg_x;
    }

    fn pha(&mut self, _mode: &AddressingMode) {
        self.push_stack(self.accumulator);
    }

    fn php(&mut self, _mode: &AddressingMode) {
        self.push_stack(self.status_reg.status | BREAK_COMMAND | ONE_FLAG);
    }

    fn pla(&mut self, _mode: &AddressingMode) {
        self.accumulator = self.pop_stack();

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
    }

    fn plp(&mut self, _mode: &AddressingMode) {
        self.status_reg.status = self.pop_stack();
        self.status_reg.set_flag(ONE_FLAG);
        self.status_reg.unset_flag(BREAK_COMMAND);
    }

    fn push_stack(&mut self, val: u8) {
        let mem_addr = STACK_ADDR + self.stack_pointer as u16;
        self.bus.mem_write(mem_addr, val);
        self.stack_pointer -= 1;
    }

    fn pop_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        let mem_addr = STACK_ADDR + self.stack_pointer as u16;
        self.bus.mem_read(mem_addr)
    }

    // logical ops
    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr);
        self.accumulator &= byte;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr);
        self.accumulator ^= byte;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr);
        self.accumulator |= byte;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr);
        let res = self.accumulator & byte;

        let overflow = (byte & OVERFLOW_FLAG) != 0;

        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_overflow_flag(overflow, &mut self.status_reg);
        Self::update_negative_flag(byte, &mut self.status_reg);
    }

    // Arithmetic ops
    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr);

        self.add_with_carry(byte);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr);

        self.add_with_carry(!byte);
    }

    fn add_with_carry(&mut self, val: u8) {
        let sum =
            self.accumulator as u16 + val as u16 + self.status_reg.get_flag(CARRY_FLAG) as u16;
        let is_carry = sum > u8::MAX as u16;
        let res = sum as u8;
        let overflow = ((res ^ self.accumulator) & (res ^ val) & BIT_7) != 0;

        Self::update_carry_flag(is_carry, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_overflow_flag(overflow, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);

        self.accumulator = res;
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr).wrapping_add(1);

        self.bus.mem_write(addr, byte);

        Self::update_zero_flag(byte, &mut self.status_reg);
        Self::update_negative_flag(byte, &mut self.status_reg);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let byte = self.bus.mem_read(addr).wrapping_sub(1);

        self.bus.mem_write(addr, byte);

        Self::update_zero_flag(byte, &mut self.status_reg);
        Self::update_negative_flag(byte, &mut self.status_reg);
    }

    fn inx(&mut self, _mode: &AddressingMode) {
        self.indx_reg_x = self.indx_reg_x.wrapping_add(1);

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    fn dex(&mut self, _mode: &AddressingMode) {
        self.indx_reg_x = self.indx_reg_x.wrapping_sub(1);

        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    fn iny(&mut self, _mode: &AddressingMode) {
        self.indx_reg_y = self.indx_reg_y.wrapping_add(1);

        Self::update_zero_flag(self.indx_reg_y, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_y, &mut self.status_reg);
    }

    fn dey(&mut self, _mode: &AddressingMode) {
        self.indx_reg_y = self.indx_reg_y.wrapping_sub(1);

        Self::update_zero_flag(self.indx_reg_y, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_y, &mut self.status_reg);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let val = self.get_val_by_mode(mode);
        let carry = (val & BIT_7) != 0;
        let res = val << 1;

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);

        self.set_val_by_mode(mode, res);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let val = self.get_val_by_mode(mode);
        let carry = (val & BIT_0) != 0;
        let res = val >> 1;

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);

        self.set_val_by_mode(mode, res);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let val = self.get_val_by_mode(mode);
        let carry = (val & BIT_7) != 0;
        let res = val.rotate_left(1);

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);

        self.set_val_by_mode(mode, res);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let val = self.get_val_by_mode(mode);
        let carry = (val & BIT_0) != 0;
        let res = val.rotate_right(1);

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);

        self.set_val_by_mode(mode, res);
    }

    // compare ops
    fn cmp(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.accumulator);
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.indx_reg_x);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.indx_reg_y);
    }

    fn compare(&mut self, mode: &AddressingMode, compare_reg: u8) {
        let addr = self.get_operand_addr(mode);
        let val = self.bus.mem_read(addr);
        let res = compare_reg.wrapping_sub(val);
        let carry = compare_reg >= val;

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);
    }

    // branch ops
    fn bcc(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(CARRY_FLAG) == 0);
    }

    fn bcs(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(CARRY_FLAG) != 0);
    }

    fn beq(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(ZERO_FLAG) != 0);
    }

    fn bne(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(ZERO_FLAG) == 0);
    }

    fn bpl(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(NEGATIVE_FLAG) == 0);
    }

    fn bmi(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(NEGATIVE_FLAG) != 0);
    }

    fn bvc(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(OVERFLOW_FLAG) == 0);
    }

    fn bvs(&mut self, mode: &AddressingMode) {
        self.branch(mode, self.status_reg.get_flag(OVERFLOW_FLAG) != 0);
    }

    fn branch(&mut self, mode: &AddressingMode, cond: bool) {
        if cond {
            let addr = self.get_operand_addr(mode);
            let val = self.bus.mem_read(addr) as i8;
            let sum = self
                .program_counter
                .wrapping_add(Self::num_of_address_mode_bytes(mode))
                .wrapping_add(val as u16);

            self.branch_taken = true;
            self.page_crossed = Self::page_cross(self.program_counter, sum);

            self.program_counter = sum;
        }
    }

    // jump ops
    fn jmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);

        if let AddressingMode::Indirect = mode {
            // 6502 bug with page boundry
            let indirect_ref = if (addr & PAGE_SIZE as u16) == PAGE_SIZE as u16 {
                let lo = self.bus.mem_read(addr);
                let hi = self.bus.mem_read(addr & 0xFF00);
                u16::from_le_bytes([lo, hi])
            } else {
                self.bus.mem_read_u16(addr)
            };

            self.program_counter = indirect_ref;
        } else {
            self.program_counter = addr;
        };
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);

        self.push_stack_u16(self.program_counter + Self::num_of_address_mode_bytes(mode) - 1);

        self.program_counter = addr;
    }

    fn rts(&mut self, _mode: &AddressingMode) {
        self.program_counter = self.pop_stack_u16() + 1;
    }

    fn brk(&mut self, _mode: &AddressingMode) {
        self.push_stack_u16(self.program_counter + 1);
        self.push_stack(self.status_reg.status | BREAK_COMMAND | ONE_FLAG);
        self.status_reg.set_flag(INTERRUPT_DISABLE);

        let handler_addr = self.bus.mem_read_u16(BRK_INTR_HANDLER_ADDR);
        self.program_counter = handler_addr;
    }

    fn rti(&mut self, _mode: &AddressingMode) {
        self.status_reg.status = self.pop_stack();
        self.status_reg.set_flag(ONE_FLAG);
        self.status_reg.unset_flag(BREAK_COMMAND);
        self.program_counter = self.pop_stack_u16();
    }

    fn push_stack_u16(&mut self, val: u16) {
        let bytes = val.to_le_bytes();

        for byte in bytes.iter().rev() {
            self.push_stack(*byte);
        }
    }

    fn pop_stack_u16(&mut self) -> u16 {
        let mut bytes = [0u8; 2];

        for byte in bytes.iter_mut() {
            *byte = self.pop_stack();
        }

        u16::from_le_bytes(bytes)
    }

    // flag ops
    fn clc(&mut self, _mode: &AddressingMode) {
        self.status_reg.unset_flag(CARRY_FLAG);
    }

    fn sec(&mut self, _mode: &AddressingMode) {
        self.status_reg.set_flag(CARRY_FLAG);
    }

    fn cld(&mut self, _mode: &AddressingMode) {
        self.status_reg.unset_flag(DECIMAL_MODE);
    }

    fn sed(&mut self, _mode: &AddressingMode) {
        self.status_reg.set_flag(DECIMAL_MODE);
    }

    fn cli(&mut self, _mode: &AddressingMode) {
        self.status_reg.unset_flag(INTERRUPT_DISABLE);
    }

    fn sei(&mut self, _mode: &AddressingMode) {
        self.status_reg.set_flag(INTERRUPT_DISABLE);
    }

    fn clv(&mut self, _mode: &AddressingMode) {
        self.status_reg.unset_flag(OVERFLOW_FLAG);
    }

    // other ops
    fn nop(&mut self, _mode: &AddressingMode) {}

    // unofficial opcodes
    fn aso(&mut self, mode: &AddressingMode) {
        self.asl(mode);
        self.ora(mode);
    }

    fn rla(&mut self, mode: &AddressingMode) {
        self.rol(mode);
        self.and(mode);
    }

    fn lse(&mut self, mode: &AddressingMode) {
        self.lsr(mode);
        self.eor(mode);
    }

    fn rra(&mut self, mode: &AddressingMode) {
        self.ror(mode);
        self.adc(mode);
    }

    fn axs(&mut self, mode: &AddressingMode) {
        let res = self.accumulator & self.indx_reg_x;
        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, res);
    }

    fn lax(&mut self, mode: &AddressingMode) {
        self.lda(mode);
        self.ldx(mode);
    }

    fn dcm(&mut self, mode: &AddressingMode) {
        self.dec(mode);
        self.cmp(mode);
    }

    fn ins(&mut self, mode: &AddressingMode) {
        self.inc(mode);
        self.sbc(mode);
    }

    fn alr(&mut self, mode: &AddressingMode) {
        self.and(mode);
        let carry = (self.accumulator & BIT_0) != 0;
        let res = self.accumulator >> 1;

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);

        self.accumulator = res;
    }

    fn arr(&mut self, mode: &AddressingMode) {
        self.and(mode);
        let res = self.accumulator.rotate_right(1);
        let carry = ((res >> 6) & BIT_0) != 0;
        let overflow = ((res >> 6) ^ (res >> 5)) != 0;

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_overflow_flag(overflow, &mut self.status_reg);
        Self::update_zero_flag(res, &mut self.status_reg);
        Self::update_negative_flag(res, &mut self.status_reg);

        self.accumulator = res;
    }

    fn xaa(&mut self, mode: &AddressingMode) {
        self.txa(mode);
        self.and(mode);
    }

    fn oal(&mut self, mode: &AddressingMode) {
        self.accumulator |= 0xEE;
        self.and(mode);
        self.tax(mode);
    }

    fn sax(&mut self, mode: &AddressingMode) {
        let res = self.accumulator & self.indx_reg_x;
        let addr = self.get_operand_addr(mode);
        let val = self.bus.mem_read(addr);
        let carry = res >= val;
        self.indx_reg_x = res.wrapping_sub(val);

        Self::update_carry_flag(carry, &mut self.status_reg);
        Self::update_zero_flag(self.indx_reg_x, &mut self.status_reg);
        Self::update_negative_flag(self.indx_reg_x, &mut self.status_reg);
    }

    fn skb(&mut self, _mode: &AddressingMode) {
        self.program_counter += 1;
    }

    fn skw(&mut self, _mode: &AddressingMode) {
        self.program_counter += 2
    }

    fn hlt(&mut self, _mode: &AddressingMode) {
        std::process::exit(0);
    }

    fn tas(&mut self, mode: &AddressingMode) {
        let mut res = self.accumulator & self.indx_reg_x;
        self.stack_pointer = res;
        let un_indxd_addr = self.bus.mem_read(self.program_counter + 1).wrapping_add(1);
        res &= un_indxd_addr;

        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, res);
    }

    fn say(&mut self, mode: &AddressingMode) {
        let un_indexd_addr = self.bus.mem_read(self.program_counter + 1).wrapping_add(1);
        let res = un_indexd_addr & self.indx_reg_y;

        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, res);
    }

    fn xas(&mut self, mode: &AddressingMode) {
        let un_indexd_addr = self.bus.mem_read(self.program_counter + 1).wrapping_add(1);
        let res = un_indexd_addr & self.indx_reg_x;

        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, res);
    }

    fn axa(&mut self, mode: &AddressingMode) {
        let un_indexd_addr = self.bus.mem_read(self.program_counter + 1).wrapping_add(1);
        let res = un_indexd_addr & self.accumulator & self.indx_reg_x;

        let addr = self.get_operand_addr(mode);
        self.bus.mem_write(addr, res);
    }

    fn anc(&mut self, mode: &AddressingMode) {
        self.and(mode);
        let carry = (self.accumulator & NEGATIVE_FLAG) != 0;

        Self::update_carry_flag(carry, &mut self.status_reg);
    }

    fn las(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_addr(mode);
        let val = self.bus.mem_read(addr);
        let res = val & self.stack_pointer;

        self.indx_reg_x = res;
        self.accumulator = res;
        self.stack_pointer = res;

        Self::update_zero_flag(self.accumulator, &mut self.status_reg);
        Self::update_negative_flag(self.accumulator, &mut self.status_reg);
    }

    fn get_val_by_mode(&mut self, mode: &AddressingMode) -> u8 {
        if let AddressingMode::Accumulator = mode {
            self.accumulator
        } else {
            let addr = self.get_operand_addr(mode);
            self.bus.mem_read(addr)
        }
    }

    fn set_val_by_mode(&mut self, mode: &AddressingMode, val: u8) {
        if let AddressingMode::Accumulator = mode {
            self.accumulator = val;
        } else {
            let addr = self.get_operand_addr(mode);
            self.bus.mem_write(addr, val);
        }
    }

    fn update_carry_flag(carry: bool, status: &mut StatusReg) {
        if carry {
            status.set_flag(CARRY_FLAG);
        } else {
            status.unset_flag(CARRY_FLAG);
        }
    }

    fn update_zero_flag(reg: u8, status: &mut StatusReg) {
        if reg == 0 {
            status.set_flag(ZERO_FLAG);
        } else {
            status.unset_flag(ZERO_FLAG);
        }
    }

    fn update_overflow_flag(overflow: bool, status: &mut StatusReg) {
        if overflow {
            status.set_flag(OVERFLOW_FLAG);
        } else {
            status.unset_flag(OVERFLOW_FLAG);
        }
    }

    fn update_negative_flag(reg: u8, status: &mut StatusReg) {
        if is_negative(reg) {
            status.set_flag(NEGATIVE_FLAG);
        } else {
            status.unset_flag(NEGATIVE_FLAG);
        }
    }

    fn move_pc(&mut self, mode: &AddressingMode, pc_before_inst: u16) {
        if pc_before_inst == self.program_counter {
            self.program_counter += Self::num_of_address_mode_bytes(mode);
        }
    }

    fn num_of_address_mode_bytes(mode: &AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            Implicit | Accumulator => 0,
            Relative | Immediate | ZeroPage | ZeroPageX | ZeroPageY | IndexedIndirectX
            | IndirectIndexedY => 1,
            Absolute | AbsoluteX | AbsoluteY | Indirect => 2,
        }
    }

    fn op_code_instraction(&mut self, op_code: u8) -> bool {
        let mut is_break = false;
        let opcode = &OPCODE_TABLE[op_code as usize];
        // let addres_mode = Self::get_opcode_address_mode(op_code);
        let pc_before_inst = self.program_counter;

        println!(
            "opcode: {:#04x}, address mode: {:?}",
            op_code, opcode.addr_mode
        );

        if op_code == 0x00 {
            is_break = true;
        }

        self.page_crossed = false;
        self.branch_taken = false;

        (opcode.instraction)(self, &opcode.addr_mode);

        self.bus
            .tick(opcode.cycles + self.page_crossed as u8 + self.branch_taken as u8);

        // match op_code {
        //     // load and store ops
        //     0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&addres_mode),
        //     0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&addres_mode),
        //     0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&addres_mode),
        //     0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&addres_mode),
        //     0x86 | 0x96 | 0x8E => self.stx(&addres_mode),
        //     0x84 | 0x94 | 0x8C => self.sty(&addres_mode),
        //     // transfer ops
        //     0xAA => self.tax(&addres_mode),
        //     0xA8 => self.tay(&addres_mode),
        //     0x8A => self.txa(&addres_mode),
        //     0x98 => self.tya(&addres_mode),
        //     // stack ops
        //     0xBA => self.tsx(&addres_mode),
        //     0x9A => self.txs(&addres_mode),
        //     0x48 => self.pha(&addres_mode),
        //     0x08 => self.php(&addres_mode),
        //     0x68 => self.pla(&addres_mode),
        //     0x28 => self.plp(&addres_mode),
        //     // logical ops
        //     0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&addres_mode),
        //     0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&addres_mode),
        //     0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(&addres_mode),
        //     0x24 | 0x2C => self.bit(&addres_mode),
        //     // Arithmetic ops
        //     0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&addres_mode),
        //     0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&addres_mode),
        //     0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&addres_mode),
        //     0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&addres_mode),
        //     0xE8 => self.inx(&addres_mode),
        //     0xCA => self.dex(&addres_mode),
        //     0xC8 => self.iny(&addres_mode),
        //     0x88 => self.dey(&addres_mode),
        //     // shift ops
        //     0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(&addres_mode),
        //     0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.lsr(&addres_mode),
        //     0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rol(&addres_mode),
        //     0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.ror(&addres_mode),
        //     // compare ops
        //     0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(&addres_mode),
        //     0xE0 | 0xE4 | 0xEC => self.cpx(&addres_mode),
        //     0xC0 | 0xC4 | 0xCC => self.cpy(&addres_mode),
        //     // branch ops
        //     0x90 => self.bcc(&addres_mode),
        //     0xB0 => self.bcs(&addres_mode),
        //     0xF0 => self.beq(&addres_mode),
        //     0xD0 => self.bne(&addres_mode),
        //     0x10 => self.bpl(&addres_mode),
        //     0x30 => self.bmi(&addres_mode),
        //     0x50 => self.bvc(&addres_mode),
        //     0x70 => self.bvs(&addres_mode),
        //     // jump ops
        //     0x4C | 0x6C => self.jmp(&addres_mode),
        //     0x20 => self.jsr(&addres_mode),
        //     0x60 => self.rts(&addres_mode),
        //     // 0x00 => self.brk(&addres_mode),
        //     0x40 => self.rti(&addres_mode),
        //     // flag ops
        //     0x18 => self.clc(&addres_mode),
        //     0xD8 => self.cld(&addres_mode),
        //     0x58 => self.cli(&addres_mode),
        //     0xB8 => self.clv(&addres_mode),
        //     0x38 => self.sec(&addres_mode),
        //     0xF8 => self.sed(&addres_mode),
        //     0x78 => self.sei(&addres_mode),
        //     // other
        //     0xEA => self.nop(&addres_mode),
        //     // unofficial ops
        //     0x0F | 0x1F | 0x1B | 0x07 | 0x17 | 0x03 | 0x13 => self.aso(&addres_mode),
        //     0x2F | 0x3F | 0x3B | 0x27 | 0x37 | 0x23 | 0x33 => self.rla(&addres_mode),
        //     0x4F | 0x5F | 0x5B | 0x47 | 0x57 | 0x43 | 0x53 => self.lse(&addres_mode),
        //     0x6F | 0x7F | 0x7B | 0x67 | 0x77 | 0x63 | 0x73 => self.rra(&addres_mode),
        //     0x8F | 0x87 | 0x97 | 0x83 => self.axs(&addres_mode),
        //     0xAF | 0xBF | 0xA7 | 0xB7 | 0xA3 | 0xB3 => self.lax(&addres_mode),
        //     0xCF | 0xDF | 0xDB | 0xC7 | 0xD7 | 0xC3 | 0xD3 => self.dcm(&addres_mode),
        //     0xEF | 0xFF | 0xFB | 0xE7 | 0xF7 | 0xE3 | 0xF3 => self.ins(&addres_mode),
        //     0x4B => self.alr(&addres_mode),
        //     0x6B => self.arr(&addres_mode),
        //     0x8B => self.xaa(&addres_mode),
        //     0xAB => self.oal(&addres_mode),
        //     0xCB => self.sax(&addres_mode),
        //     0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => self.nop(&addres_mode),
        //     0x80 | 0x82 | 0xC2 | 0xE2 | 0x04 | 0x14 | 0x34 | 0x44 | 0x54 | 0x64 | 0x74 | 0xD4
        //     | 0xF4 | 0x89 => self.skb(&addres_mode),
        //     0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => self.skw(&addres_mode),
        //     0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => {
        //         self.hlt(&addres_mode)
        //     }
        //     0x9B => self.tas(&addres_mode),
        //     0x9C => self.say(&addres_mode),
        //     0x9E => self.xas(&addres_mode),
        //     0x9F | 0x93 => self.axa(&addres_mode),
        //     0x2B | 0x0B => self.anc(&addres_mode),
        //     0xBB => self.las(&addres_mode),
        //     0xEB => self.sbc(&AddressingMode::Immediate),

        //     0x00 => is_break = true,
        //     // _ => println!("unknown opcode: {}", op_code),
        // };

        self.move_pc(&opcode.addr_mode, pc_before_inst);
        is_break
    }

    // fn get_opcode_address_mode(opcode: u8) -> AddressingMode {
    //     match opcode {
    //         0x00 | 0x18 | 0xD8 | 0xB8 | 0x58 | 0xCA | 0x88 | 0xE8 | 0xC8 | 0xEA | 0x48 | 0x08
    //         | 0x68 | 0x28 | 0x40 | 0x60 | 0x38 | 0xF8 | 0x78 | 0xAA | 0xA8 | 0xBA | 0x8A | 0x9A
    //         | 0x98 | 0x80 | 0x82 | 0xC2 | 0xE2 | 0x04 | 0x14 | 0x34 | 0x44 | 0x54 | 0x64 | 0x74
    //         | 0xD4 | 0xF4 | 0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2
    //         | 0xD2 | 0xF2 | 0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC | 0x1A | 0x3A | 0x5A
    //         | 0x7A | 0xDA | 0xFA | 0x89 => AddressingMode::Implicit,
    //         0x0A | 0x4A | 0x2A | 0x6A => AddressingMode::Accumulator,
    //         0xA9 | 0xA2 | 0xA0 | 0x09 | 0xE9 | 0x69 | 0x29 | 0xC9 | 0xE0 | 0xC0 | 0x49 | 0x0B
    //         | 0x2B | 0x4B | 0x6B | 0x8B | 0xAB | 0xCB | 0xEB => AddressingMode::Immediate,
    //         0x65 | 0x25 | 0x06 | 0x24 | 0xC5 | 0xE4 | 0xC4 | 0xC6 | 0x45 | 0xE6 | 0xA5 | 0xA6
    //         | 0xA4 | 0x46 | 0x05 | 0x26 | 0x66 | 0xE5 | 0x85 | 0x86 | 0x84 | 0x07 | 0x27 | 0x47
    //         | 0x67 | 0x87 | 0xA7 | 0xC7 | 0xE7 => AddressingMode::ZeroPage,
    //         0x75 | 0x35 | 0x16 | 0xD5 | 0xD6 | 0x55 | 0xF6 | 0xB5 | 0xB4 | 0x56 | 0x15 | 0x36
    //         | 0x76 | 0xF5 | 0x95 | 0x94 | 0x17 | 0x37 | 0x57 | 0x77 | 0xD7 | 0xF7 => {
    //             AddressingMode::ZeroPageX
    //         }
    //         0xB6 | 0x96 | 0x97 | 0xB7 => AddressingMode::ZeroPageY,
    //         0x90 | 0xB0 | 0xF0 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70 => AddressingMode::Relative,
    //         0x6D | 0x2D | 0x0E | 0x2C | 0xCD | 0xEC | 0xCC | 0xCE | 0x4D | 0xEE | 0x4C | 0x20
    //         | 0xAD | 0xAE | 0xAC | 0x4E | 0x0D | 0x2E | 0x6E | 0xED | 0x8D | 0x8E | 0x8C | 0x0F
    //         | 0x2F | 0x4F | 0x6F | 0x8F | 0xAF | 0xCF | 0xEF => AddressingMode::Absolute,
    //         0x7D | 0x3D | 0x1E | 0xDD | 0xDE | 0x5D | 0xFE | 0xBD | 0xBC | 0x5E | 0x1D | 0x3E
    //         | 0x7E | 0xFD | 0x9D | 0x1F | 0x3F | 0x5F | 0x7F | 0xDF | 0xFF | 0x9C => {
    //             AddressingMode::AbsoluteX
    //         }
    //         0x79 | 0x39 | 0xD9 | 0x59 | 0xB9 | 0xBE | 0x19 | 0xF9 | 0x99 | 0x1B | 0x3B | 0x5B
    //         | 0x7B | 0xBF | 0xDB | 0xFB | 0x9F | 0x9E | 0x9B | 0xBB => AddressingMode::AbsoluteY,
    //         0x6C => AddressingMode::Indirect,
    //         0x61 | 0x21 | 0xC1 | 0x41 | 0xA1 | 0x01 | 0xE1 | 0x81 | 0x03 | 0x23 | 0x43 | 0x63
    //         | 0x83 | 0xA3 | 0xC3 | 0xE3 => AddressingMode::IndexedIndirectX,
    //         0x71 | 0x31 | 0xD1 | 0x51 | 0xB1 | 0x11 | 0xF1 | 0x91 | 0x13 | 0x33 | 0x53 | 0x73
    //         | 0xB3 | 0xD3 | 0xF3 | 0x93 => AddressingMode::IndirectIndexedY,
    //         // _ => panic!("invalid opcode: {:#04x}", opcode),
    //     }
    // }

    fn get_operand_addr(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implicit => panic!("Implicit mode"),
            AddressingMode::Accumulator => panic!("Accumulator mode"),
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
        self.program_counter
    }

    fn zero_page_addr(&mut self) -> u16 {
        let addr = self.bus.mem_read(self.program_counter);
        addr as u16
    }

    fn zero_page_x_addr(&mut self) -> u16 {
        let param = self.bus.mem_read(self.program_counter);
        let addr = self.indx_reg_x.wrapping_add(param);
        addr as u16
    }

    fn zero_page_y_addr(&mut self) -> u16 {
        let param = self.bus.mem_read(self.program_counter);
        let addr = self.indx_reg_y.wrapping_add(param);
        addr as u16
    }

    fn relative_addr(&self) -> u16 {
        self.program_counter
    }

    fn absolute_addr(&mut self) -> u16 {
        self.bus.mem_read_u16(self.program_counter)
    }

    fn absolute_x_addr(&mut self) -> u16 {
        let param = self.bus.mem_read_u16(self.program_counter);
        let addr = param.wrapping_add(self.indx_reg_x as u16);

        self.page_crossed = Self::page_cross(param, addr);
        addr
    }

    fn absolute_y_addr(&mut self) -> u16 {
        let param = self.bus.mem_read_u16(self.program_counter);
        let addr = param.wrapping_add(self.indx_reg_y as u16);

        self.page_crossed = Self::page_cross(param, addr);
        addr
    }

    fn indirect_addr(&mut self) -> u16 {
        self.bus.mem_read_u16(self.program_counter)
    }

    fn indexed_indirect_addr(&mut self) -> u16 {
        let param = self.bus.mem_read(self.program_counter);
        let peek1 = self
            .bus
            .mem_read(self.indx_reg_x.wrapping_add(param) as u16);
        let peek2 = self
            .bus
            .mem_read(self.indx_reg_x.wrapping_add(param).wrapping_add(1) as u16)
            as u16;
        let addr = peek1 as u16 + (peek2 << 8);

        addr
    }

    fn indirect_indexed_addr(&mut self) -> u16 {
        let param = self.bus.mem_read(self.program_counter);
        let peek1 = self.bus.mem_read(param as u16);
        let peek2 = self.bus.mem_read(param.wrapping_add(1) as u16) as u16;
        let tmp = peek1 as u16 + (peek2 << 8);
        let addr = tmp + self.indx_reg_y as u16;

        self.page_crossed = Self::page_cross(tmp, addr);

        addr
    }

    fn page_cross(arg: u16, res: u16) -> bool {
        (arg & HI_BYTE) != (res & HI_BYTE)
    }
}

fn is_negative(val: u8) -> bool {
    const NEGATIVE_BIT: u8 = 0b1000_0000;
    (val & NEGATIVE_BIT) != 0
}

#[cfg(test)]
mod test;
