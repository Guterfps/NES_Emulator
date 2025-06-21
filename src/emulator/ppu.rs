mod address_reg;
mod control_reg;
mod internal_regs;
mod mask_reg;
pub mod render;
mod scroll_reg;
mod status_reg;

use core::panic;

use super::rom::Mirroring;
use address_reg::AddressReg;
use control_reg::*;
use internal_regs::*;
use mask_reg::MaskReg;
use render::frame::Frame;
use scroll_reg::ScrollReg;
use status_reg::{SPRITE_0_HIT_FLAG, StatusReg};

pub struct Ppu {
    chr_rom: Vec<u8>,
    screen: Frame,
    palette_table: [u8; PALETTE_TABLE_SIZE],
    vram: [u8; VRAM_SIZE],
    oam_data: [u8; OAM_DATA_SIZE],
    oam_cache: [u8; OAM_CACHE_SIZE],
    oam_cache_len: u8,
    mirroring: Mirroring,
    ctrl_reg: ControlReg,
    mask_reg: MaskReg,
    status_reg: StatusReg,
    oam_addr_reg: u8,
    scroll_reg: ScrollReg,
    addr_reg: AddressReg,
    internal_regs: InternalRegs,
    internal_data_buf: u8,
    scanline: u16,
    cycles: usize,
    nmi_interrupt: Option<u8>,
    is_odd_frame: bool,
}

const PALETTE_TABLE_SIZE: usize = 32;
const VRAM_SIZE: usize = 2048;
const OAM_DATA_SIZE: usize = 256;
const OAM_CACHE_SIZE: usize = 8;

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

const VISIBLE_SCANLINES: u16 = 239;
const SCANLINES_PER_FRAME: u16 = 262;
const CYCLES_PER_SCANLINE: usize = 341;
const VERTICAL_BLANKING_LINES: u16 = 241;
const DOT_256_IN_SCANLINE: usize = 256;
const DOT_257_IN_SCANLINE: usize = 257;
const DOT_280_IN_SCANLINE: usize = 280;
const DOT_304_IN_SCANLINE: usize = 304;
const DOT_328_IN_SCANLINE: usize = 328;
const PRE_RENDER_SCANLINE: u16 = 261;

