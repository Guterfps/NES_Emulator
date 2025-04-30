use super::{AddressingMode, CPU6502};

pub struct OpCode {
    pub instraction: fn(&mut CPU6502, &AddressingMode),
    pub addr_mode: AddressingMode,
    pub cycles: u8,
}

pub const NUM_OF_OPCODES: usize = 256;

pub const OPCODE_TABLE: [OpCode; NUM_OF_OPCODES] = [
    OpCode {
        instraction: CPU6502::brk,
        addr_mode: AddressingMode::Implicit,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::aso,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::asl,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::aso,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::php,
        addr_mode: AddressingMode::Implicit,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::asl,
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::anc,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::asl,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::aso,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::bpl,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::aso,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::asl,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::aso,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::clc,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::aso,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ora,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::asl,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::aso,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::jsr,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::rla,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::bit,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::rol,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::rla,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::plp,
        addr_mode: AddressingMode::Implicit,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::rol,
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::anc,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::bit,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::rol,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::rla,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::bmi,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::rla,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::rol,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::rla,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::sec,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::rla,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::and,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::rol,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::rla,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::rti,
        addr_mode: AddressingMode::Implicit,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::lse,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::lsr,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::lse,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::pha,
        addr_mode: AddressingMode::Implicit,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lsr,
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::alr,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::jmp,
        addr_mode: AddressingMode::Absolute,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lsr,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::lse,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::bvc,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::lse,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lsr,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::lse,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::cli,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lse,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::eor,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lsr,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::lse,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::rts,
        addr_mode: AddressingMode::Implicit,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::rra,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::ror,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::rra,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::pla,
        addr_mode: AddressingMode::Implicit,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::ror,
        addr_mode: AddressingMode::Accumulator,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::arr,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::jmp,
        addr_mode: AddressingMode::Indirect,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ror,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::rra,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::bvs,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::rra,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ror,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::rra,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::sei,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::rra,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::adc,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ror,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::rra,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sta,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sax,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::sty,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::sta,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::stx,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::sax,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::dey,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::txa,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::xaa,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sty,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::sta,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::stx,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::sax,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::bcc,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sta,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::axa,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::sty,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::sta,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::stx,
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::sax,
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::tya,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sta,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::txs,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::tas,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::say,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::sta,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::xas,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::axa,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::ldy,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::ldx,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lax,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::ldy,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::ldx,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::lax,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::tay,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::tax,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lax,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::ldy,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ldx,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lax,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::bcs,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::lax,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::ldy,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ldx,
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lax,
        addr_mode: AddressingMode::ZeroPageY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::clv,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::tsx,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::las,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ldy,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lda,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::ldx,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::lax,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::cpy,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::dcm,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::cpy,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::dec,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::dcm,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::iny,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::dex,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::axs,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::cpy,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::dec,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::dcm,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::bne,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::dcm,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::dec,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::dcm,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::cld,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::dcm,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::cmp,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::dec,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::dcm,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::cpx,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::ins,
        addr_mode: AddressingMode::IndexedIndirectX,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::cpx,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 3,
    },
    OpCode {
        instraction: CPU6502::inc,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::ins,
        addr_mode: AddressingMode::ZeroPage,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::inx,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::Immediate,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::cpx,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::Absolute,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::inc,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::ins,
        addr_mode: AddressingMode::Absolute,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::beq,
        addr_mode: AddressingMode::Relative,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 5,
    },
    OpCode {
        instraction: CPU6502::hlt,
        addr_mode: AddressingMode::Implicit,
        cycles: 0,
    },
    OpCode {
        instraction: CPU6502::ins,
        addr_mode: AddressingMode::IndirectIndexedY,
        cycles: 8,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::inc,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::ins,
        addr_mode: AddressingMode::ZeroPageX,
        cycles: 6,
    },
    OpCode {
        instraction: CPU6502::sed,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::Implicit,
        cycles: 2,
    },
    OpCode {
        instraction: CPU6502::ins,
        addr_mode: AddressingMode::AbsoluteY,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::nop,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::sbc,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 4,
    },
    OpCode {
        instraction: CPU6502::inc,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
    OpCode {
        instraction: CPU6502::ins,
        addr_mode: AddressingMode::AbsoluteX,
        cycles: 7,
    },
];
