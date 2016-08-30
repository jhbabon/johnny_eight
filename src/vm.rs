// Chip-8 Virtual Machine

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
mod tests {
    use super::*;
    use instructions::Instruction;

    #[test]
    fn vm_has_4k_of_memory() {
        let vm: VM = Default::default();

        assert_eq!(4096, vm.ram.len());
    }

    #[test]
    fn vm_has_16_general_purpose_registers() {
        let vm: VM = Default::default();

        assert_eq!([0 as u8; 16], vm.registers);
    }

    #[test]
    fn vm_has_the_i_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u16, vm.i);
    }

    #[test]
    fn vm_has_the_dt_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u8, vm.dt);
    }

    #[test]
    fn vm_has_the_st_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u8, vm.st);
    }

    #[test]
    fn vm_has_the_pc_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u16, vm.pc);
    }

    #[test]
    fn vm_has_the_sp_register() {
        let vm: VM = Default::default();

        assert_eq!(0, vm.sp);
    }

    #[test]
    fn vm_has_a_stack() {
        let vm: VM = Default::default();

        assert_eq!([0 as u16; 16], vm.stack);
    }

    #[test]
    fn vm_keeps_tracks_of_the_keypad() {
        let vm: VM = Default::default();

        assert_eq!([0 as u8; 16], vm.keypad);
    }

    #[test]
    fn vm_has_graphics() {
        let vm: VM = Default::default();

        assert_eq!((64 * 32), vm.gfx.len());
    }

    #[test]
    fn vm_sets_fonts_in_memory_at_boot_time() {
        let mut vm: VM = Default::default();

        vm.boot();

        assert_eq!([0xF0, 0x90, 0x90, 0x90, 0xF0], vm.ram[0..5]);   // 0
        assert_eq!([0x20, 0x60, 0x20, 0x20, 0x70], vm.ram[5..10]);  // 1
        assert_eq!([0xF0, 0x10, 0xF0, 0x80, 0xF0], vm.ram[10..15]); // 2
        assert_eq!([0xF0, 0x10, 0xF0, 0x10, 0xF0], vm.ram[15..20]); // 3
        assert_eq!([0x90, 0x90, 0xF0, 0x10, 0x10], vm.ram[20..25]); // 4
        assert_eq!([0xF0, 0x80, 0xF0, 0x10, 0xF0], vm.ram[25..30]); // 5
        assert_eq!([0xF0, 0x80, 0xF0, 0x90, 0xF0], vm.ram[30..35]); // 6
        assert_eq!([0xF0, 0x10, 0x20, 0x40, 0x40], vm.ram[35..40]); // 7
        assert_eq!([0xF0, 0x90, 0xF0, 0x90, 0xF0], vm.ram[40..45]); // 8
        assert_eq!([0xF0, 0x90, 0xF0, 0x10, 0xF0], vm.ram[45..50]); // 9
        assert_eq!([0xF0, 0x90, 0xF0, 0x90, 0x90], vm.ram[50..55]); // A
        assert_eq!([0xE0, 0x90, 0xE0, 0x90, 0xE0], vm.ram[55..60]); // B
        assert_eq!([0xF0, 0x80, 0x80, 0x80, 0xF0], vm.ram[60..65]); // C
        assert_eq!([0xE0, 0x90, 0x90, 0x90, 0xE0], vm.ram[65..70]); // D
        assert_eq!([0xF0, 0x80, 0xF0, 0x80, 0xF0], vm.ram[70..75]); // E
        assert_eq!([0xF0, 0x80, 0xF0, 0x80, 0x80], vm.ram[75..80]); // F
    }

    #[test]
    fn vm_executes_clear_instruction() {
        let instruction = Instruction::decode(0x00E0).unwrap();
        let mut vm: VM = VM {
            gfx: [1; (64 * 32)], // set a black screen
            ..Default::default()
        };
        vm.boot();

        vm.exec(instruction);

        assert!(vm.gfx.iter().all(|&x| x == 0));
    }

    #[test]
    fn vm_executes_return_instruction() {
        let instruction = Instruction::decode(0x00EE).unwrap();
        let mut stack = [0; 16];
        stack[1] = 0xA1;

        let mut vm: VM = VM {
            stack: stack,
            sp: 1,
            pc: 0,

            ..Default::default()
        };
        vm.boot();

        vm.exec(instruction);

        assert_eq!(0xA1, vm.pc);
        assert_eq!(0, vm.sp);
    }

    #[test]
    fn vm_executes_jump_instruction() {
        let instruction = Instruction::decode(0x1FA1).unwrap();

        let mut vm: VM = Default::default();
        vm.boot();

        vm.exec(instruction);

        assert_eq!(0x0FA1, vm.pc);
    }

    #[test]
    fn vm_executes_call_instruction() {
        let instruction = Instruction::decode(0x2FA1).unwrap();

        let mut vm: VM = VM {
            pc: 0x0123,

            ..Default::default()
        };
        vm.boot();

        vm.exec(instruction);

        assert_eq!(0x0FA1, vm.pc);
        assert_eq!(1, vm.sp);
        assert_eq!(0x0123, vm.stack[1]);
    }

    #[test]
    fn vm_executes_skip_on_equal_byte_instruction_with_equal_values() {
        let instruction = Instruction::decode(0x32AB).unwrap();

        let mut vm: VM = Default::default();
        vm.boot();

        vm.registers[2] = 0xAB; // same value as the fixture

        vm.exec(instruction);

        assert_eq!(0x0002, vm.pc);
    }

    #[test]
    fn vm_executes_skip_on_equal_byte_instruction_with_diff_values() {
        let instruction = Instruction::decode(0x32AB).unwrap();

        let mut vm: VM = Default::default();
        vm.boot();

        vm.registers[2] = 0xAF; // different value as the fixture

        vm.exec(instruction);

        assert_eq!(0x0000, vm.pc);
    }

    #[test]
    fn vm_executes_skip_on_not_equal_byte_instruction_with_equal_values() {
        let instruction = Instruction::decode(0x42AB).unwrap();

        let mut vm: VM = Default::default();
        vm.boot();

        vm.registers[2] = 0xAB; // same value as the fixture

        vm.exec(instruction);

        assert_eq!(0x0000, vm.pc);
    }

    #[test]
    fn vm_executes_skip_on_not_equal_byte_instruction_with_diff_values() {
        let instruction = Instruction::decode(0x42AB).unwrap();

        let mut vm: VM = Default::default();
        vm.boot();

        vm.registers[2] = 0xAF; // different value as the fixture

        vm.exec(instruction);

        assert_eq!(0x0002, vm.pc);
    }

    #[test]
    fn vm_executes_skip_on_equal_instruction_with_equal_values() {
        let instruction = Instruction::decode(0x5280).unwrap();

        let mut vm: VM = Default::default();
        vm.boot();

        vm.registers[2] = 0xAB;
        vm.registers[8] = 0xAB;

        vm.exec(instruction);

        assert_eq!(0x0002, vm.pc);
    }

    #[test]
    fn vm_executes_skip_on_equal_instruction_with_diff_values() {
        let instruction = Instruction::decode(0x5280).unwrap();

        let mut vm: VM = Default::default();
        vm.boot();

        vm.registers[2] = 0xAF;
        vm.registers[8] = 0x12;

        vm.exec(instruction);

        assert_eq!(0x0000, vm.pc);
    }
}
