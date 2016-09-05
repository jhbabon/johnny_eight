extern crate rand;
extern crate sdl2;
extern crate chip_8;

use chip_8::vm::bootstrap::Bootstrap;
use chip_8::vm::specs::*;
use chip_8::instructions::Instruction;
use std::fs::File;

use sdl2::event::{Event};
use sdl2::rect::{Rect,Point};
use sdl2::keyboard::Keycode;

use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::Duration;
use std::env;
use std::process::exit;

fn main() {
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
    let window  = match video_ctx.window("Chip-8", 1280, 640).position_centered().opengl().build() {
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

    // Swap our buffer for the present buffer, displaying it.
    let _ = renderer.present();

    let mut events = ctx.event_pump().unwrap();

    // CLOCK!
    // Create channels for sending and receiving
    let (tx, rx) = channel();
    // Spawn clock timer
    let clock = thread::spawn(move || {
        'clock : loop {
            thread::sleep(Duration::from_millis(CLOCK));
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
                Event::Quit{..} => break 'event,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Num0 => { vm.keypad[0x0] = 1 },
                        Keycode::Num1 => { vm.keypad[0x1] = 1 },
                        Keycode::Num2 => { vm.keypad[0x2] = 1 },
                        Keycode::Num3 => { vm.keypad[0x3] = 1 },
                        Keycode::Num4 => { vm.keypad[0x4] = 1 },
                        Keycode::Num5 => { vm.keypad[0x5] = 1 },
                        Keycode::Num6 => { vm.keypad[0x6] = 1 },
                        Keycode::Num7 => { vm.keypad[0x7] = 1 },
                        Keycode::Num8 => { vm.keypad[0x8] = 1 },
                        Keycode::Num9 => { vm.keypad[0x9] = 1 },
                        Keycode::A    => { vm.keypad[0xA] = 1 },
                        Keycode::B    => { vm.keypad[0xB] = 1 },
                        Keycode::C    => { vm.keypad[0xC] = 1 },
                        Keycode::D    => { vm.keypad[0xD] = 1 },
                        Keycode::E    => { vm.keypad[0xE] = 1 },
                        Keycode::F    => { vm.keypad[0xF] = 1 },
                        _ => {},
                    };
                },
                Event::KeyUp {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Num0 => { vm.keypad[0x0] = 0 },
                        Keycode::Num1 => { vm.keypad[0x1] = 0 },
                        Keycode::Num2 => { vm.keypad[0x2] = 0 },
                        Keycode::Num3 => { vm.keypad[0x3] = 0 },
                        Keycode::Num4 => { vm.keypad[0x4] = 0 },
                        Keycode::Num5 => { vm.keypad[0x5] = 0 },
                        Keycode::Num6 => { vm.keypad[0x6] = 0 },
                        Keycode::Num7 => { vm.keypad[0x7] = 0 },
                        Keycode::Num8 => { vm.keypad[0x8] = 0 },
                        Keycode::Num9 => { vm.keypad[0x9] = 0 },
                        Keycode::A    => { vm.keypad[0xA] = 0 },
                        Keycode::B    => { vm.keypad[0xB] = 0 },
                        Keycode::C    => { vm.keypad[0xC] = 0 },
                        Keycode::D    => { vm.keypad[0xD] = 0 },
                        Keycode::E    => { vm.keypad[0xE] = 0 },
                        Keycode::F    => { vm.keypad[0xF] = 0 },
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
                    None => panic!("Unknown instruction {:?}", bytes),
                };
                // println!("Decoded instruction {:?}", instruction);
                vm.exec(instruction);

                let mut x = 0;
                let mut y = 0;
                for pixel in vm.gfx.iter() {
                    if *pixel == 1 {
                        let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                    } else {
                        let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0 , 0));
                    };

                    let point = Point::new(x, y).scale(20);
                    let rect = Rect::new(point.x(), point.y(), 20, 20);
                    let _ = renderer.fill_rect(rect);

                    if x == 63 {
                        x = 0;
                        y += 1;
                    } else {
                        x += 1;
                    }

                }

                // Decrement the timers
                if vm.dt > 0 {
                    vm.dt -= 1;
                }

                if vm.st > 0 {
                    println!("BEEP!");
                    vm.st -= 1;
                }

                // reset keypad
                // for key in vm.keypad.iter_mut() {
                //     *key = 0;
                // }

                let _ = renderer.present();
            },
            _ => {}
        };
    }

    // TODO: Is this necessary?
    drop(clock);
    drop(rx);
}
