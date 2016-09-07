extern crate rand;
extern crate sdl2;
extern crate chip_8;
#[macro_use]
extern crate log;
extern crate env_logger;

use chip_8::display::Display;
use chip_8::vm::VM;
use chip_8::specs;
use chip_8::keypad::Key;
use std::fs::File;

use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

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
    let window = video_ctx
        .window("Chip-8", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let scale = specs::DISPLAY_SCALE as f32;
    let _ = renderer.set_scale(scale, scale);

    // Black
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0 , 0));
    let _ = renderer.clear();

    // Display the black screen.
    let _ = renderer.present();

    let mut events = ctx.event_pump().unwrap();

    // Build a Display with its data bus
    let (bus, display) = Display::build();

    // Build the VM
    let mut vm = VM::boot();
    vm.load_sprites()
        .load_rom(&mut rom)
        .set_display_bus(bus)
        .init_clock();

    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..}
                | Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'event
                },

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Num0 => { vm.set_key(Key::Num0) },
                        Keycode::Num1 => { vm.set_key(Key::Num1) },
                        Keycode::Num2 => { vm.set_key(Key::Num2) },
                        Keycode::Num3 => { vm.set_key(Key::Num3) },
                        Keycode::Num4 => { vm.set_key(Key::Num4) },
                        Keycode::Num5 => { vm.set_key(Key::Num5) },
                        Keycode::Num6 => { vm.set_key(Key::Num6) },
                        Keycode::Num7 => { vm.set_key(Key::Num7) },
                        Keycode::Num8 => { vm.set_key(Key::Num8) },
                        Keycode::Num9 => { vm.set_key(Key::Num9) },
                        Keycode::A    => { vm.set_key(Key::A) },
                        Keycode::B    => { vm.set_key(Key::B) },
                        Keycode::C    => { vm.set_key(Key::C) },
                        Keycode::D    => { vm.set_key(Key::D) },
                        Keycode::E    => { vm.set_key(Key::E) },
                        Keycode::F    => { vm.set_key(Key::F) },
                        _ => {},
                    };
                },

                _ => {},
            }
        }

        vm.cycle();
        display.flush(&mut renderer);
    }
}
