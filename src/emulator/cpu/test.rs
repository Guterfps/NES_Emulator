use super::*;

#[test]
fn lda_0xa9_immediate_load_data() {
    let mut cpu = CPU6502::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.accumulator, 0x05);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn lda_0xa9_zero_flag() {
    let mut cpu = CPU6502::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0b10);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn lda_0xa9_negative_flag() {
    let mut cpu = CPU6502::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0b1000_0000);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
}

#[test]
fn ldx_0xa2_immediate_load() {
    let mut cpu = CPU6502::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
    assert_eq!(cpu.indx_reg_x, 0x05);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn ldy_0xa0_immediate_load() {
    let mut cpu = CPU6502::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
    assert_eq!(cpu.indx_reg_y, 0x05);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn tax_0xaa_move_a_to_x() {
    let mut cpu = CPU6502::new();
    cpu.load(vec![0xaa, 0x00]);
    cpu.reset();
    cpu.accumulator = 10;
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 10);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn inx_0e8_increment_x() {
    let mut cpu = CPU6502::new();
    cpu.load_and_run(vec![0xe8, 0x00]);
    assert_eq!(cpu.indx_reg_x, 1);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn inx_0e8_increment_x_negative_flag() {
    let mut cpu = CPU6502::new();
    cpu.load(vec![0xe8, 0x00]);
    cpu.reset();
    cpu.indx_reg_x = 127;
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 128);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0b1000_0000)
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU6502::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.indx_reg_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU6502::new();
    cpu.load(vec![0xe8, 0xe8, 0x00]);
    cpu.reset();
    cpu.indx_reg_x = 0xff;
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 1)
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = CPU6502::new();
    cpu.memory.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
fn test_sta_0x85_store_accumulatore() {
    let mut cpu = CPU6502::new();
    cpu.load(vec![0x85, 0x10, 0x00]);
    cpu.reset();
    cpu.accumulator = 123;
    cpu.run();
    assert_eq!(cpu.memory.mem_read(0x10), 123);
}
