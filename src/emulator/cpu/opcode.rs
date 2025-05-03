use super::{AddressingMode, CPU6502};

pub struct OpCode {
    pub instraction: fn(&mut CPU6502, &AddressingMode),
    pub addr_mode: AddressingMode,
    pub cycles: u8,
}

pub const NUM_OF_OPCODES: usize = 256;

pub const OPCODE_TABLE: [OpCode; NUM_OF_OPCODES] = [
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.brk(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.aso(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.asl(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.aso(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.php(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.asl(mode),
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.anc(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.asl(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.aso(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bpl(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.aso(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.asl(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.aso(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.clc(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.aso(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ora(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.asl(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.aso(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.jsr(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rla(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bit(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rol(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rla(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.plp(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rol(mode),
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.anc(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bit(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rol(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rla(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bmi(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rla(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rol(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rla(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sec(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rla(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.and(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rol(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rla(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rti(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lse(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lsr(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lse(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.pha(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lsr(mode),
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.alr(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.jmp(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lsr(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lse(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bvc(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lse(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lsr(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lse(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cli(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lse(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.eor(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lsr(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lse(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rts(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rra(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ror(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rra(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.pla(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ror(mode),
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.arr(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.jmp(mode),
        addr_mode: AddressingMode::Indirect,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ror(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rra(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bvs(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rra(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ror(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rra(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sei(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rra(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.adc(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ror(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.rra(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sta(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sax(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sty(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sta(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.stx(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sax(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dey(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.txa(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.xaa(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sty(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sta(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.stx(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sax(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bcc(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sta(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.axa(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sty(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sta(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.stx(mode),
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sax(mode),
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.tya(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sta(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.txs(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.tas(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.say(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sta(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.xas(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.axa(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldy(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldx(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lax(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldy(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldx(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lax(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.tay(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.tax(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lax(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldy(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldx(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lax(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bcs(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lax(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldy(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldx(mode),
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lax(mode),
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.clv(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.tsx(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.las(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldy(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lda(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ldx(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.lax(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cpy(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dcm(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cpy(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dec(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dcm(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.iny(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dex(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.axs(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cpy(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dec(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dcm(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.bne(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dcm(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dec(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dcm(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cld(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dcm(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cmp(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dec(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.dcm(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cpx(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ins(mode),
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cpx(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.inc(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ins(mode),
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.inx(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.cpx(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.inc(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ins(mode),
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.beq(mode),
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.hlt(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ins(mode),
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.inc(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ins(mode),
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sed(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ins(mode),
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.nop(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.sbc(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.inc(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: |cpu: &mut CPU6502, mode: &AddressingMode| cpu.ins(mode),
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
];
