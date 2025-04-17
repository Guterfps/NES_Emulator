mod emulator;

use emulator::bus::Bus;
use emulator::cpu::CPU6502;
use emulator::rom::Rom;

use rand::Rng;
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::pixels::PixelFormat;
use sdl3::sys::pixels::SDL_PixelFormat;

fn main() {
    snake_game();
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
    let bus = Bus::new(rom);

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
