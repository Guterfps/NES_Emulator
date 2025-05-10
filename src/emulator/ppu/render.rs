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

const SPRITE_SIZE: usize = 4;
const SPRITE_INDEX_BYTE: usize = 1;
const SPRITE_ATTR_BYTE: usize = 2;
const SPRITE_X_POS_BYTE: usize = 3;

const SPRITE_FLIP_HOR: u8 = 0b0100_0000;
const SPRITE_FLIP_VER: u8 = 0b1000_0000;
const SPRITE_PALETTE: u8 = 0b0000_0011;
const NUM_OF_SPRITE_PALETTES: usize = 4;

pub fn render(ppu: &Ppu, frame: &mut Frame) {
    draw_backgound(ppu, frame);
    draw_sprites(ppu, frame);
}

fn draw_backgound(ppu: &Ppu, frame: &mut Frame) {
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
                    0 => SYSTEM_PALLETE[ppu.palette_table[0] as usize],
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

fn draw_sprites(ppu: &Ppu, frame: &mut Frame) {
    for i in (0..ppu.oam_data.len()).step_by(SPRITE_SIZE).rev() {
        let tile_idx = ppu.oam_data[i + SPRITE_INDEX_BYTE] as u16;
        let tile_x = ppu.oam_data[i + SPRITE_X_POS_BYTE] as usize;
        let tile_y = ppu.oam_data[i] as usize;
        let tile_attr = ppu.oam_data[i + SPRITE_ATTR_BYTE];

        let flip_ver = (tile_attr & SPRITE_FLIP_VER) != 0;
        let flip_hor = (tile_attr & SPRITE_FLIP_HOR) != 0;

        let palette_idx = tile_attr & SPRITE_PALETTE;
        let sprite_palette = sprite_palette(ppu, palette_idx);

        let bank = ppu.ctrl_reg.sprt_pattern_addr() as u16;

        let rom_idx = bank as usize + tile_idx as usize * TILE_SIZE;
        let tile = &ppu.chr_rom[(rom_idx)..(rom_idx + TILE_SIZE)];

        for y in 0..TILE_HIGHT {
            let mut upper = tile[y];
            let mut lower = tile[y + TILE_WIDTH];

            for x in (0..TILE_WIDTH).rev() {
                let val = ((1 & lower) << 1) | (1 & upper);
                upper >>= 1;
                lower >>= 1;

                let mut color = true;
                let rgb = match val {
                    0 => {
                        color = false;
                        (0, 0, 0)
                    }
                    1 => SYSTEM_PALLETE[sprite_palette[1] as usize],
                    2 => SYSTEM_PALLETE[sprite_palette[2] as usize],
                    3 => SYSTEM_PALLETE[sprite_palette[3] as usize],
                    _ => panic!("imposible"),
                };

                if color {
                    match (flip_hor, flip_ver) {
                        (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                        (true, false) => {
                            frame.set_pixel(tile_x + TILE_WIDTH - 1 - x, tile_y + y, rgb)
                        }
                        (false, true) => {
                            frame.set_pixel(tile_x + x, tile_y + TILE_HIGHT - 1 - y, rgb)
                        }
                        (true, true) => frame.set_pixel(
                            tile_x + TILE_WIDTH - 1 - x,
                            tile_y + TILE_HIGHT - 1 - y,
                            rgb,
                        ),
                    }
                }
            }
        }
    }
}

fn sprite_palette(ppu: &Ppu, pal_idx: u8) -> [u8; NUM_OF_SPRITE_PALETTES] {
    let start = SPRITE_PALETTE as usize + (pal_idx as usize) * SPRITE_SIZE;

    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}
