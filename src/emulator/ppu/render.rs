pub mod frame;
pub mod pallete_table;

use core::panic;

use super::{PALETTE_TABLE_SIZE, Ppu};
use frame::Frame;
use pallete_table::SYSTEM_PALLETE;

const TILE_SIZE: usize = 16;
const TILE_WIDTH: usize = 8;
const TILE_HIGHT: usize = 8;
const NUM_OF_BG_PALLETES: usize = 4;
const COLOR_BITS: u8 = 0b11;

pub fn render(ppu: &Ppu, frame: &mut Frame) {
    let bank = ppu.ctrl_reg.bknd_pattern_addr();

    for i in 0..0x03c0 {
        let tile = ppu.vram[i];
        let tile_col = i & (PALETTE_TABLE_SIZE - 1);
        let tile_row = i / PALETTE_TABLE_SIZE;
        let tile_indx = bank as usize + tile as usize * TILE_SIZE;
        let tile = &ppu.chr_rom[tile_indx..(tile_indx + TILE_SIZE)];
        let pallete = bg_pallete(ppu, tile_col, tile_row);

        for y in 0..TILE_HIGHT {
            let mut upper = tile[y];
            let mut lower = tile[y + TILE_WIDTH];

            for x in (0..TILE_WIDTH).rev() {
                let val = ((1 & lower) << 1) | (1 & upper);
                upper >>= 1;
                lower >>= 1;

                let rgb = match val {
                    0 => SYSTEM_PALLETE[pallete[0] as usize],
                    1 => SYSTEM_PALLETE[pallete[1] as usize],
                    2 => SYSTEM_PALLETE[pallete[2] as usize],
                    3 => SYSTEM_PALLETE[pallete[3] as usize],
                    _ => panic!("imposible"),
                };

                frame.set_pixel(tile_col * TILE_WIDTH + x, tile_row * TILE_HIGHT + y, rgb);
            }
        }
    }
}

fn bg_pallete(ppu: &Ppu, tile_col: usize, tile_row: usize) -> [u8; NUM_OF_BG_PALLETES] {
    let attr_table_idx = (tile_row / NUM_OF_BG_PALLETES) * 8 + tile_col / NUM_OF_BG_PALLETES;
    let attr_byte = ppu.vram[0x03c0 + attr_table_idx];

    let pallet_idx = match (
        (tile_col & (NUM_OF_BG_PALLETES - 1)) >> 1,
        (tile_row & (NUM_OF_BG_PALLETES - 1)) >> 1,
    ) {
        (0, 0) => attr_byte & COLOR_BITS,
        (1, 0) => (attr_byte >> 2) & COLOR_BITS,
        (0, 1) => (attr_byte >> 4) & COLOR_BITS,
        (1, 1) => (attr_byte >> 6) & COLOR_BITS,
        (_, _) => panic!("should not happen"),
    };

    let pallete_start = 1 + (pallet_idx as usize) * NUM_OF_BG_PALLETES;

    [
        ppu.palette_table[0],
        ppu.palette_table[pallete_start],
        ppu.palette_table[pallete_start + 1],
        ppu.palette_table[pallete_start + 2],
    ]
}
