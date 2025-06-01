pub mod frame;
pub mod pallete_table;
mod rect;

use core::panic;

use crate::emulator::rom::Mirroring;

use super::{NAME_TABLE_SIZE, PALETTE_TABLE_SIZE, Ppu, VRAM_ADDR};
use frame::Frame;
use pallete_table::SYSTEM_PALLETE;
use rect::Rect;

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
const SPRITE_PALETTE_MASK: u8 = 0b11;
const START_SPRITE_PALETTE_OFFSET: u8 = 0x11;
const NUM_OF_SPRITE_PALETTES: usize = 4;

const ATTRIBUTE_OFFSET: usize = 0x03C0;
const TWO_NAMETABLE_SIZE: u16 = NAME_TABLE_SIZE * 2;
const THREE_NAMETABLE_SIZE: u16 = NAME_TABLE_SIZE * 3;
const FIRST_TABLE_ADDR: u16 = VRAM_ADDR;
const SECOND_TABLE_ADDR: u16 = VRAM_ADDR + NAME_TABLE_SIZE;
const THIRD_TABLE_ADDR: u16 = VRAM_ADDR + TWO_NAMETABLE_SIZE;
const FORTH_TABLE_ADDR: u16 = VRAM_ADDR + THREE_NAMETABLE_SIZE;

const DISPLAY_WIDTH: usize = 256;
const DISPLAY_HIGHT: usize = 240;

pub fn render(ppu: &Ppu, frame: &mut Frame) {
    draw_backgound(ppu, frame);
    draw_sprites(ppu, frame);
}

fn draw_backgound(ppu: &Ppu, frame: &mut Frame) {
    let scroll_x = ppu.scroll_reg.scroll_x() as usize;
    let scroll_y = ppu.scroll_reg.scroll_y() as usize;
    let base_nametable = ppu.ctrl_reg.nametable_addr();

    // let (main_nt, right_nt, bottom_nt, bottom_right_nt) = get_nametables(ppu, base_nametable);

    let (main_nametable, second_nametable) = match (&ppu.mirroring, base_nametable) {
        (Mirroring::Vertical, FIRST_TABLE_ADDR)
        | (Mirroring::Vertical, THIRD_TABLE_ADDR)
        | (Mirroring::Horizontal, FIRST_TABLE_ADDR)
        | (Mirroring::Horizontal, SECOND_TABLE_ADDR) => (
            &ppu.vram[0..NAME_TABLE_SIZE as usize],
            &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
        ),
        (Mirroring::Vertical, SECOND_TABLE_ADDR)
        | (Mirroring::Vertical, FORTH_TABLE_ADDR)
        | (Mirroring::Horizontal, THIRD_TABLE_ADDR)
        | (Mirroring::Horizontal, FORTH_TABLE_ADDR) => (
            &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
            &ppu.vram[0..NAME_TABLE_SIZE as usize],
        ),
        (_, _) => {
            panic!("not supported mirroring type {:?}", ppu.mirroring);
        }
    };

    render_name_table(
        ppu,
        frame,
        main_nametable,
        Rect::new(scroll_x, scroll_y, DISPLAY_WIDTH, DISPLAY_HIGHT),
        -(scroll_x as isize),
        -(scroll_y as isize),
    );

    if scroll_x > 0 {
        render_name_table(
            ppu,
            frame,
            second_nametable,
            Rect::new(0, 0, scroll_x, DISPLAY_HIGHT),
            (DISPLAY_WIDTH - scroll_x) as isize,
            0,
        );
    } else if scroll_y > 0 {
        render_name_table(
            ppu,
            frame,
            second_nametable,
            Rect::new(0, 0, DISPLAY_WIDTH, scroll_y),
            0,
            (DISPLAY_HIGHT - scroll_y) as isize,
        );
    }
    // if scroll_x > 0 {
    //     render_name_table(
    //         ppu,
    //         frame,
    //         right_nt,
    //         Rect::new(0, 0, scroll_x, DISPLAY_HIGHT),
    //         (DISPLAY_WIDTH - scroll_x) as isize,
    //         0,
    //     );
    // }
    // if scroll_y > 0 {
    //     render_name_table(
    //         ppu,
    //         frame,
    //         bottom_nt,
    //         Rect::new(0, 0, DISPLAY_WIDTH, scroll_y),
    //         0,
    //         (DISPLAY_HIGHT - scroll_y) as isize,
    //     );
    // }
    // if (scroll_x > 0) && (scroll_y > 0) {
    //     render_name_table(
    //         ppu,
    //         frame,
    //         bottom_right_nt,
    //         Rect::new(0, 0, scroll_x, scroll_y),
    //         (DISPLAY_WIDTH - scroll_x) as isize,
    //         (DISPLAY_HIGHT - scroll_y) as isize,
    //     );
    // }
}

