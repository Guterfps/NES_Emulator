use super::{VRAM_ADDR, control_reg::AddressInc};

pub struct InternalRegs {
    v: u16,
    t: u16,
    x: u8,
    w: bool,
}

pub const COARSE_X_SCROLL: u16 = 0b0000_0000_0001_1111;
pub const COARSE_Y_SCROLL: u16 = 0b0000_0011_1110_0000;
pub const NAMETABLE_SELECT: u16 = 0b0000_1100_0000_0000;
pub const FINE_Y_SCROLL: u16 = 0b0111_0000_0000_0000;

const BIT_Z: u16 = 0b0100_0000_0000_0000;

impl InternalRegs {
    pub fn new() -> Self {
        InternalRegs {
            v: 0,
            t: 0,
            x: 0,
            w: false,
        }
    }

    pub fn ctrl_write(&mut self, data: u8) {
        self.t = assign_bits(self.t, (data as u16) << 10, NAMETABLE_SELECT);
    }

    pub fn status_read(&mut self) {
        self.w = false;
    }

    pub fn scroll_write(&mut self, data: u8) {
        if !self.w {
            // first write
            self.t = assign_bits(self.t, (data as u16) >> 3, COARSE_X_SCROLL);
            self.x = data & 0b111;
        } else {
            // second write
            self.t = assign_bits(self.t, (data as u16) << 2, COARSE_Y_SCROLL);
            self.t = assign_bits(self.t, (data as u16) << 12, FINE_Y_SCROLL);
        }

        self.togle_w();
    }

    pub fn addr_write(&mut self, data: u8) {
        if !self.w {
            // first write
            self.t = assign_bits(self.t, (data as u16) << 8, 0b0011_1111 << 8);
            self.t &= !BIT_Z;
        } else {
            // second write
            self.t = assign_bits(self.t, data as u16, 0b1111_1111);
            self.v = self.t;
        }

        self.togle_w();
    }

    pub fn data_read_write(&mut self, inc_bit: bool) {
        self.v = self.v.wrapping_add(if inc_bit {
            AddressInc::GoingDown as u16
        } else {
            AddressInc::GoingAcross as u16
        });
    }

    pub fn get_w(&self) -> bool {
        self.w
    }

    pub fn coarse_x_inc(&mut self) {
        if (self.v & COARSE_X_SCROLL) == COARSE_X_SCROLL {
            self.v &= !COARSE_X_SCROLL;
            self.v ^= 0x0400; // switch horizontal nametable
        } else {
            self.v += 1;
        }
    }

    pub fn coarse_y_inc(&mut self) {
        if (self.v & FINE_Y_SCROLL) != FINE_Y_SCROLL {
            self.v += 0x1000;
        } else {
            self.v &= !FINE_Y_SCROLL;
            let mut y = (self.v & COARSE_Y_SCROLL) >> 5;
            if y == 29 {
                y = 0;
                self.v ^= 0x0800; // switch vertical nametable
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.v = (self.v & !COARSE_Y_SCROLL) | (y << 5);
        }
    }

    pub fn fetch_tile_addr(&self) -> u16 {
        VRAM_ADDR | (self.v & 0x0FFF)
    }

    pub fn fetch_attr_addr(&self) -> u16 {
        0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) | ((self.v >> 2) & 0x07)
    }

    pub fn get_scroll_x(&self) -> u16 {
        ((self.t & COARSE_X_SCROLL) << 3) | (self.x as u16)
    }

    pub fn get_scroll_y(&self) -> u16 {
        ((self.t & COARSE_Y_SCROLL) >> 2) | ((self.t & FINE_Y_SCROLL) >> 12)
    }

    pub fn get_nametable_select(&self) -> u16 {
        (self.t & NAMETABLE_SELECT) >> 10
    }

    pub fn dot_257(&mut self) {
        self.v = assign_bits(self.v, self.t, 0b0000_0100_0001_1111);
    }

    pub fn dot_280_to_304(&mut self) {
        self.v = assign_bits(self.v, self.t, 0b0111_1011_1110_0000);
    }

    pub fn get_v(&self) -> u16 {
        self.v
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    fn togle_w(&mut self) {
        self.w = !self.w;
    }
}

fn assign_bits(dest: u16, src: u16, mask: u16) -> u16 {
    (dest & !mask) | (src & mask)
}
