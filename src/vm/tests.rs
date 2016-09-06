#[cfg(test)]

use vm::bootstrap::*;
use instructions::Instruction;
use vm::specs::*;

#[test]
fn vm_advances_the_pc() {
    let mut vm = Bootstrap::new().finish();
    vm.advance();

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_advances_the_pc_x_times() {
    let mut vm = Bootstrap::new().finish();
    vm.advance_by(2);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn vm_executes_clear_instruction() {
    let instruction = Instruction::decode(0x00E0).unwrap();
    let mut vm = Bootstrap::new().finish();

    vm.gfx = [1; DISPLAY_PIXELS];

    vm.exec(instruction);

    assert!(vm.gfx.iter().all(|&x| x == 0));
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_return_instruction() {
    let instruction = Instruction::decode(0x00EE).unwrap();
    let mut stack = [0; 16];
    stack[1] = 0xA1;

    let mut vm = Bootstrap::new().finish();
    vm.stack = stack;
    vm.sp = 1;
    vm.pc = 0;

    vm.exec(instruction);

    assert_eq!(0xA1 + 2, vm.pc);
    assert_eq!(0, vm.sp);
}

#[test]
fn vm_executes_jump_instruction() {
    let instruction = Instruction::decode(0x1FA1).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.exec(instruction);

    assert_eq!(0x0FA1, vm.pc);
}

#[test]
fn vm_executes_call_instruction() {
    let instruction = Instruction::decode(0x2FA1).unwrap();

    let mut vm = Bootstrap::new().finish();
    vm.pc = 0x0123;

    vm.exec(instruction);

    assert_eq!(0x0FA1, vm.pc);
    assert_eq!(1, vm.sp);
    assert_eq!(0x0123, vm.stack[1]);
}

#[test]
fn vm_executes_skip_on_equal_byte_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x32AB).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAB; // same value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn vm_executes_skip_on_equal_byte_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x32AB).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAF; // different value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_skip_on_not_equal_byte_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x42AB).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAB; // same value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_skip_on_not_equal_byte_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x42AB).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAF; // different value as the fixture

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn vm_executes_skip_on_equal_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x5280).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAB;
    vm.registers[0x8] = 0xAB;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn vm_executes_skip_on_equal_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x5280).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAF;
    vm.registers[0x8] = 0x12;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_skip_on_not_equal_instruction_with_equal_values() {
    let instruction = Instruction::decode(0x9280).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAB;
    vm.registers[0x8] = 0xAB;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_skip_on_not_equal_instruction_with_diff_values() {
    let instruction = Instruction::decode(0x9280).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0xAF;
    vm.registers[0x8] = 0x12;

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn vm_executes_set_byte_instruction() {
    let instruction = Instruction::decode(0x62AB).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.exec(instruction);

    assert_eq!(0xAB, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_add_byte_instruction() {
    let instruction = Instruction::decode(0x7211).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x11;

    vm.exec(instruction);

    assert_eq!(0x22, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_set_instruction() {
    let instruction = Instruction::decode(0x8210).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x11;
    vm.registers[0x1] = 0xAB;

    vm.exec(instruction);

    assert_eq!(0xAB, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_or_instruction() {
    let instruction = Instruction::decode(0x8211).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x11; // Vx
    vm.registers[0x1] = 0xAB; // Vy

    vm.exec(instruction);

    assert_eq!(0xBB, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_and_instruction() {
    let instruction = Instruction::decode(0x8212).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x11; // Vx
    vm.registers[0x1] = 0xAB; // Vy

    vm.exec(instruction);

    assert_eq!(0x01, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_xor_instruction() {
    let instruction = Instruction::decode(0x8213).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x11; // Vx
    vm.registers[0x1] = 0xAB; // Vy

    vm.exec(instruction);

    assert_eq!(0xBA, vm.registers[0x2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_add_instruction_with_carry() {
    let instruction = Instruction::decode(0x8214).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x83; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0x0, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_add_instruction_without_carry() {
    let instruction = Instruction::decode(0x8214).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x82; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0xFF, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_sub_x_y_instruction_with_borrow() {
    let instruction = Instruction::decode(0x8215).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0x82; // Vy

    vm.exec(instruction);

    assert_eq!(0xFB, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_sub_x_y_instruction_without_borrow() {
    let instruction = Instruction::decode(0x8215).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x82; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0x5, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_sub_y_x_instruction_with_borrow() {
    let instruction = Instruction::decode(0x8217).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x82; // Vx
    vm.registers[0x1] = 0x7D; // Vy

    vm.exec(instruction);

    assert_eq!(0xFB, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_sub_y_x_instruction_without_borrow() {
    let instruction = Instruction::decode(0x8217).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0x82; // Vy

    vm.exec(instruction);

    assert_eq!(0x5, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_shift_right_instruction_with_carry() {
    let instruction = Instruction::decode(0x8216).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0xFF; // Vy

    vm.exec(instruction);

    assert_eq!(0x7F, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_shift_right_instruction_without_carry() {
    let instruction = Instruction::decode(0x8216).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0xFE; // Vy

    vm.exec(instruction);

    assert_eq!(0x7F, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_shift_left_instruction_with_carry() {
    let instruction = Instruction::decode(0x821E).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0xFF; // Vy

    vm.exec(instruction);

    assert_eq!(0xFE, vm.registers[0x2]);
    assert_eq!(0x1, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_shift_left_instruction_without_carry() {
    let instruction = Instruction::decode(0x821E).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x2] = 0x7D; // Vx
    vm.registers[0x1] = 0x7F; // Vy

    vm.exec(instruction);

    assert_eq!(0xFE, vm.registers[0x2]);
    assert_eq!(0x0, vm.registers[0xF]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_set_i_instruction() {
    let instruction = Instruction::decode(0xA21E).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.i = 0x007D;

    vm.exec(instruction);

    assert_eq!(0x021E, vm.i);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_jump_plus_instruction() {
    let instruction = Instruction::decode(0xBABC).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0x0] = 0x1E;

    vm.exec(instruction);

    let expected = 0x0ABC + 0x001E;

    assert_eq!(expected, vm.pc);
}

#[test]
fn vm_executes_random_mask_instruction() {
    let instruction = Instruction::decode(0xCABC).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0x1E;

    vm.exec(instruction);

    assert!(vm.registers[0xA] != 0x1E);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

// #[test]
// TODO
// fn vm_executes_draw_instruction() {
//     let instruction = Instruction::decode(0xD123).unwrap();

//     let mut vm = Bootstrap::new().finish();

//     vm.registers[0x1] = 0x1;
//     vm.registers[0x2] = 0x2;
//     vm.i = 0x1;
//     vm.ram[0x1] = 0xFF;
//     vm.ram[0x2] = 0xFF;
//     vm.ram[0x3] = 0xFF;

//     vm.exec(instruction);
//
//     // TODO: What is expected?
//     let mut expected: [u8; (64 * 32)] = [0; (64 * 32)];

//     assert_eq!(expected, vm.gfx);
// }

#[test]
fn vm_executes_skip_on_key_pressed_instruction_when_key_is_pressed() {
    let instruction = Instruction::decode(0xEA9E).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0xF; // We look for key F
    vm.keypad[0xF] = 1;      // key F is pressed

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn vm_executes_skip_on_key_pressed_instruction_when_key_is_not_pressed() {
    let instruction = Instruction::decode(0xEA9E).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0xF; // We look for key F
    vm.keypad[0xF] = 0;      // key F is not pressed

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_skip_on_key_not_pressed_instruction_when_key_is_pressed() {
    let instruction = Instruction::decode(0xEAA1).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0xF; // We look for key F
    vm.keypad[0xF] = 1;      // key F is pressed

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_skip_on_key_not_pressed_instruction_when_key_is_not_pressed() {
    let instruction = Instruction::decode(0xEAA1).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0xF; // We look for key F
    vm.keypad[0xF] = 0;      // key F is not pressed

    vm.exec(instruction);

    assert_eq!(PROGRAM_START + 4, vm.pc);
}

#[test]
fn vm_executes_store_delay_timer_instruction() {
    let instruction = Instruction::decode(0xFA07).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.dt = 0xE;

    vm.exec(instruction);

    assert_eq!(0xE, vm.registers[0xA]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_set_delay_timer_instruction() {
    let instruction = Instruction::decode(0xFA15).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0xE;

    vm.exec(instruction);

    assert_eq!(0xE, vm.dt);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_set_sound_timer_instruction() {
    let instruction = Instruction::decode(0xFA18).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0xE;

    vm.exec(instruction);

    assert_eq!(0xE, vm.st);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_wait_key_instruction_without_any_key_pressed() {
    let instruction = Instruction::decode(0xFA0A).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.exec(instruction);

    assert_eq!(0x0, vm.registers[0xA]);
    assert_eq!(PROGRAM_START, vm.pc); // It doesn't move
}

#[test]
fn vm_executes_wait_key_instruction_with_a_key_pressed() {
    let instruction = Instruction::decode(0xFA0A).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.keypad[0xB] = 1;

    vm.exec(instruction);

    assert_eq!(0xB, vm.registers[0xA]);
    assert_eq!(PROGRAM_START + 2, vm.pc); // It moves
}

#[test]
fn vm_executes_add_i_instruction() {
    let instruction = Instruction::decode(0xFA1E).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.i = 0x1;
    vm.registers[0xA] = 0x1;

    vm.exec(instruction);

    assert_eq!(0x2, vm.i);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_set_sprite_instruction() {
    let instruction = Instruction::decode(0xFA29).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.registers[0xA] = 0xA;

    let expected = SPRITES_ADDR + 0xA * SPRITE_HEIGHT;

    vm.exec(instruction);

    assert_eq!(expected, vm.i);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_bcd_instruction() {
    let instruction = Instruction::decode(0xFA33).unwrap();

    let mut vm = Bootstrap::new().finish();

    vm.i = 0x0FF0;
    vm.registers[0xA] = 254;

    vm.exec(instruction);

    assert_eq!(2, vm.ram[0xFF0]);
    assert_eq!(5, vm.ram[0xFF1]);
    assert_eq!(4, vm.ram[0xFF2]);
    assert_eq!(PROGRAM_START + 2, vm.pc);
}

#[test]
fn vm_executes_store_instruction() {
    let instruction = Instruction::decode(0xFE55).unwrap();

    let mut vm = Bootstrap::new().finish();

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
fn vm_executes_read_instruction() {
    let instruction = Instruction::decode(0xFE65).unwrap();

    let mut vm = Bootstrap::new().finish();

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
