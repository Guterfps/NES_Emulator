use core::panic;

use super::joypad::JoyPad;
use super::memory::MemAccess;
use super::ppu::Ppu;
use super::rom::Rom;

pub struct Bus<'call> {
    cpu_vram: [u8; VRAM_SIZE],
    prg_rom: Vec<u8>,
    ppu: Ppu,
    joy_pad: JoyPad,
    cycles: usize,
    gameloop_callback: Box<dyn FnMut(&Ppu, &mut JoyPad) + 'call>,
}

const VRAM_SIZE: usize = 2048;
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const MASK_11_BITS: u16 = 0b0000_0111_1111_1111;
const PRG_ROM_START_ADDR: u16 = 0x8000;
const PRG_ROM_END_ADDR: u16 = 0xFFFF;
const PRG_ROM_PAGE_SIZE: u16 = 0x4000;

const PPU_CTRL_REG: u16 = 0x2000;
const PPU_MASK_REG: u16 = 0x2001;
const PPU_STATUS_REG: u16 = 0x2002;
const PPU_OAM_ADDR_REG: u16 = 0x2003;
const PPU_OAM_DATA_REG: u16 = 0x2004;
const PPU_SCROLL_REG: u16 = 0x2005;
const PPU_ADDR_REG: u16 = 0x2006;
const PPU_DATA_REG: u16 = 0x2007;
const PPU_REG_MIRROR_START: u16 = 0x2008;
const PPU_REG_MIRROR_ADDR_DOWN_MASK: u16 = 0b0010_0000_0000_0111;

const PPU_OAM_DMA_REG: u16 = 0x4014;

const JOYPAD_ADDR: u16 = 0x4016;

const PPU_CPU_CYCLES_RATIO: u8 = 3;

const PAGE_SIZE: usize = 256;
const BYTE_SIZE: u8 = 8;

impl<'a> Bus<'a> {
    pub fn new<'call, F>(mut rom: Rom, gameloop_cb: F) -> Bus<'call>
    where
        F: FnMut(&Ppu, &mut JoyPad) + 'call,
    {
        Bus {
            cpu_vram: [0; VRAM_SIZE],
            prg_rom: rom.take_prg_rom(),
            ppu: Ppu::new(rom.take_chr_rom(), rom.get_mirroring()),
            joy_pad: JoyPad::new(),
            cycles: 0,
            gameloop_callback: Box::from(gameloop_cb),
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;

        for _ in 0..(cycles * PPU_CPU_CYCLES_RATIO) {
            let nmi_before = self.ppu.is_nmi_interrupt();
            self.ppu.tick();
            let nmi_after = self.ppu.is_nmi_interrupt();

            if !nmi_before && nmi_after {
                (self.gameloop_callback)(&self.ppu, &mut self.joy_pad);
            }
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.take_nmi_interrupt()
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut rom_addr = addr - PRG_ROM_START_ADDR;
        if self.prg_rom.len() == PRG_ROM_PAGE_SIZE as usize {
            rom_addr &= PRG_ROM_PAGE_SIZE - 1;
        }
        self.prg_rom[rom_addr as usize]
    }
}

impl MemAccess for Bus<'_> {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & MASK_11_BITS;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_CTRL_REG | PPU_MASK_REG | PPU_OAM_ADDR_REG | PPU_SCROLL_REG | PPU_ADDR_REG
            | PPU_OAM_DMA_REG => {
                // panic!("Attempt to read from write only PPU address {:x}", addr)
                0
            }
            PPU_STATUS_REG => self.ppu.read_status(),
            PPU_DATA_REG => self.ppu.read_data(),
            PPU_OAM_DATA_REG => self.ppu.read_oam_data(),
            PPU_REG_MIRROR_START..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & PPU_REG_MIRROR_ADDR_DOWN_MASK;
                self.mem_read(mirror_down_addr)
            }
            JOYPAD_ADDR => self.joy_pad.read(),
            PRG_ROM_START_ADDR..=PRG_ROM_END_ADDR => self.read_prg_rom(addr),
            _ => {
                println!("memory not supported yet at: {:x}", addr);
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
            PPU_CTRL_REG => self.ppu.write_to_ctrl(data),
            PPU_MASK_REG => self.ppu.write_to_mask(data),
            PPU_STATUS_REG => panic!("attempt to write to PPU status reg"),
            PPU_OAM_ADDR_REG => self.ppu.write_to_oam_addr(data),
            PPU_OAM_DATA_REG => self.ppu.write_to_oam_data(data),
            PPU_SCROLL_REG => self.ppu.write_to_scroll(data),
            PPU_ADDR_REG => self.ppu.write_to_ppu_addr(data),
            PPU_DATA_REG => self.ppu.write_to_data(data),
            PPU_REG_MIRROR_START..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & PPU_REG_MIRROR_ADDR_DOWN_MASK;
                self.mem_write(mirror_down_addr, data);
            }
            PPU_OAM_DMA_REG => {
                let mut buffer: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
                let hi = (data as u16) << BYTE_SIZE;

                for (i, byte) in buffer.iter_mut().enumerate() {
                    *byte = self.mem_read(hi + i as u16);
                }

                self.ppu.write_to_oam_dma(&buffer);
            }
            JOYPAD_ADDR => self.joy_pad.write(data),
            PRG_ROM_START_ADDR..=PRG_ROM_END_ADDR => {
                panic!("Attempt to write to Cartridge ROM at: {:x}", addr);
            }
            _ => {
                println!("memory not supported yet at: {:x}", addr);
            }
        }
    }
}
