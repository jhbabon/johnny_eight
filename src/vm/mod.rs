// Chip-8 Virtual Machine

// TODO: Use consistent indexes with hex values.

mod runtime;

use std::io::Read;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::time::Duration;
use std::thread;

use instructions::Instruction;
use keypad::Key;
use display::Pixel;
use specs;
use vm::runtime::Next;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Tick;

pub struct VM {
    ram: [u8; specs::RAM_SIZE], // Memory
    registers: [u8; specs::GENERAL_REGISTERS_SIZE], // V0 - VF registers
    stack: [u16; specs::STACK_SIZE], // Stack for return addresses of subroutines
    keypad: [u8; specs::KEYPAD_SIZE], // Keep track of any key pressed in the keypad
    gfx: [u8; specs::DISPLAY_PIXELS], // Graphics "card"

    i: usize, // Store memory addresses

    dt: u8, // Delay Timer register
    st: u8, // Sound Timer register

    pc: usize, // Program Counter
    sp: usize, // Stack Pointer

    display_bus: Option<Sender<Vec<Pixel>>>, // Bus for the display

    clock: Option<Receiver<Tick>>, // Clock notifications
}

impl VM {
    pub fn boot() -> VM {
        info!("Booting VM");

        VM {
            ram: [0; specs::RAM_SIZE],
            registers: [0; specs::GENERAL_REGISTERS_SIZE],
            stack: [0; specs::STACK_SIZE],
            keypad: [0; specs::KEYPAD_SIZE],
            gfx: [0; specs::DISPLAY_PIXELS],

            pc: specs::PROGRAM_START,
            i: 0,
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
            'clock: loop {
                thread::sleep(Duration::from_millis(specs::CLOCK));
                if ticker.send(Tick).is_err() {
                    break 'clock;
                };
            }
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
                }
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
                    _ => false,
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
        }
    }

    pub fn exec(&mut self, instruction: Instruction) {
        let next = match instruction {
            Instruction::Clear => runtime::clear(self),
            Instruction::Return => runtime::ret(self),
            Instruction::Jump(opcode) => runtime::jump(self, opcode),
            Instruction::Call(opcode) => runtime::call(self, opcode),

            Instruction::SkipOnEqualByte(opcode) => runtime::skip_on_equal_byte(self, opcode),

            Instruction::SkipOnNotEqualByte(opcode) => {
                runtime::skip_on_not_equal_byte(self, opcode)
            }

            Instruction::SkipOnEqual(opcode) => runtime::skip_on_equal(self, opcode),

            Instruction::SkipOnNotEqual(opcode) => runtime::skip_on_not_equal(self, opcode),

            Instruction::SetByte(opcode) => runtime::set_byte(self, opcode),

            Instruction::AddByte(opcode) => runtime::add_byte(self, opcode),

            Instruction::Set(opcode) => runtime::set(self, opcode),

            Instruction::Or(opcode) => runtime::or(self, opcode),

            Instruction::And(opcode) => runtime::and(self, opcode),

            Instruction::Xor(opcode) => runtime::xor(self, opcode),

            Instruction::Add(opcode) => runtime::add(self, opcode),

            Instruction::SubXY(opcode) => runtime::sub_x_y(self, opcode),

            Instruction::SubYX(opcode) => runtime::sub_y_x(self, opcode),

            Instruction::ShiftRight(opcode) => runtime::shift_right(self, opcode),

            Instruction::ShiftLeft(opcode) => runtime::shift_left(self, opcode),

            Instruction::SetI(opcode) => runtime::set_i(self, opcode),

            Instruction::JumpPlus(opcode) => runtime::jump_plus(self, opcode),

            Instruction::RandomMask(opcode) => runtime::random_mask(self, opcode),

            Instruction::Draw(opcode) => runtime::draw(self, opcode),

            Instruction::SkipOnKeyPressed(opcode) => runtime::skip_on_key_pressed(self, opcode),

            Instruction::SkipOnKeyNotPressed(opcode) => {
                runtime::skip_on_key_not_pressed(self, opcode)
            }

            Instruction::StoreDelayTimer(opcode) => runtime::store_delay_timer(self, opcode),

            Instruction::SetDelayTimer(opcode) => runtime::set_delay_timer(self, opcode),

            Instruction::SetSoundTimer(opcode) => runtime::set_sound_timer(self, opcode),

            Instruction::WaitKey(opcode) => runtime::wait_key(self, opcode),

            Instruction::AddI(opcode) => runtime::add_i(self, opcode),

            Instruction::SetSprite(opcode) => runtime::set_sprite(self, opcode),

            Instruction::Bcd(opcode) => runtime::bcd(self, opcode),

            Instruction::Store(opcode) => runtime::store(self, opcode),

            Instruction::Read(opcode) => runtime::read(self, opcode),
        };

        match next {
            Next::Advance(steps) => self.advance_by(steps),
            Next::Noop => (),
        };
    }
}

#[cfg(test)]
mod tests;
