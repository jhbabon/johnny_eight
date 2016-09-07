extern crate rand;
extern crate sdl2;
extern crate johnny_eight;
#[macro_use]
extern crate env_logger;

use johnny_eight::display::Display;
use johnny_eight::vm::VM;
use johnny_eight::specs;
use johnny_eight::keypad::Key;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::fs::File;
use std::env;
use std::process::exit;

fn main() {
    env_logger::init().unwrap();

    let rom_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("You must provide a path to the ROM file");
            exit(1);
        }
    };
    let mut rom = File::open(rom_path).unwrap();

    // Window initialization
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let width = (specs::DISPLAY_WIDTH * specs::DISPLAY_SCALE) as u32;
    let height = (specs::DISPLAY_HEIGHT * specs::DISPLAY_SCALE) as u32;
    let window = video_ctx.window("Johnny Eight", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let scale = specs::DISPLAY_SCALE as f32;
    let _ = renderer.set_scale(scale, scale);

    // Paint screen black
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    let _ = renderer.clear();

    // Display the black screen.
    let _ = renderer.present();

    // Build a Display with its data bus
    let (bus, display) = Display::build();

    // Build the VM
    let mut vm = VM::boot();
    vm.load_sprites()
        .load_rom(&mut rom)
        .set_display_bus(bus)
        .init_clock();

    let mut events = ctx.event_pump().unwrap();

    // loop until we receive a QuitEvent
    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'event,

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => vm.set_key(Key::Num1),
                        Keycode::Num2 => vm.set_key(Key::Num2),
                        Keycode::Num3 => vm.set_key(Key::Num3),
                        Keycode::Num4 => vm.set_key(Key::C),
                        Keycode::Q => vm.set_key(Key::Num4),
                        Keycode::W => vm.set_key(Key::Num5),
                        Keycode::E => vm.set_key(Key::Num6),
                        Keycode::R => vm.set_key(Key::D),
                        Keycode::A => vm.set_key(Key::Num7),
                        Keycode::S => vm.set_key(Key::Num8),
                        Keycode::D => vm.set_key(Key::Num9),
                        Keycode::F => vm.set_key(Key::E),
                        Keycode::Z => vm.set_key(Key::A),
                        Keycode::X => vm.set_key(Key::Num0),
                        Keycode::C => vm.set_key(Key::B),
                        Keycode::V => vm.set_key(Key::F),
                        _ => {}
                    };
                }

                _ => {}
            }
        }

        vm.cycle();
        display.flush(&mut renderer);
    }
}
