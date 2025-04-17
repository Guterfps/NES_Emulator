use sdl3::libc::SKF_AD_VLAN_TAG_PRESENT;

use crate::emulator::rom::Rom;

use super::*;

fn test_rom(program: &[u8]) -> Rom {
    const HEADER_SIZE: usize = 16;
    const PRG_PAGE_SIZE: usize = 0x4000;
    const CHR_PAGE_SIZE: usize = 0x2000;

    let nes_tag: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
    let prg_size: u8 = 1;
    let chr_size: u8 = 1;
    let flags6: u8 = 0;
    let flags7: u8 = 0;
    let flags8: u8 = 0;
    let flags9: u8 = 0;
    let flags10: u8 = 0;
    let bytes11_15: [u8; 5] = [0; 5];
    let mut prg_rom = [0u8; PRG_PAGE_SIZE];
    let mut chr_rom = [0u8; CHR_PAGE_SIZE];

    // set rom start address
    prg_rom[PRG_PAGE_SIZE - 3] = 0x80;
    prg_rom[PRG_PAGE_SIZE - 4] = 0x00;
    prg_rom[..program.len()].copy_from_slice(program);

    let mut rom = Vec::with_capacity(HEADER_SIZE + PRG_PAGE_SIZE + CHR_PAGE_SIZE);

    rom.extend_from_slice(&nes_tag);
    rom.push(prg_size);
    rom.push(chr_size);
    rom.push(flags6);
    rom.push(flags7);
    rom.push(flags8);
    rom.push(flags9);
    rom.push(flags10);
    rom.extend_from_slice(&bytes11_15);
    rom.extend_from_slice(&prg_rom);
    rom.extend_from_slice(&chr_rom);

    Rom::new(&rom).unwrap()
}

#[test]
fn lda_0xa9_immediate_load_data() {
    let rom = test_rom(&[0xa9, 0x05, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
    assert_eq!(cpu.accumulator, 0x05);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn lda_0xa9_zero_flag() {
    let rom = test_rom(&[0xa9, 0x00, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0b10);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn lda_0xa9_negative_flag() {
    let rom = test_rom(&vec![0xa9, 0xff, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0b1000_0000);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
}

#[test]
fn ldx_0xa2_immediate_load() {
    let rom = test_rom(&vec![0xa2, 0x05, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 0x05);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn ldy_0xa0_immediate_load() {
    let rom = test_rom(&vec![0xa0, 0x05, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
    assert_eq!(cpu.indx_reg_y, 0x05);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn tax_0xaa_move_a_to_x() {
    let rom = test_rom(&vec![0xaa, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.accumulator = 10;
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 10);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn inx_0e8_increment_x() {
    let rom = test_rom(&vec![0xe8, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 1);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0)
}

#[test]
fn inx_0e8_increment_x_negative_flag() {
    let rom = test_rom(&vec![0xe8, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.indx_reg_x = 127;
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 128);
    assert!((cpu.status_reg.status & ZERO_FLAG) == 0);
    assert!((cpu.status_reg.status & NEGATIVE_FLAG) == 0b1000_0000)
}

#[test]
fn test_5_ops_working_together() {
    let rom = test_rom(&vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let rom = test_rom(&vec![0xe8, 0xe8, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.indx_reg_x = 0xff;
    cpu.run();
    assert_eq!(cpu.indx_reg_x, 1)
}

#[test]
fn test_lda_from_bus() {
    let rom = test_rom(&vec![0xa5, 0x10, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.bus.mem_write(0x10, 0x55);
    cpu.reset();
    cpu.run();

    assert_eq!(cpu.accumulator, 0x55);
}

#[test]
fn test_sta_0x85_store_accumulatore() {
    let rom = test_rom(&vec![0x85, 0x10, 0x00]);
    let bus = Bus::new(rom);
    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.accumulator = 123;
    cpu.run();
    assert_eq!(cpu.bus.mem_read(0x10), 123);
}

// fn trace(cpu: &CPU6502) -> String {

// }

// #[test]
// fn test_format_trace() {
//     let mut bus = Bus::new(test_rom(&vec![]));
//     bus.mem_write(100, 0xa2);
//     bus.mem_write(101, 0x01);
//     bus.mem_write(102, 0xca);
//     bus.mem_write(103, 0x88);
//     bus.mem_write(104, 0x00);

//     let mut cpu = CPU6502::new(bus);
//     cpu.program_counter = 0x64;
//     cpu.accumulator = 1;
//     cpu.indx_reg_x = 2;
//     cpu.indx_reg_y = 3;
//     let mut result: Vec<String> = vec![];
//     cpu.run_with_callback(|cpu| {
//         result.push(trace(cpu));
//     });
//     assert_eq!(
//         "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
//         result[0]
//     );
//     assert_eq!(
//         "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
//         result[1]
//     );
//     assert_eq!(
//         "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
//         result[2]
//     );
// }

// #[test]
// fn test_format_mem_access() {
//     let mut bus = Bus::new(test_rom(&vec![]));
//     // ORA ($33), Y
//     bus.mem_write(100, 0x11);
//     bus.mem_write(101, 0x33);

//     //data
//     bus.mem_write(0x33, 00);
//     bus.mem_write(0x34, 04);

//     //target cell
//     bus.mem_write(0x400, 0xAA);

//     let mut cpu = CPU6502::new(bus);
//     cpu.program_counter = 0x64;
//     cpu.indx_reg_y = 0;
//     let mut result: Vec<String> = vec![];
//     cpu.run_with_callback(|cpu| {
//         result.push(trace(cpu));
//     });
//     assert_eq!(
//         "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
//         result[0]
//     );
// }

// #[test]
// fn dummy_reads_test() {
//     let program = std::fs::read("roms/tests/cpu_dummy_reads.nes").unwrap();
//     let rom = Rom::new(&program).unwrap();
//     let bus = Bus::new(rom);

//     let mut cpu = CPU6502::new(bus);
//     println!("program size: {}", program.len());
//     cpu.reset();
//     cpu.run();
// }
