// Chip-8 Virtual Machine

// TODO: Use constants in tests
// TODO: Use consistent indexes with hex values.

mod specs;
mod bootstrap;

use rand::{thread_rng, Rng};
use instructions::Instruction;
use std::io::{Write, BufWriter};
use vm::specs::*;

// TODO: How to print things to the screen/display?

pub struct VM {
    ram: [u8; RAM_SIZE],                     // Memory
    registers: [u8; GENERAL_REGISTERS_SIZE], // V0 - VF registers
    stack: [u16; STACK_SIZE],                // Stack for return addresses of subroutines
    keypad: [u8; KEYPAD_SIZE],               // Keep track of any key pressed in the keypad
    gfx: [u8; DISPLAY_PIXELS],               // Graphics "card"

    i: usize,                                // Store memory addresses

    dt: u8,                                  // Delay Timer register
    st: u8,                                  // Sound Timer register

    pc: usize,                               // Program Counter
    sp: usize,                               // Stack Pointer
}

impl VM {
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
        //     runtime::clear::exec(self, opcode)
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
                self.registers[opcode.x as usize] += opcode.data;

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

            Instruction::Draw(_) => {
                // TODO

                self.advance();
            },

            Instruction::SkipOnKeyPressed(opcode) => {
                let key = self.registers[opcode.x as usize] as usize;

                if self.keypad[key] == 1 {
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
                let key = self.keypad.iter().position(|&s| s == 1);

                if let Some(value) = key {
                    self.registers[opcode.x as usize] = value as u8;
                    self.advance();
                }
            },

            Instruction::AddI(opcode) => {
                let vx = self.registers[opcode.x as usize] as u16;
                self.i += vx as usize;

                self.advance();
            },

            Instruction::SetFont(_) => {
                // TODO

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
                let mut pointer = self.i;

                for v in 0..opcode.x {
                    self.ram[pointer] = self.registers[v as usize];
                    pointer += 1;
                }

                self.advance();
            }

            Instruction::Read(opcode) => {
                for v in 0..opcode.x {
                    let pointer = self.i + v as usize;
                    self.registers[v as usize] = self.ram[pointer];
                }

                self.advance();
            }
        }
    }
}

#[cfg(test)]
mod tests;
