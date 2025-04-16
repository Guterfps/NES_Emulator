use super::memory::MemAccess;
use super::rom::Rom;

pub struct Bus {
    cpu_vram: [u8; VRAM_SIZE],
    rom: Rom,
}

const VRAM_SIZE: usize = 2048;
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const MASK_11_BITS: u16 = 0b0000_0111_1111_1111;
const PPU_REGISTERS_MASK: u16 = 0b0010_0000_0000_0111;
const PRG_ROM_START_ADDR: u16 = 0x8000;
const PRG_ROM_END_ADDR: u16 = 0xFFFF;
const PRG_ROM_PAGE_SIZE: u16 = 0x4000;

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; VRAM_SIZE],
            rom,
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut rom_addr = addr - PRG_ROM_START_ADDR;
        if self.rom.prg_size() == PRG_ROM_PAGE_SIZE as usize {
            rom_addr &= PRG_ROM_PAGE_SIZE - 1;
        }
        self.rom.read_prg(rom_addr)
    }
}

impl MemAccess for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & MASK_11_BITS;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & PPU_REGISTERS_MASK;
                todo!("PPU is not supported yet")
            }
            PRG_ROM_START_ADDR..=PRG_ROM_END_ADDR => self.read_prg_rom(addr),
            _ => {
                println!("memory not supported yet at: {}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & MASK_11_BITS;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & PPU_REGISTERS_MASK;
                todo!("PPU is not supported yet")
            }
            PRG_ROM_START_ADDR..=PRG_ROM_END_ADDR => {
                panic!("Attempt to write to Cartridge ROM at: {}", addr);
            }
            _ => {
                println!("memory not supported yet at: {}", addr);
            }
        }
    }
}
