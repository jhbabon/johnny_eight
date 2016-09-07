extern crate rand;
extern crate sdl2;
extern crate chip_8;
#[macro_use]
extern crate log;
extern crate env_logger;

use chip_8::display::Display;
use chip_8::vm::bootstrap::Bootstrap; // TODO: deprecate bootstrap in favor of VM::build()
use chip_8::specs;
use chip_8::instructions::Instruction;
use chip_8::keypad::Key;
use std::fs::File;

use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::Duration;
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

    // TODO: Move this to a cli module?
    let mut vm = Bootstrap::new()
        .load_sprites()
        .load_rom(&mut rom)
        .finish();

    // start sdl2 with everything
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // Create a window
    // Use a factor or scale constant. Now is scaled using a factor of 20.
    let width = (specs::DISPLAY_WIDTH * specs::DISPLAY_SCALE) as u32;
    let height = (specs::DISPLAY_HEIGHT * specs::DISPLAY_SCALE) as u32;
    let window = video_ctx
        .window("Chip-8", width, height)
        .position_centered()
        .opengl()
        .build();

    let window = match window {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Create a rendering context
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    // Black
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0 , 0));
    let _ = renderer.clear();

    // White
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

    let scale = specs::DISPLAY_SCALE as f32;
    let _ = renderer.set_scale(scale, scale);

    // Swap our buffer for the present buffer, displaying it.
    let _ = renderer.present();

    let mut events = ctx.event_pump().unwrap();

    // Build display with its data bus
    let (bus, display) = Display::build();

    // TODO: Move this to the boot system
    //
    // Example:
    //
    //     let mut vm = VM::boot()
    //         .load_sprites()
    //         .load_rom()
    //         .load_display_bus(tbus);
    vm.set_bus(bus);

    // CLOCK!
    // Create channels for sending and receiving
    let (tx, rx) = channel();
    // Spawn clock timer
    let clock = thread::spawn(move || {
        'clock : loop {
            thread::sleep(Duration::from_millis(specs::CLOCK));
            if tx.send("tick").is_err() {
                break 'clock;
            };
        };

        // TODO: Is this necessary?
        drop(tx);
    });

    // Wait a little bit before stating the VM
    thread::sleep(Duration::from_millis(50));

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

        match rx.try_recv() {
            Err(TryRecvError::Disconnected) => panic!("The clock died!"),
            Ok("tick") => {
                let mut bytes = 0x0 as u16;
                bytes = vm.ram[vm.pc] as u16;
                bytes = bytes << 8;
                bytes = bytes | vm.ram[vm.pc + 1] as u16;

                let instruction = match Instruction::decode(bytes) {
                    Some(ins) => ins,
                    None => panic!("Unknown instruction {:?}", bytes), // TODO: Ignore unknown instructions.
                };
                // println!("Decoded instruction {:?}", instruction);
                vm.exec(instruction);

                // Decrement the timers
                if vm.dt > 0 {
                    vm.dt -= 1;
                }

                if vm.st > 0 {
                    println!("BEEP!");
                    vm.st -= 1;
                }

                // Send all pixel information to the renderer.
                display.flush(&mut renderer);
            },
            _ => {}
        };
    }

    // TODO: Is this necessary?
    drop(clock);
    drop(rx);
}
