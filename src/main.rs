mod emulator;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use emulator::bus::Bus;
use emulator::cpu::CPU6502;
use emulator::joypad::{self, JoyPad};
use emulator::ppu::Ppu;
use emulator::ppu::render;
use emulator::ppu::render::frame::Frame;
use emulator::rom::Rom;

// use emulator::cpu::trace;

use rand::Rng;
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::pixels::PixelFormat;
use sdl3::sys::pixels::SDL_PixelFormat;

#[macro_use]
extern crate lazy_static;

fn main() {
    // snake_game();
    // tiles();
    // nes_test();
    game_test();
}

fn nes_test() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Static Screen", 256 * 2, 240 * 2)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    let pixel_format = unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) };
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(pixel_format, 256, 240)
        .unwrap();

    let program = std::fs::read("roms/tests/nestest.nes").unwrap();
    let rom = Rom::new(&program).unwrap();

    let mut frame = Frame::new();

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, joypad::Buttons::Down);
    key_map.insert(Keycode::Up, joypad::Buttons::Up);
    key_map.insert(Keycode::Right, joypad::Buttons::Right);
    key_map.insert(Keycode::Left, joypad::Buttons::Left);
    key_map.insert(Keycode::Space, joypad::Buttons::Select);
    key_map.insert(Keycode::Return, joypad::Buttons::Start);
    key_map.insert(Keycode::A, joypad::Buttons::A);
    key_map.insert(Keycode::S, joypad::Buttons::B);

    let bus = Bus::new(rom, move |ppu: &Ppu, joypad: &mut JoyPad| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(button) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button(*button);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(button) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.unset_button(*button);
                    }
                }
                _ => {}
            }
        }
    });

    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    // cpu.program_counter = 0xC000;
    cpu.run_with_callback(move |cpu| {
        // println!("{}", trace::trace(cpu));
    });
}

fn game_test() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game", 256 * 2, 240 * 2)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    let pixel_format = unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) };
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(pixel_format, 256, 240)
        .unwrap();

    let program = std::fs::read("roms/games/super_mario.nes").unwrap();
    let rom = Rom::new(&program).unwrap();

    let mut frame = Frame::new();

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, joypad::Buttons::Down);
    key_map.insert(Keycode::Up, joypad::Buttons::Up);
    key_map.insert(Keycode::Right, joypad::Buttons::Right);
    key_map.insert(Keycode::Left, joypad::Buttons::Left);
    key_map.insert(Keycode::Space, joypad::Buttons::Select);
    key_map.insert(Keycode::Return, joypad::Buttons::Start);
    key_map.insert(Keycode::A, joypad::Buttons::A);
    key_map.insert(Keycode::S, joypad::Buttons::B);

    let mut time = Instant::now();
    const FRAME_RATE: f32 = 1.0 / 60.0;

    let bus = Bus::new(rom, move |ppu: &Ppu, joypad: &mut JoyPad| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(button) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button(*button);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(button) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.unset_button(*button);
                    }
                }
                _ => {}
            }
        }

        while time.elapsed().as_secs_f32() < FRAME_RATE {}
        time = Instant::now();
    });

    let mut cpu = CPU6502::new(bus);
    cpu.reset();
    cpu.run();
}

use emulator::ppu::render::pallete_table as palette;

fn show_tile(chr_rom: &Vec<u8>, bank: usize, tile_n: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    let bank = bank * 0x1000;

    let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

    for y in 0..=7 {
        let mut upper = tile[y];
        let mut lower = tile[y + 8];

        for x in (0..=7).rev() {
            let value = (1 & upper) << 1 | (1 & lower);
            upper >>= 1;
            lower >>= 1;
            let rgb = match value {
                0 => palette::SYSTEM_PALLETE[0x01],
                1 => palette::SYSTEM_PALLETE[0x23],
                2 => palette::SYSTEM_PALLETE[0x27],
                3 => palette::SYSTEM_PALLETE[0x30],
                _ => panic!("can't be"),
            };
            frame.set_pixel(x, y, rgb)
        }
    }

    frame
}

fn show_tile_bank(chr_rom: &Vec<u8>, bank: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    let mut tile_y = 0;
    let mut tile_x = 0;
    let bank = (bank * 0x1000) as usize;

    for tile_n in 0..255 {
        if tile_n != 0 && tile_n % 20 == 0 {
            tile_y += 10;
            tile_x = 0;
        }
        let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => palette::SYSTEM_PALLETE[0x01],
                    1 => palette::SYSTEM_PALLETE[0x23],
                    2 => palette::SYSTEM_PALLETE[0x27],
                    3 => palette::SYSTEM_PALLETE[0x30],
                    _ => panic!("can't be"),
                };
                frame.set_pixel(tile_x + x, tile_y + y, rgb)
            }
        }

        tile_x += 10;
    }
    frame
}

fn tiles() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile View", 256 * 3, 240 * 3)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let pixel_format = unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) };
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(pixel_format, 256, 240)
        .unwrap();

    let program = std::fs::read("roms/games/Pac-Man.nes").unwrap();
    let mut rom = Rom::new(&program).unwrap();

    let right_bank = show_tile_bank(&rom.take_chr_rom(), 0);

    texture.update(None, &right_bank.data, 256 * 3).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => { /* do nothing */ }
            }
        }
    }
}

fn snake_game() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake Game", 32 * 10, 32 * 10)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let pixel_format = unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) };
    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(pixel_format, 32, 32).unwrap();

    let program = std::fs::read("roms/games/snake.nes").unwrap();
    let rom = Rom::new(&program).unwrap();
    let bus = Bus::new(rom, |_, _| {});

    let mut cpu = CPU6502::new(bus);
    cpu.reset();

    let mut screen_state = [0u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();

    cpu.run_with_callback(|cpu| {
        handle_user_input(cpu, &mut event_pump);
        cpu.mem_write(0xfe, rng.gen_range(1, 16));

        if read_screan_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        std::thread::sleep(std::time::Duration::new(0, 50_000));
    });
}

fn handle_user_input(cpu: &mut CPU6502, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => cpu.mem_write(0xff, 0x77),
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => cpu.mem_write(0xff, 0x73),
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => cpu.mem_write(0xff, 0x61),
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => cpu.mem_write(0xff, 0x64),
            _ => {}
        }
    }
}

fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn read_screan_state(cpu: &mut CPU6502, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x0600 {
        let color_idx = cpu.mem_read(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}
