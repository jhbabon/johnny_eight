#[cfg(test)]

use instructions::Instruction;
use keypad::Key;
use specs::*;
use vm::{VM, Tick};
use std::io::Cursor;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::mpsc::{channel, Sender, Receiver};

#[test]
fn loads_an_empty_vm_by_default() {
    let vm = VM::boot();

    assert_eq!(RAM_SIZE, vm.ram.len());
    assert_eq!([0 as u8; GENERAL_REGISTERS_SIZE], vm.registers);
    assert_eq!([0 as u16; STACK_SIZE], vm.stack);
    assert_eq!([0 as u8; KEYPAD_SIZE], vm.keypad);
    assert_eq!(DISPLAY_PIXELS, vm.gfx.len());

    assert_eq!(PROGRAM_START, vm.pc);
    assert_eq!(0 as usize, vm.i);
    assert_eq!(0 as usize, vm.sp);
    assert_eq!(0 as u8, vm.dt);
    assert_eq!(0 as u8, vm.st);
}

#[test]
fn loads_the_sprites() {
    let mut vm = VM::boot();
    vm.load_sprites();

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
fn loads_a_rom() {
    let rom: Vec<u8> = vec![0xA; RAM_SIZE - PROGRAM_START - 10];
    let mut reader = Cursor::new(rom);

    let mut vm = VM::boot();
    vm.load_rom(&mut reader);
    let range = PROGRAM_START..(RAM_SIZE - 10);

    assert!(vm.ram[range].iter().all(|&x| x == 0xA));
}

#[test]
fn inits_the_clock() {
    let mut vm = VM::boot();
    vm.init_clock();

    assert!(vm.clock.is_some());
}

#[test]
fn the_clock_ticks() {
    let mut vm = VM::boot();
    vm.init_clock();

    let clock = vm.clock.unwrap();
    assert_eq!(Tick, clock.recv().unwrap());
}

#[test]
fn loads_a_rom_from_a_file() {
    let rom_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/chip_8_logo.rom");
    let mut rom = File::open(rom_path).unwrap();

    let mut vm = VM::boot();
    vm.load_rom(&mut rom);

    let txt_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/chip_8_logo.txt");
    let txt = File::open(txt_path).unwrap();
    let file = BufReader::new(&txt);
    let mut index = PROGRAM_START;
    for line in file.lines() {
        let l = line.unwrap();
        let value = format!("{:#X}", vm.ram[index]);
        index += 1;
        assert_eq!(l, value);
    }
}

#[test]
fn advances_the_pc() {
    let mut vm = VM::boot();
    vm.advance();

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn advances_the_pc_x_times() {
    let mut vm = VM::boot();
    vm.advance_by(2);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn sets_a_key() {
    let mut vm = VM::boot();
    let key = Key::A;

    vm.set_key(key);

    assert_eq!(1, vm.keypad[0xA]);
}

#[test]
fn sets_a_key_more_than_once() {
    let mut vm = VM::boot();

    vm.set_key(Key::A);
    vm.set_key(Key::A);

    assert_eq!(2, vm.keypad[0xA]);
}

#[test]
fn cycles_on_clock_tick() {
    let rom_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/chip_8_logo.rom");
    let mut rom = File::open(rom_path).unwrap();

    let mut vm = VM::boot();
    vm.load_rom(&mut rom);

    // We need to force the clock tick
    let (ticker, clock): (Sender<Tick>, Receiver<Tick>) = channel();
    vm.clock = Some(clock);
    ticker.send(Tick).unwrap();

    vm.gfx = [1; DISPLAY_PIXELS];
    vm.dt = 1;
    vm.st = 1;

    vm.cycle();

    assert_eq!(PROGRAM_START + 2, vm.pc);
    assert_eq!(0, vm.dt);
    assert_eq!(0, vm.st);
    // The first instruction of the ROM is Clear
    assert!(vm.gfx.iter().all(|&x| x == 0));
}

#[test]
fn does_not_cycle_without_tick() {
    let rom_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/chip_8_logo.rom");
    let mut rom = File::open(rom_path).unwrap();

    let mut vm = VM::boot();
    vm.load_rom(&mut rom);

    let (_ticker, clock): (Sender<Tick>, Receiver<Tick>) = channel();
    vm.clock = Some(clock);
    // We don't send anything to the channel

    vm.gfx = [1; DISPLAY_PIXELS];
    vm.dt = 1;
    vm.st = 1;

    vm.cycle();

    assert_eq!(PROGRAM_START, vm.pc);
    assert_eq!(1, vm.dt);
    assert_eq!(1, vm.st);
    assert!(vm.gfx.iter().all(|&x| x == 1));
}

#[test]
fn executes_clear_instruction() {
    let instruction = Instruction::decode(0x00E0).unwrap();
    let mut vm = VM::boot();

    vm.gfx = [1; DISPLAY_PIXELS];

    vm.exec(instruction);

    assert!(vm.gfx.iter().all(|&x| x == 0));
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_return_instruction() {
    let instruction = Instruction::decode(0x00EE).unwrap();
    let mut stack = [0; 16];
    stack[1] = 0xA1;

    let mut vm = VM::boot();
    vm.stack = stack;
    vm.sp = 1;
    vm.pc = 0;

    vm.exec(instruction);

    assert_eq!(0xA1 + 2, vm.pc);
    assert_eq!(0, vm.sp);
}

#[test]
fn executes_jump_instruction() {
    let instruction = Instruction::decode(0x1FA1).unwrap();

    let mut vm = VM::boot();

    vm.exec(instruction);

    assert_eq!(0x0FA1, vm.pc);
}

#[test]
fn executes_call_instruction() {
    let instruction = Instruction::decode(0x2FA1).unwrap();

    let mut vm = VM::boot();
    vm.pc = 0x0123;

    vm.exec(instruction);

    assert_eq!(0x0FA1, vm.pc);
    assert_eq!(1, vm.sp);
    assert_eq!(0x0123, vm.stack[1]);
}

#[test]
fn executes_skip_on_equal_byte_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x32AB).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAB; // same value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn executes_skip_on_equal_byte_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x32AB).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAF; // different value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_skip_on_not_equal_byte_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x42AB).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAB; // same value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_skip_on_not_equal_byte_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x42AB).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAF; // different value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn executes_skip_on_equal_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x5280).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAB;
    vm.registers[0x8] = 0xAB;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn executes_skip_on_equal_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x5280).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAF;
    vm.registers[0x8] = 0x12;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_skip_on_not_equal_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x9280).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAB;
    vm.registers[0x8] = 0xAB;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_skip_on_not_equal_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x9280).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0xAF;
    vm.registers[0x8] = 0x12;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn executes_set_byte_instruction() {
    let instruction = Instruction::decode(0x62AB).unwrap();

    let mut vm = VM::boot();

    vm.exec(instruction);

    assert_eq!(0xAB, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_add_byte_instruction() {
    let instruction = Instruction::decode(0x7211).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x11;

    vm.exec(instruction);

    assert_eq!(0x22, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_set_instruction() {
    let instruction = Instruction::decode(0x8210).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x11;
    vm.registers[0x1] = 0xAB;

    vm.exec(instruction);

    assert_eq!(0xAB, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_or_instruction() {
    let instruction = Instruction::decode(0x8211).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x11; // Vx
    vm.registers[0x1] = 0xAB; // Vy

    vm.exec(instruction);

    assert_eq!(0xBB, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_and_instruction() {
    let instruction = Instruction::decode(0x8212).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x11; // Vx
    vm.registers[0x1] = 0xAB; // Vy

    vm.exec(instruction);

    assert_eq!(0x01, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_xor_instruction() {
    let instruction = Instruction::decode(0x8213).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x11; // Vx
    vm.registers[0x1] = 0xAB; // Vy

    vm.exec(instruction);

    assert_eq!(0xBA, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_add_instruction_with_carry() {
    let instruction = Instruction::decode(0x8214).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x83; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0x0, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_add_instruction_without_carry() {
    let instruction = Instruction::decode(0x8214).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x82; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0xFF, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_sub_x_y_instruction_with_borrow() {
    let instruction = Instruction::decode(0x8215).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0x82; // Vy

    vm.exec(instruction);

    assert_eq!(0xFB, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_sub_x_y_instruction_without_borrow() {
    let instruction = Instruction::decode(0x8215).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x82; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0x5, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_sub_y_x_instruction_with_borrow() {
    let instruction = Instruction::decode(0x8217).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x82; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0xFB, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_sub_y_x_instruction_without_borrow() {
    let instruction = Instruction::decode(0x8217).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0x82; // Vy

    vm.exec(instruction);

    assert_eq!(0x5, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_shift_right_instruction_with_carry() {
    let instruction = Instruction::decode(0x8216).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0xFF; // Vy

    vm.exec(instruction);

    assert_eq!(0x7F, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_shift_right_instruction_without_carry() {
    let instruction = Instruction::decode(0x8216).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0xFE; // Vy

    vm.exec(instruction);

    assert_eq!(0x7F, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_shift_left_instruction_with_carry() {
    let instruction = Instruction::decode(0x821E).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0xFF; // Vy

    vm.exec(instruction);

    assert_eq!(0xFE, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_shift_left_instruction_without_carry() {
    let instruction = Instruction::decode(0x821E).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0x7F; // Vy

    vm.exec(instruction);

    assert_eq!(0xFE, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_set_i_instruction() {
    let instruction = Instruction::decode(0xA21E).unwrap();

    let mut vm = VM::boot();

    vm.i = 0x007D;

    vm.exec(instruction);

    assert_eq!(0x021E, vm.i);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_jump_plus_instruction() {
    let instruction = Instruction::decode(0xBABC).unwrap();

    let mut vm = VM::boot();

    vm.registers[0x0] = 0x1E;

    vm.exec(instruction);

    let expected = 0x0ABC + 0x001E;

    assert_eq!(expected, vm.pc);
}

#[test]
fn executes_random_mask_instruction() {
    let instruction = Instruction::decode(0xCABC).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0x1E;

    vm.exec(instruction);

    assert!(vm.registers[0xA] != 0x1E);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_skip_on_key_pressed_instruction_when_key_is_pressed() {
    let instruction = Instruction::decode(0xEA9E).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0xF; // We look for key F
    vm.set_key(Key::F);      // key F is pressed

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn executes_skip_on_key_pressed_instruction_when_key_is_not_pressed() {
    let instruction = Instruction::decode(0xEA9E).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0xF; // We look for key F

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_skip_on_key_not_pressed_instruction_when_key_is_pressed() {
    let instruction = Instruction::decode(0xEAA1).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0xF; // We look for key F
    vm.set_key(Key::F);      // key F is pressed

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_skip_on_key_not_pressed_instruction_when_key_is_not_pressed() {
    let instruction = Instruction::decode(0xEAA1).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0xF; // We look for key F

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn executes_store_delay_timer_instruction() {
    let instruction = Instruction::decode(0xFA07).unwrap();

    let mut vm = VM::boot();

    vm.dt = 0xE;

    vm.exec(instruction);

    assert_eq!(0xE, vm.registers[0xA]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_set_delay_timer_instruction() {
    let instruction = Instruction::decode(0xFA15).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0xE;

    vm.exec(instruction);

    assert_eq!(0xE, vm.dt);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_set_sound_timer_instruction() {
    let instruction = Instruction::decode(0xFA18).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0xE;

    vm.exec(instruction);

    assert_eq!(0xE, vm.st);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_wait_key_instruction_without_any_key_pressed() {
    let instruction = Instruction::decode(0xFA0A).unwrap();

    let mut vm = VM::boot();

    vm.exec(instruction);

    assert_eq!(0x0, vm.registers[0xA]);
    assert_eq!(PROGRAM_START, vm.pc); // It doesn't move
}

#[test]
fn executes_wait_key_instruction_with_a_key_pressed() {
    let instruction = Instruction::decode(0xFA0A).unwrap();

    let mut vm = VM::boot();

    vm.set_key(Key::B);

    vm.exec(instruction);

    assert_eq!(0xB, vm.registers[0xA]);
    assert_eq!(PROGRAM_START + 2, vm.pc); // It moves
}

#[test]
fn executes_add_i_instruction() {
    let instruction = Instruction::decode(0xFA1E).unwrap();

    let mut vm = VM::boot();

    vm.i = 0x1;
    vm.registers[0xA] = 0x1;

    vm.exec(instruction);

    assert_eq!(0x2, vm.i);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_set_sprite_instruction() {
    let instruction = Instruction::decode(0xFA29).unwrap();

    let mut vm = VM::boot();

    vm.registers[0xA] = 0xA;

    let expected = SPRITES_ADDR + 0xA * SPRITE_HEIGHT;

    vm.exec(instruction);

    assert_eq!(expected, vm.i);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_bcd_instruction() {
    let instruction = Instruction::decode(0xFA33).unwrap();

    let mut vm = VM::boot();

    vm.i = 0x0FF0;
    vm.registers[0xA] = 254;

    vm.exec(instruction);

    assert_eq!(2, vm.ram[0xFF0]);
    assert_eq!(5, vm.ram[0xFF1]);
    assert_eq!(4, vm.ram[0xFF2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn executes_store_instruction() {
    let instruction = Instruction::decode(0xFE55).unwrap();

    let mut vm = VM::boot();

    vm.i = 0x0F00;

    for i in 0x0..0xE {
        vm.registers[i] = 0xA;
    }

    vm.exec(instruction);

    for i in 0x0..0xE {
        let index = (0x0F00 + i) as usize;
        assert_eq!(0xA, vm.ram[index]);
    }
    assert_eq!(PROGRAM_START + 2, vm.pc);
    assert_eq!(0x0F0F, vm.i);
}

#[test]
fn executes_read_instruction() {
    let instruction = Instruction::decode(0xFE65).unwrap();

    let mut vm = VM::boot();

    vm.i = 0x0F00;

    for i in 0x0..0xE {
        vm.ram[0x0F00 + i] = 0xA;
    }

    vm.exec(instruction);

    for i in 0x0..0xE {
        assert_eq!(0xA, vm.registers[i as usize]);
    }
    assert_eq!(PROGRAM_START + 2, vm.pc);
    assert_eq!(0x0F0F, vm.i);
}
