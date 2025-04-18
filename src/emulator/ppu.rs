mod address_reg;
mod control_reg;
mod mask_reg;
mod scroll_reg;
mod status_reg;

use core::panic;

use super::rom::Mirroring;
use address_reg::AddressReg;
use control_reg::*;
use mask_reg::MaskReg;
use scroll_reg::ScrollReg;
use status_reg::StatusReg;

pub struct Ppu {
    chr_rom: Vec<u8>,
    palette_table: [u8; PALETTE_TABLE_SIZE],
    vram: [u8; VRAM_SIZE],
    oam_data: [u8; OAM_DATA_SIZE],
    mirroring: Mirroring,
    ctrl_reg: ControlReg,
    mask_reg: MaskReg,
    status_reg: StatusReg,
    oam_addr_reg: u8,
    scroll_reg: ScrollReg,
    addr_reg: AddressReg,
    oam_dma_reg: u8,
    internal_data_buf: u8,
}

const PALETTE_TABLE_SIZE: usize = 32;
const VRAM_SIZE: usize = 2048;
const OAM_DATA_SIZE: usize = 256;

const ROM_ADDR: u16 = 0x0000;
const VRAM_ADDR: u16 = 0x2000;
const VRAM_END_ADDR: u16 = 0x3000;
const PALETTES_ADDR: u16 = 0x3F00;
const MIRRORS_ADDR: u16 = 0x4000;

const MIRROR_DOWN_VRAM_ADDR_MASK: u16 = 0b1011_1111_1111_1111;
const NAME_TABLE_SIZE: u16 = 0x0400;
const NAME_TABLE_0: u16 = 0;
const NAME_TABLE_1: u16 = 1;
const NAME_TABLE_2: u16 = 2;
const NAME_TABLE_3: u16 = 3;

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Ppu {
            chr_rom,
            mirroring,
            vram: [0; VRAM_SIZE],
            oam_data: [0; OAM_DATA_SIZE],
            palette_table: [0; PALETTE_TABLE_SIZE],
            ctrl_reg: ControlReg::new(),
            mask_reg: MaskReg::new(),
            status_reg: StatusReg::new(),
            oam_addr_reg: 0,
            scroll_reg: ScrollReg::new(),
            addr_reg: AddressReg::new(),
            oam_dma_reg: 0,
            internal_data_buf: 0,
        }
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        self.ctrl_reg.update(value);
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask_reg.update(value);
    }

    pub fn read_status(&mut self) -> u8 {
        self.addr_reg.reset_latch();
        self.status_reg.get()
    }

    pub fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr_reg = value;
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr_reg as usize]
    }

    pub fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr_reg as usize] = value;
        self.oam_addr_reg = self.oam_addr_reg.wrapping_add(1);
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        if self.addr_reg.get_latch() {
            self.scroll_reg.update(scroll_reg::Mode::X, value);
        } else {
            self.scroll_reg.update(scroll_reg::Mode::Y, value);
        }

        self.addr_reg.toggle_latch();
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr_reg.update(value);
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.addr_reg.get();

        match addr {
            ROM_ADDR..VRAM_ADDR => {
                println!("attempt to write to rom space: {addr}")
            }
            VRAM_ADDR..VRAM_END_ADDR => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            VRAM_END_ADDR..PALETTES_ADDR => {
                panic!("addr space 0x3000..0x3f00 not expected to be used")
            }
            // mirrors of 0x3F00 | 0x3F04 | 0x3F08 | 0x3F0C
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let mirror_addr = addr - 0x10;
                self.palette_table[mirror_addr as usize] = value;
            }
            PALETTES_ADDR..MIRRORS_ADDR => {
                self.palette_table[(addr - PALETTES_ADDR) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {addr}"),
        }
        self.increment_vram_addr();
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr_reg.get();
        self.increment_vram_addr();

        match addr {
            ROM_ADDR..VRAM_ADDR => {
                let res = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                res
            }
            VRAM_ADDR..VRAM_END_ADDR => {
                let res = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                res
            }
            VRAM_END_ADDR..PALETTES_ADDR => {
                panic!("addr space 0x3000..0x3f00 not expected to be used")
            }
            // mirrors of 0x3F00 | 0x3F04 | 0x3F08 | 0x3F0C
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let mirror_addr = addr - 0x10;
                self.palette_table[mirror_addr as usize]
            }
            PALETTES_ADDR..MIRRORS_ADDR => self.palette_table[(addr - PALETTES_ADDR) as usize],
            _ => panic!("unexpected access to mirrored space {addr}"),
        }
    }

    pub fn write_to_oam_dma(&mut self, data: &[u8; OAM_DATA_SIZE]) {
        for val in data {
            self.write_to_oam_data(*val);
        }
    }

    fn increment_vram_addr(&mut self) {
        self.addr_reg.increment(self.ctrl_reg.vram_addr_increment());
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & MIRROR_DOWN_VRAM_ADDR_MASK;
        let vram_indx = mirrored_vram - VRAM_ADDR;
        let name_table = vram_indx / NAME_TABLE_SIZE;

        use Mirroring::*;
        match (&self.mirroring, name_table) {
            (Vertical, NAME_TABLE_2) | (Vertical, NAME_TABLE_3) => {
                vram_indx - (2 * NAME_TABLE_SIZE)
            }
            (Horizontal, NAME_TABLE_1) | (Horizontal, NAME_TABLE_2) => vram_indx - NAME_TABLE_SIZE,
            (Horizontal, NAME_TABLE_3) => vram_indx - (2 * NAME_TABLE_SIZE),
            _ => vram_indx,
        }
    }
}
