// Chip-8 Virtual Machine

// TODO: Use constants in tests
// TODO: Use consistent indexes with hex values.

use rand::{thread_rng, Rng};
use instructions::Instruction;
use std::io::{Write, BufWriter};

const RAM_SIZE: usize = 4096;
const GENERAL_REGISTERS_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

const FONT_HEIGHT: usize = 5;
const FONTS_SIZE: usize = FONT_HEIGHT * 16;
const FONTS_ADDR: usize = 0;
const FONTS: [u8; FONTS_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const KEYPAD_SIZE: usize = 16;

const CLOCK_HZ: f32 = 600.0; // I don't really know why a float is necessary.

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_PIXELS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

// TODO: How to print things to the screen/display?

pub struct VM {
    ram: [u8; RAM_SIZE],                     // Memory
    registers: [u8; GENERAL_REGISTERS_SIZE], // V0 - VF registers
    stack: [u16; STACK_SIZE],                // Stack for return addresses of subroutines
    keypad: [u8; KEYPAD_SIZE],               // Keep track of any key pressed in the keypad
    gfx: [u8; DISPLAY_PIXELS],               // Graphics "card"

    i: u16,                                  // Store memory addresses

    dt: u8,                                  // Delay Timer register
    st: u8,                                  // Sound Timer register

    pc: u16,                                 // Program Counter
    sp: usize,                               // Stack Pointer
}

impl VM {
    // Fill the VM with all the information it needs, like fonts registers.
    // TODO: Move all the default and boot process to a Boot struct
    // following the builder pattern
    //
    // E.g:
    //
    //   let mut vm = Boot.new().init_fonts().finish();
    pub fn boot(&mut self) {
        let size = FONTS_ADDR..(FONTS_ADDR + FONTS_SIZE);
        let mut buffer = BufWriter::new(&mut self.ram[size]);

        buffer.write_all(&FONTS).unwrap();
    }

    pub fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Clear => {
                for pixel in self.gfx.iter_mut() {
                    *pixel = 0;
                }
            },

            Instruction::Return => {
                self.pc = self.stack[self.sp];
                self.sp -= 1;
            },

            Instruction::Jump(opcode) => {
                self.pc = opcode.address;
            },

            Instruction::Call(opcode) => {
                self.sp += 1;
                self.stack[self.sp] = self.pc;
                self.pc = opcode.address;
            },

            Instruction::SkipOnEqualByte(opcode) => {
                let vx = self.registers[opcode.x as usize];
                if vx == opcode.data {
                    self.pc += 2;
                };
            },

            Instruction::SkipOnNotEqualByte(opcode) => {
                let vx = self.registers[opcode.x as usize];
                if vx != opcode.data {
                    self.pc += 2;
                };
            },

            Instruction::SkipOnEqual(opcode) => {
                let vx = self.registers[opcode.x as usize];
                let vy = self.registers[opcode.y as usize];
                if vx == vy {
                    self.pc += 2;
                };
            },

            Instruction::SkipOnNotEqual(opcode) => {
                let vx = self.registers[opcode.x as usize];
                let vy = self.registers[opcode.y as usize];
                if vx != vy {
                    self.pc += 2;
                };
            },

            Instruction::SetByte(opcode) => {
                self.registers[opcode.x as usize] = opcode.data;
            },

            Instruction::AddByte(opcode) => {
                self.registers[opcode.x as usize] += opcode.data;
            },

            Instruction::Set(opcode) => {
                let vy = self.registers[opcode.y as usize];
                self.registers[opcode.x as usize] = vy;
            },

            Instruction::Or(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                self.registers[opcode.x as usize] = vx | vy;
            },

            Instruction::And(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                self.registers[opcode.x as usize] = vx & vy;
            },

            Instruction::Xor(opcode) => {
                let vy = self.registers[opcode.y as usize];
                let vx = self.registers[opcode.x as usize];

                self.registers[opcode.x as usize] = vx ^ vy;
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
            },

            Instruction::ShiftRight(opcode) => {
                let vy = self.registers[opcode.y as usize];

                self.registers[0xF] = vy & 0x1;
                self.registers[opcode.x as usize] = vy >> 1;
            },

            Instruction::ShiftLeft(opcode) => {
                let vy = self.registers[opcode.y as usize];

                self.registers[0xF] = (vy >> 7) & 0x1;
                self.registers[opcode.x as usize] = vy << 1;
            },

            Instruction::SetI(opcode) => {
                self.i = opcode.address;
            },

            Instruction::JumpPlus(opcode) => {
                let v0 = self.registers[0] as u16;

                self.pc = v0 + opcode.address;
            },

            Instruction::RandomMask(opcode) => {
                let mut rng = thread_rng();
                let rnd: u16 = rng.gen_range(0, 256);
                let rnd: u8 = rnd as u8;

                self.registers[opcode.x as usize] = rnd & opcode.data;
            },

            Instruction::Draw(_) => {
                // TODO
            },

            Instruction::SkipOnKeyPressed(opcode) => {
                let key = self.registers[opcode.x as usize] as usize;

                if self.keypad[key] == 1 {
                    self.pc += 2;
                };
            },

            Instruction::SkipOnKeyNotPressed(opcode) => {
                let key = self.registers[opcode.x as usize] as usize;

                if self.keypad[key] == 0 {
                    self.pc += 2;
                };
            },

            Instruction::StoreDelayTimer(opcode) => {
                self.registers[opcode.x as usize] = self.dt;
            },

            Instruction::SetDelayTimer(opcode) => {
                self.dt = self.registers[opcode.x as usize];
            },

            Instruction::SetSoundTimer(opcode) => {
                self.st = self.registers[opcode.x as usize];
            },

            _ => {}
        }
    }
}

impl Default for VM {
    fn default() -> VM {
        VM {
            ram:       [0; RAM_SIZE],
            registers: [0; GENERAL_REGISTERS_SIZE],
            stack:     [0; STACK_SIZE],
            keypad:    [0; KEYPAD_SIZE],
            gfx:       [0; DISPLAY_PIXELS],

            i:  0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
        }
    }
}

#[cfg(test)]
mod tests;
