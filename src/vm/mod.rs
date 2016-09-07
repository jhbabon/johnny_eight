// Chip-8 Virtual Machine

// TODO: Use consistent indexes with hex values.

use std::io::Read;
use std::sync::mpsc::{channel,Sender,Receiver,TryRecvError};
use std::time::Duration;
use std::thread;
use rand::{thread_rng, Rng};

use instructions::Instruction;
use keypad::Key;
use display::Pixel;
use specs;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Tick;

pub struct VM {
    ram:       [u8; specs::RAM_SIZE],               // Memory
    registers: [u8; specs::GENERAL_REGISTERS_SIZE], // V0 - VF registers
    stack:     [u16; specs::STACK_SIZE],            // Stack for return addresses of subroutines
    keypad:    [u8; specs::KEYPAD_SIZE],            // Keep track of any key pressed in the keypad
    gfx:       [u8; specs::DISPLAY_PIXELS],         // Graphics "card"

    i: usize,                                       // Store memory addresses

    dt: u8,                                         // Delay Timer register
    st: u8,                                         // Sound Timer register

    pc: usize,                                      // Program Counter
    sp: usize,                                      // Stack Pointer

    display_bus: Option<Sender<Vec<Pixel>>>,        // Bus for the display

    clock: Option<Receiver<Tick>>,                  // Clock notifications
}

impl VM {
    pub fn boot() -> VM {
        info!("Booting VM");

        VM {
            ram:       [0; specs::RAM_SIZE],
            registers: [0; specs::GENERAL_REGISTERS_SIZE],
            stack:     [0; specs::STACK_SIZE],
            keypad:    [0; specs::KEYPAD_SIZE],
            gfx:       [0; specs::DISPLAY_PIXELS],

            pc: specs::PROGRAM_START,
            i:  0,
            sp: 0,
            dt: 0,
            st: 0,

            display_bus: None,
            clock: None,
        }
    }

