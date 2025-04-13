use super::*;

#[test]
fn lda_0xa9_immediate_load_data() {
    let mut cpu = CPU6502::new();
    cpu.interpret(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.accumulator, 0x05);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn lda_0xa9_zero_flag() {
    let mut cpu = CPU6502::new();
    cpu.interpret(vec![0xa9, 0x00, 0x00]);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0b10);
}

#[test]
fn lda_0xa9_negative_flag() {
    let mut cpu = CPU6502::new();
    cpu.interpret(vec![0xa9, 0xff, 0x00]);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0b1000_0000);
}

#[test]
fn tax_0xaa_move_a_to_x() {
    let mut cpu = CPU6502::new();
    cpu.accumulator = 10;
    cpu.interpret(vec![0xaa, 0x00]);
    assert_eq!(cpu.indx_reg_x, 10);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}