const FLIP_HORIZONTAL: u8 = 0b0100_0000;
const FLIP_VERTICAL: u8 = 0b1000_0000;

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Ppu {
            chr_rom,
            mirroring,
            screen: Frame::new(),
            vram: [0; VRAM_SIZE],
            oam_data: [0; OAM_DATA_SIZE],
            oam_cache: [0; OAM_CACHE_SIZE],
            oam_cache_len: 0,
            palette_table: [0; PALETTE_TABLE_SIZE],
            ctrl_reg: ControlReg::new(),
            mask_reg: MaskReg::new(),
            status_reg: StatusReg::new(),
            oam_addr_reg: 0,
            scroll_reg: ScrollReg::new(),
            addr_reg: AddressReg::new(),
            internal_regs: InternalRegs::new(),
            internal_data_buf: 0,
            scanline: 0,
            cycles: 0,
            nmi_interrupt: None,
            is_odd_frame: false,
        }
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        let prev_nmi_status = self.ctrl_reg.gen_vblank_nmi();
        self.ctrl_reg.update(value);
        self.internal_regs.ctrl_write(value);
        if !prev_nmi_status && self.ctrl_reg.gen_vblank_nmi() && self.status_reg.is_in_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask_reg.update(value);
    }

    pub fn read_status(&mut self) -> u8 {
        let data = self.status_reg.get();
        self.status_reg.reset_vblank();
        self.internal_regs.status_read();
        data
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
        // self.scroll_reg.write(value, self.internal_regs.get_w());
        self.internal_regs.scroll_write(value);
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        // self.addr_reg.update(value, self.internal_regs.get_w());
        self.internal_regs.addr_write(value);
    }

    pub fn read_data(&mut self) -> u8 {
        let res = self.read_vram(self.internal_regs.get_v());
        self.handle_data_internal_regs();

        res
    }

    pub fn write_to_data(&mut self, value: u8) {
        self.write_to_vram(value);
        self.handle_data_internal_regs();
    }

    fn write_to_vram(&mut self, value: u8) {
        // let addr = self.addr_reg.get();
        let addr = self.internal_regs.get_v() & 0x3FFF;

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
            PALETTES_ADDR..MIRRORS_ADDR => {
                self.palette_table[Self::get_palette_table_index(addr)] = value;
            }
            _ => panic!("unexpected access to mirrored space {addr}"),
        }
        // self.increment_vram_addr();
    }

    fn read_vram(&mut self, addr: u16) -> u8 {
        let mask_addr = addr & 0x3FFF;
        // let addr = self.addr_reg.get();
        // self.increment_vram_addr();

        match mask_addr {
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
            PALETTES_ADDR..MIRRORS_ADDR => self.palette_table[Self::get_palette_table_index(addr)],
            _ => panic!("unexpected access to mirrored space {addr}"),
        }
    }

    fn handle_data_internal_regs(&mut self) {
        self.internal_regs.data_read_write(self.ctrl_reg.addr_inc());
    }

    fn get_palette_table_index(addr: u16) -> usize {
        let mut index = addr & (PALETTE_TABLE_SIZE - 1) as u16;

        // mirrors of 0x3F00 | 0x3F04 | 0x3F08 | 0x3F0C
        if let 0x10 | 0x14 | 0x18 | 0x1C = index {
            index -= 0x10;
        }

        index as usize
    }

    pub fn write_to_oam_dma(&mut self, data: &[u8; OAM_DATA_SIZE]) {
        for val in data {
            self.write_to_oam_data(*val);
        }
    }

    pub fn tick(&mut self) {
        match self.scanline {
            0..=VISIBLE_SCANLINES => {
                if self.is_rendering() {
                    self.non_vblank_scanlines();
                }

                if (self.cycles <= 256) && self.is_sprite_0_hit(self.cycles) {
                    self.status_reg.set_sprite_zero_hit();
                }
            }
            240 => {}
            VERTICAL_BLANKING_LINES => {
                if self.cycles == 1 {
                    self.status_reg.set_vblank();
                    self.status_reg.unset_sprite_zero_hit();

                    if self.ctrl_reg.gen_vblank_nmi() {
                        self.nmi_interrupt = Some(1);
                    }
                }
            }
            242..PRE_RENDER_SCANLINE => {}
            PRE_RENDER_SCANLINE => {
                if self.is_rendering() {
                    self.non_vblank_scanlines();
                    if (self.cycles >= DOT_280_IN_SCANLINE) && (self.cycles <= DOT_304_IN_SCANLINE)
                    {
                        self.internal_regs.dot_280_to_304();
                    }
                }
                if self.cycles == 1 {
                    self.nmi_interrupt = None;
                    self.status_reg.unset_sprite_zero_hit();
                    self.status_reg.reset_vblank();
                }
            }
            SCANLINES_PER_FRAME => {
                self.scanline = 0;
                self.cycles = if self.is_odd_frame { 0 } else { 1 };
                self.is_odd_frame = !self.is_odd_frame;
            }
            _ => unreachable!(),
        }

        self.cycles += 1;
        if self.cycles >= CYCLES_PER_SCANLINE {
            self.cycles = 0;
            self.scanline += 1;
        }
    }

    fn non_vblank_scanlines(&mut self) {
        if (self.cycles != 0)
            && ((self.cycles >= DOT_328_IN_SCANLINE) || (self.cycles <= DOT_256_IN_SCANLINE))
            && ((self.cycles & (8 - 1)) == 0)
        {
            self.internal_regs.coarse_x_inc();
        }
        if self.cycles == DOT_256_IN_SCANLINE {
            self.internal_regs.coarse_y_inc();
        }
        if self.cycles == DOT_257_IN_SCANLINE {
            self.internal_regs.dot_257();
        }
    }

    fn is_rendering(&self) -> bool {
        self.mask_reg.show_sprites() | self.mask_reg.show_background()
    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool {
        let y = self.oam_data[0] as usize;
        let x = self.oam_data[3] as usize;

        (y == self.scanline as usize) && (cycle >= x) && (self.mask_reg.show_sprites())
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

    pub fn take_nmi_interrupt(&mut self) -> Option<u8> {
        self.nmi_interrupt.take()
    }

    pub fn is_nmi_interrupt(&self) -> bool {
        self.nmi_interrupt.is_some()
    }

    fn render_background(&mut self) -> u16 {
        let x = (self.cycles - 1) as u16;
        let fine_x = (self.internal_regs.get_x() as u16 + x) & (8 - 1);
        let v = self.internal_regs.get_v();
        let mut res = 0;

        let tile_addr = self.internal_regs.fetch_tile_addr();
        let attr_addr = self.internal_regs.fetch_attr_addr();

        let pattern_addr = (self.read_vram(tile_addr) as u16 * 16 + ((v >> 12) & 0x7))
            | self.ctrl_reg.bknd_pattern_addr();

        let mut palette_addr = (self.read_vram(pattern_addr) >> (7 ^ fine_x)) & 1;
        palette_addr |= ((self.read_vram(pattern_addr + 8) >> (7 ^ fine_x)) & 1) << 1;

        if palette_addr != 0 {
            let attr = self.read_vram(attr_addr);
            res = palette_addr as u16 | (((attr as u16 >> ((v >> 4) & 4 | v & 2)) & 0x3) << 2);
        }

        res
    }

    fn render_sprites(&mut self, bg_addr: u16) -> u16 {
        let x = self.cycles as u16 - 1;
        let y = self.scanline;
        let mut palette_addr = 0;
        let len = self.ctrl_reg.sprite_size();
        let mut j = 0;

        while palette_addr == 0 {
            let i = self.oam_cache[j as usize] as usize;
            let tile_x = self.oam_data[i + 3] as u16;

            if (x > tile_x) && ((x - tile_x) < 8) {
                let tile = self.oam_data[i + 1] as u16;
                let tile_y = self.oam_data[i] as u16 + 1;
                let attr = self.oam_data[i + 2];
                let mut x_off = (x - tile_x) & (8 - 1);
                let mut y_off = (y - tile_y) & (len as u16 - 1);

                if (attr & FLIP_HORIZONTAL) == 0 {
                    x_off ^= 7;
                }
                if (attr & FLIP_VERTICAL) == 0 {
                    y_off ^= len as u16 - 1;
                }

                let mut tile_addr;

                if self.ctrl_reg.sprite_size() == 16 {
                    y_off = (y_off & 7) | ((y_off & 8) << 1);
                    tile_addr = (tile >> 1) * 32 + y_off;
                    tile_addr |= (tile & 1) << 12;
                } else {
                    tile_addr = tile * 16 + y_off + (self.ctrl_reg.sprt_pattern_addr());
                }

                palette_addr = (self.read_vram(tile_addr) as u16 >> x_off) & 1;
                palette_addr |= ((self.read_vram(tile_addr + 8) as u16 >> x_off) & 1) << 1;

                if palette_addr != 0 {
                    palette_addr |= 0x10 | ((attr as u16 & 0x3) << 2);

                    if !(self.status_reg.is_sprite_0_hit())
                        && (self.mask_reg.show_background())
                        && (i == 0)
                        && (palette_addr != 0)
                        && (bg_addr != 0)
                        && (x < 255)
                    {
                        self.status_reg.set_sprite_zero_hit();
                    }
                }

                j += 1;
            }
        }

        palette_addr
    }
}
