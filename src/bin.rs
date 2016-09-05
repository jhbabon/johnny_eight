extern crate rand;
extern crate sdl2;
extern crate chip_8;

use chip_8::vm::bootstrap::Bootstrap;
use chip_8::vm::specs::*;
use chip_8::instructions::Instruction;
use std::fs::File;
use sdl2::event::{Event};
use sdl2::rect::Point;

use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let rom_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        // "/fixtures/ibm_logo.rom"
        "/fixtures/chip_8_picture.rom"
        // "/fixtures/random_number_test.rom"
        // "/fixtures/clock.rom"
    );
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
    // TODO: Scale window and pixels, it is too small.
    let window  = match video_ctx.window("Chip-8", 64, 32).position_centered().opengl().build() {
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

                let instruction = Instruction::decode(bytes).unwrap();
                vm.exec(instruction);

                let mut x = 0;
                let mut y = 0;
                for pixel in vm.gfx.iter() {
                    if *pixel == 1 {
                        let point = Point::new(x, y);
                        let _ = renderer.draw_point(point);
                    }

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

                let _ = renderer.present();
            },
            _ => {}
        };
    }

    // TODO: Is this necessary?
    drop(clock);
    drop(rx);
}
