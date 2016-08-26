// Trying to emulate a chip-8 computer!

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

pub struct VM {
    ram: [u8; RAM_SIZE],                     // Memory
    registers: [u8; GENERAL_REGISTERS_SIZE], // V0 - VF registers
    stack: [u16; STACK_SIZE],                // Stack for return addresses of subroutines
    keypad: [u8; KEYPAD_SIZE],               // Keep track of any key pressed in the keypad.

    i: u16,                                  // Store memory addresses

    dt: u8,                                  // Delay Timer register
    st: u8,                                  // Sound Timer register

    pc: u16,                                 // Program Counter
    sp: u8,                                  // Stack Pointer
}

impl Default for VM {
    fn default() -> VM {
        let mut vm = VM {
            ram: [0; RAM_SIZE],
            registers: [0; GENERAL_REGISTERS_SIZE],
            stack: [0; STACK_SIZE],
            keypad: [0; KEYPAD_SIZE],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
        };

        {
            // Copy all fonts to memory
            // TODO: Try to move this to another function/struct
            let mut buf = BufWriter::new(&mut vm.ram[FONTS_ADDR..(FONTS_ADDR + FONTS_SIZE)]);
            buf.write_all(FONTS.as_ref()).unwrap();
        };

        vm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_has_4k_of_memory() {
        let vm: VM = Default::default();

        assert_eq!(4096, vm.ram.len());
    }

    #[test]
    fn vm_has_the_fonts_in_memory() {
        let vm: VM = Default::default();

        assert_eq!([0xF0, 0x90, 0x90, 0x90, 0xF0], vm.ram[0..5]);   //0
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

        assert_eq!(0 as u8, vm.sp);
    }

    #[test]
    fn vm_has_a_stack() {
        let vm: VM = Default::default();

        assert_eq!([0 as u16; 16], vm.stack);
    }

    #[test]
    fn vm_has_keeps_tracks_of_the_keypad() {
        let vm: VM = Default::default();

        assert_eq!([0 as u8; 16], vm.keypad);
    }
}