    pub fn load_sprites<'a>(&'a mut self) -> &'a mut VM {
        info!("Loading SPRITES into memory");

        let range = specs::SPRITES_ADDR..(specs::SPRITES_ADDR + specs::SPRITES_SIZE);
        for addr in range {
            let index = (addr - specs::SPRITES_ADDR) as usize;
            self.ram[addr] = specs::SPRITES[index];
        }

        self
    }

    pub fn load_rom<'a>(&'a mut self, reader: &mut Read) -> &'a mut VM {
        info!("Loading ROM into memory");

        let mut rom = Vec::new();
        if let Err(_) = reader.read_to_end(&mut rom) {
            panic!("Error reading ROM");
        }

        let mut addr = specs::PROGRAM_START;
        for byte in &rom {
            self.ram[addr] = *byte;
            addr += 1;
        }

        self
    }

    pub fn set_display_bus<'a>(&'a mut self, bus: Sender<Vec<Pixel>>) -> &'a mut VM {
        self.display_bus = Some(bus);

        self
    }

    pub fn init_clock<'a>(&'a mut self) -> &'a mut VM {
        let (ticker, clock) = channel();

        let _ = thread::spawn(move || {
            'clock : loop {
                thread::sleep(Duration::from_millis(specs::CLOCK));
                if ticker.send(Tick).is_err() {
                    break 'clock;
                };
            };
        });

        self.clock = Some(clock);

        self
    }

    pub fn cycle(&mut self) {
        if self.tick() {
            let mut bytes = self.ram[self.pc] as u16;
            bytes = bytes << 8;
            bytes = bytes | self.ram[self.pc + 1] as u16;

            match Instruction::decode(bytes) {
                Some(ins) => {
                    debug!("Decoded instruction {:?}", ins);
                    self.exec(ins);
                },
                None => debug!("Unknown instruction {:?}", bytes),
            };

            // Decrement the timers
            if self.dt > 0 {
                self.dt -= 1;
            }

            if self.st > 0 {
                println!("BEEP!"); // TODO: Sound system.
                self.st -= 1;
            }
        };
    }

    fn tick(&mut self) -> bool {
        match self.clock {
            None => false,
            Some(ref clk) => {
                match clk.try_recv() {
                    Err(TryRecvError::Disconnected) => panic!("The clock died!"),
                    Ok(Tick) => true,
                    _ => false
                }
            }
        }
    }

    pub fn set_key(&mut self, key: Key) {
        debug!("Key {:?} pressed", key);
        self.keypad[key.as_usize()] += 1;
    }

    pub fn advance(&mut self) {
        // We move the PC by two because we need to read
        // two bytes in each cycle.
        self.pc += 2;
    }

    pub fn advance_by(&mut self, times: u16) {
        for _ in 0..times {
            self.advance();
        };
    }

    pub fn exec(&mut self, instruction: Instruction) {
        // TODO: I would move the actual executions
        // to a runtime module and have something like this
        //
        //   Instruction::Jump(opcode) => {
        //     // self is the VM.
        //     runtime::clear::exec(&mut self, opcode)
        //   }
        //
        // This can return a Next struct that indicates the next
        // step:
        //
        //   struct Next {
        //     Advance(steps), // Advance steps
        //     Noop, // Don't do anything!
        //   }
        match instruction {
            Instruction::Clear => {
                for pixel in self.gfx.iter_mut() {
                    *pixel = 0;
                }

                self.advance();
            },

            Instruction::Return => {
                self.pc = self.stack[self.sp] as usize;
                self.sp -= 1;

                self.advance();
            },

            Instruction::Jump(opcode) => {
                self.pc = opcode.address as usize;
            },

            Instruction::Call(opcode) => {
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16;
                self.pc = opcode.address as usize;
            },

            Instruction::SkipOnEqualByte(opcode) => {
                let vx = self.registers[opcode.x as usize];
                if vx == opcode.data {
                    self.advance_by(2);
                } else {
                    self.advance();
                };
            },

            Instruction::SkipOnNotEqualByte(opcode) => {
                let vx = self.registers[opcode.x as usize];
                if vx != opcode.data {
                    self.advance_by(2);
                } else {
                    self.advance();
                };
            },

            Instruction::SkipOnEqual(opcode) => {
                let vx = self.registers[opcode.x as usize];
                let vy = self.registers[opcode.y as usize];
                if vx == vy {
                    self.advance_by(2);
                } else {
                    self.advance();
                };
            },

            Instruction::SkipOnNotEqual(opcode) => {
                let vx = self.registers[opcode.x as usize];
                let vy = self.registers[opcode.y as usize];
                if vx != vy {
                    self.advance_by(2);
                } else {
                    self.advance();
                };
            },

            Instruction::SetByte(opcode) => {
                self.registers[opcode.x as usize] = opcode.data;

                self.advance();
            },

            Instruction::AddByte(opcode) => {
                let vx = self.registers[opcode.x as usize];
                self.registers[opcode.x as usize] = vx.wrapping_add(opcode.data);

                self.advance();
            },

            Instruction::Set(opcode) => {
                let vy = self.registers[opcode.y as usize];
                self.registers[opcode.x as usize] = vy;

                self.advance();
            },

            Instruction::Or(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                self.registers[opcode.x as usize] = vx | vy;

                self.advance();
            },

            Instruction::And(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                self.registers[opcode.x as usize] = vx & vy;

                self.advance();
            },

            Instruction::Xor(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                self.registers[opcode.x as usize] = vx ^ vy;

                self.advance();
            },

            Instruction::Add(opcode) => {
                let vy = self.registers[opcode.y as usize] as u16;
                let vx = self.registers[opcode.x as usize] as u16;
                let add = vx + vy;

                if add > 0xFF {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }

                self.registers[opcode.x as usize] = add as u8;

                self.advance();
            },

            Instruction::SubXY(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                if vx > vy {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }

                self.registers[opcode.x as usize] = vx.wrapping_sub(vy);

                self.advance();
            },

            Instruction::SubYX(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                if vy > vx {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }

                self.registers[opcode.x as usize] = vy.wrapping_sub(vx);

                self.advance();
            },

            Instruction::ShiftRight(opcode) => {
                let vy = self.registers[opcode.y as usize];

                self.registers[0xF] = vy & 0x1;
                self.registers[opcode.x as usize] = vy >> 1;

                self.advance();
            },

            Instruction::ShiftLeft(opcode) => {
                let vy = self.registers[opcode.y as usize];

                self.registers[0xF] = (vy >> 7) & 0x1;
                self.registers[opcode.x as usize] = vy << 1;

                self.advance();
            },

            Instruction::SetI(opcode) => {
                self.i = opcode.address as usize;

                self.advance();
            },

            Instruction::JumpPlus(opcode) => {
                let v0 = self.registers[0] as u16;

                self.pc = (v0 + opcode.address) as usize;
            },

            Instruction::RandomMask(opcode) => {
                let mut rng = thread_rng();
                let rnd: u16 = rng.gen_range(0, 256);
                let rnd: u8 = rnd as u8;

                self.registers[opcode.x as usize] = rnd & opcode.data;

                self.advance();
            },

            Instruction::Draw(opcode) => {
                let x = self.registers[opcode.x as usize] as usize;
                let y = self.registers[opcode.y as usize] as usize;
                let i = self.i;
                let n = opcode.nibble as usize;

                let mut pixels: Vec<Pixel> = vec![];

                self.registers[0xF] = 0;
                for (sy, byte) in self.ram[i..i+n].iter().enumerate() {
                    let dy = (y + sy) % specs::DISPLAY_HEIGHT;
                    for sx in 0usize..8 {
                        let px = (*byte >> (7 - sx)) & 0b00000001;
                        let dx = (x + sx) % specs::DISPLAY_WIDTH;
                        let idx = dy * specs::DISPLAY_WIDTH + dx;
                        self.gfx[idx] ^= px;

                        // Vf is if there was a collision
                        self.registers[0xF] |= (self.gfx[idx] == 0 && px == 1) as u8;

                        let pixel = Pixel::new(dx as i32, dy as i32, self.gfx[idx]);

                        pixels.push(pixel);
                    }
                }

                if let Some(ref bus) = self.display_bus {
                    bus.send(pixels).unwrap();
                };

                self.advance();
            },

            Instruction::SkipOnKeyPressed(opcode) => {
                let key = self.registers[opcode.x as usize] as usize;

                if self.keypad[key] > 0 {
                    self.keypad[key] -= 1;
                    self.advance_by(2);
                } else {
                    self.advance();
                };
            },

            Instruction::SkipOnKeyNotPressed(opcode) => {
                let key = self.registers[opcode.x as usize] as usize;

                if self.keypad[key] == 0 {
                    self.advance_by(2);
                } else {
                    self.keypad[key] -= 1;
                    self.advance();
                };
            },

            Instruction::StoreDelayTimer(opcode) => {
                self.registers[opcode.x as usize] = self.dt;

                self.advance();
            },

            Instruction::SetDelayTimer(opcode) => {
                self.dt = self.registers[opcode.x as usize];

                self.advance();
            },

            Instruction::SetSoundTimer(opcode) => {
                self.st = self.registers[opcode.x as usize];

                self.advance();
            },

            Instruction::WaitKey(opcode) => {
                let key = self.keypad.iter().position(|&s| s > 0);
                if let Some(value) = key {
                    self.registers[opcode.x as usize] = value as u8;
                    self.keypad[value] -= 1;
                    self.advance();
                }
            },

            Instruction::AddI(opcode) => {
                let vx = self.registers[opcode.x as usize] as u16;
                self.i += vx as usize;

                self.advance();
            },

            Instruction::SetSprite(opcode) => {
                let vx = self.registers[opcode.x as usize] as usize;
                self.i = specs::SPRITES_ADDR + vx * specs::SPRITE_HEIGHT;

                self.advance();
            },

            Instruction::Bcd(opcode) => {
                // TODO: Clean up
                let vx = self.registers[opcode.x as usize];

                let b = vx / 100;
                let c = (vx - (b * 100)) / 10;
                let d = vx - (b * 100) - (c * 10);

                self.ram[self.i]       = b as u8;
                self.ram[(self.i + 1)] = c as u8;
                self.ram[(self.i + 2)] = d as u8;

                self.advance();
            },

            Instruction::Store(opcode) => {
                for v in 0..opcode.x {
                    let pointer = self.i + v as usize;
                    self.ram[pointer] = self.registers[v as usize];
                }

                self.i += (opcode.x + 1) as usize;

                self.advance();
            }

            Instruction::Read(opcode) => {
                for v in 0..opcode.x {
                    let pointer = self.i + v as usize;
                    self.registers[v as usize] = self.ram[pointer];
                }

                self.i += (opcode.x + 1) as usize;

                self.advance();
            }
        }
    }
}

#[cfg(test)]
mod tests;