fn bg_pallete(
    ppu: &Ppu,
    attribute_table: &[u8],
    tile_col: usize,
    tile_row: usize,
) -> [u8; NUM_OF_BG_PALLETES] {
    let attr_table_idx = (tile_row / NUM_OF_BG_PALLETES) * 8 + tile_col / NUM_OF_BG_PALLETES;
    let attr_byte = attribute_table[attr_table_idx];

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

fn get_nametables(ppu: &Ppu, base_addr: u16) -> (&[u8], &[u8], &[u8], &[u8]) {
    use Mirroring::*;

    match ppu.mirroring {
        Vertical => match base_addr {
            FIRST_TABLE_ADDR => (
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
            ),
            SECOND_TABLE_ADDR => (
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
            ),
            _ => panic!("invalid nametable address for vertical mirroring"),
        },
        Horizontal => match base_addr {
            FIRST_TABLE_ADDR => (
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
            ),
            SECOND_TABLE_ADDR => (
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
                &ppu.vram[NAME_TABLE_SIZE as usize..TWO_NAMETABLE_SIZE as usize],
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
                &ppu.vram[0..NAME_TABLE_SIZE as usize],
            ),
            _ => panic!("invalid nametable address for vertical mirroring"),
        },
        FourScreen => panic!("four screen not implemented yet"),
    }
}

fn draw_sprites(ppu: &Ppu, frame: &mut Frame) {
    for i in (0..ppu.oam_data.len()).step_by(SPRITE_SIZE).rev() {
        let tile_idx = ppu.oam_data[i + SPRITE_INDEX_BYTE] as u16;
        let tile_x = ppu.oam_data[i + SPRITE_X_POS_BYTE] as usize;
        let tile_y = ppu.oam_data[i] as usize;
        let tile_attr = ppu.oam_data[i + SPRITE_ATTR_BYTE];

        let flip_ver = (tile_attr & SPRITE_FLIP_VER) != 0;
        let flip_hor = (tile_attr & SPRITE_FLIP_HOR) != 0;

        let palette_idx = tile_attr & SPRITE_PALETTE_MASK;
        let sprite_palette = sprite_palette(ppu, palette_idx);

        let bank = ppu.ctrl_reg.sprt_pattern_addr();

        let rom_idx = bank as usize + tile_idx as usize * TILE_SIZE;
        let tile = &ppu.chr_rom[rom_idx..(rom_idx + TILE_SIZE)];

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
                    let (pixel_x, pixel_y) = match (flip_hor, flip_ver) {
                        (false, false) => (tile_x + x, tile_y + y),
                        (true, false) => (tile_x + TILE_WIDTH - 1 - x, tile_y + y),
                        (false, true) => (tile_x + x, tile_y + TILE_HIGHT - 1 - y),
                        (true, true) => (tile_x + TILE_WIDTH - 1 - x, tile_y + TILE_HIGHT - 1 - y),
                    };

                    if (pixel_x < DISPLAY_WIDTH) && (pixel_y < DISPLAY_HIGHT) {
                        frame.set_pixel(pixel_x, pixel_y, rgb);
                    }
                }
            }
        }
    }
}

fn sprite_palette(ppu: &Ppu, pal_idx: u8) -> [u8; NUM_OF_SPRITE_PALETTES] {
    let start = START_SPRITE_PALETTE_OFFSET as usize + (pal_idx as usize) * SPRITE_SIZE;

    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}

fn render_name_table(
    ppu: &Ppu,
    frame: &mut Frame,
    name_table: &[u8],
    view_port: Rect,
    shift_x: isize,
    shift_y: isize,
) {
    let bank = ppu.ctrl_reg.bknd_pattern_addr();
    let attribute_table = &name_table[ATTRIBUTE_OFFSET..NAME_TABLE_SIZE as usize];

    for i in 0..ATTRIBUTE_OFFSET {
        let tile_col = i & (PALETTE_TABLE_SIZE - 1);
        let tile_row = i / PALETTE_TABLE_SIZE;
        let tile_idx = name_table[i] as u16;
        let tile_offset = bank as usize + tile_idx as usize * TILE_SIZE;
        let tile = &ppu.chr_rom[tile_offset..(tile_offset + TILE_SIZE)];
        let palette = bg_pallete(ppu, attribute_table, tile_col, tile_row);

        for y in 0..TILE_HIGHT {
            let mut upper = tile[y];
            let mut lower = tile[y + TILE_WIDTH];

            for x in (0..TILE_WIDTH).rev() {
                let val = ((1 & lower) << 1) | (1 & upper);
                upper >>= 1;
                lower >>= 1;

                let rgb = match val {
                    0 => SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                    1 => SYSTEM_PALLETE[palette[1] as usize],
                    2 => SYSTEM_PALLETE[palette[2] as usize],
                    3 => SYSTEM_PALLETE[palette[3] as usize],
                    _ => panic!("imposible"),
                };

                let pixel_x = tile_col * TILE_WIDTH + x;
                let pixel_y = tile_row * TILE_HIGHT + y;

                if (pixel_x >= view_port.x1)
                    && (pixel_x < view_port.x2)
                    && (pixel_y >= view_port.y1)
                    && (pixel_y < view_port.y2)
                {
                    frame.set_pixel(
                        (pixel_x as isize + shift_x) as usize,
                        (pixel_y as isize + shift_y) as usize,
                        rgb,
                    );
                }
            }
        }
    }
}
