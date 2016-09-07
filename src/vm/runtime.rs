use rand::{thread_rng, Rng};

use instructions::Opcode;
use vm::VM;
use display::Pixel;
use specs;

pub fn clear(vm: &mut VM) {
    for pixel in vm.gfx.iter_mut() {
        *pixel = 0;
    }

    vm.advance();
}

pub fn ret(vm: &mut VM) {
    vm.pc = vm.stack[vm.sp] as usize;
    vm.sp -= 1;

    vm.advance();
}

pub fn jump(vm: &mut VM, opcode: Opcode) {
    vm.pc = opcode.address as usize;
}

pub fn call(vm: &mut VM, opcode: Opcode) {
    vm.sp += 1;
    vm.stack[vm.sp] = vm.pc as u16;
    vm.pc = opcode.address as usize;
}

pub fn skip_on_equal_byte(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize];
    if vx == opcode.data {
        vm.advance_by(2);
    } else {
        vm.advance();
    };
}

pub fn skip_on_not_equal_byte(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize];
    if vx != opcode.data {
        vm.advance_by(2);
    } else {
        vm.advance();
    };
}

pub fn skip_on_equal(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize];
    let vy = vm.registers[opcode.y as usize];
    if vx == vy {
        vm.advance_by(2);
    } else {
        vm.advance();
    };
}

pub fn skip_on_not_equal(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize];
    let vy = vm.registers[opcode.y as usize];
    if vx != vy {
        vm.advance_by(2);
    } else {
        vm.advance();
    };
}

pub fn set_byte(vm: &mut VM, opcode: Opcode) {
    vm.registers[opcode.x as usize] = opcode.data;

    vm.advance();
}

pub fn add_byte(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize];
    vm.registers[opcode.x as usize] = vx.wrapping_add(opcode.data);

    vm.advance();
}

pub fn set(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];
    vm.registers[opcode.x as usize] = vy;

    vm.advance();
}

pub fn or(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    vm.registers[opcode.x as usize] = vx | vy;

    vm.advance();
}

pub fn and(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    vm.registers[opcode.x as usize] = vx & vy;

    vm.advance();
}

pub fn xor(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    vm.registers[opcode.x as usize] = vx ^ vy;

    vm.advance();
}

pub fn add(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize] as u16;
    let vx = vm.registers[opcode.x as usize] as u16;
    let add = vx + vy;

    if add > 0xFF {
        vm.registers[0xF] = 1;
    } else {
        vm.registers[0xF] = 0;
    }

    vm.registers[opcode.x as usize] = add as u8;

    vm.advance();
}

pub fn sub_x_y(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    if vx > vy {
        vm.registers[0xF] = 1;
    } else {
        vm.registers[0xF] = 0;
    }

    vm.registers[opcode.x as usize] = vx.wrapping_sub(vy);

    vm.advance();
}

pub fn sub_y_x(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    if vy > vx {
        vm.registers[0xF] = 1;
    } else {
        vm.registers[0xF] = 0;
    }

    vm.registers[opcode.x as usize] = vy.wrapping_sub(vx);

    vm.advance();
}

pub fn shift_right(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];

    vm.registers[0xF] = vy & 0x1;
    vm.registers[opcode.x as usize] = vy >> 1;

    vm.advance();
}

pub fn shift_left(vm: &mut VM, opcode: Opcode) {
    let vy = vm.registers[opcode.y as usize];

    vm.registers[0xF] = (vy >> 7) & 0x1;
    vm.registers[opcode.x as usize] = vy << 1;

    vm.advance();
}

pub fn set_i(vm: &mut VM, opcode: Opcode) {
    vm.i = opcode.address as usize;

    vm.advance();
}

pub fn jump_plus(vm: &mut VM, opcode: Opcode) {
    let v0 = vm.registers[0] as u16;

    vm.pc = (v0 + opcode.address) as usize;
}

pub fn random_mask(vm: &mut VM, opcode: Opcode) {
    let mut rng = thread_rng();
    let rnd: u16 = rng.gen_range(0, 256);
    let rnd: u8 = rnd as u8;

    vm.registers[opcode.x as usize] = rnd & opcode.data;

    vm.advance();
}

pub fn draw(vm: &mut VM, opcode: Opcode) {
    let x = vm.registers[opcode.x as usize] as usize;
    let y = vm.registers[opcode.y as usize] as usize;
    let i = vm.i;
    let n = opcode.nibble as usize;

    let mut pixels: Vec<Pixel> = vec![];

    vm.registers[0xF] = 0;
    for (sy, byte) in vm.ram[i..i + n].iter().enumerate() {
        let dy = (y + sy) % specs::DISPLAY_HEIGHT;
        for sx in 0usize..8 {
            let px = (*byte >> (7 - sx)) & 0b00000001;
            let dx = (x + sx) % specs::DISPLAY_WIDTH;
            let idx = dy * specs::DISPLAY_WIDTH + dx;
            vm.gfx[idx] ^= px;

            // Vf is if there was a collision
            vm.registers[0xF] |= (vm.gfx[idx] == 0 && px == 1) as u8;

            let pixel = Pixel::new(dx as i32, dy as i32, vm.gfx[idx]);

            pixels.push(pixel);
        }
    }

    if let Some(ref bus) = vm.display_bus {
        bus.send(pixels).unwrap();
    };

    vm.advance();
}

pub fn skip_on_key_pressed(vm: &mut VM, opcode: Opcode) {
    let key = vm.registers[opcode.x as usize] as usize;

    if vm.keypad[key] > 0 {
        vm.keypad[key] -= 1;
        vm.advance_by(2);
    } else {
        vm.advance();
    };
}

pub fn skip_on_key_not_pressed(vm: &mut VM, opcode: Opcode) {
    let key = vm.registers[opcode.x as usize] as usize;

    if vm.keypad[key] == 0 {
        vm.advance_by(2);
    } else {
        vm.keypad[key] -= 1;
        vm.advance();
    };
}

pub fn store_delay_timer(vm: &mut VM, opcode: Opcode) {
    vm.registers[opcode.x as usize] = vm.dt;

    vm.advance();
}

pub fn set_delay_timer(vm: &mut VM, opcode: Opcode) {
    vm.dt = vm.registers[opcode.x as usize];

    vm.advance();
}

pub fn set_sound_timer(vm: &mut VM, opcode: Opcode) {
    vm.st = vm.registers[opcode.x as usize];

    vm.advance();
}

pub fn wait_key(vm: &mut VM, opcode: Opcode) {
    let key = vm.keypad.iter().position(|&s| s > 0);
    if let Some(value) = key {
        vm.registers[opcode.x as usize] = value as u8;
        vm.keypad[value] -= 1;
        vm.advance();
    }
}

pub fn add_i(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize] as u16;
    vm.i += vx as usize;

    vm.advance();
}

pub fn set_sprite(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize] as usize;
    vm.i = specs::SPRITES_ADDR + vx * specs::SPRITE_HEIGHT;

    vm.advance();
}

pub fn bcd(vm: &mut VM, opcode: Opcode) {
    let vx = vm.registers[opcode.x as usize];

    let b = vx / 100;
    let c = (vx - (b * 100)) / 10;
    let d = vx - (b * 100) - (c * 10);

    vm.ram[vm.i] = b as u8;
    vm.ram[(vm.i + 1)] = c as u8;
    vm.ram[(vm.i + 2)] = d as u8;

    vm.advance();
}

pub fn store(vm: &mut VM, opcode: Opcode) {
    for v in 0..opcode.x {
        let pointer = vm.i + v as usize;
        vm.ram[pointer] = vm.registers[v as usize];
    }

    vm.i += (opcode.x + 1) as usize;

    vm.advance();
}

pub fn read(vm: &mut VM, opcode: Opcode) {
    for v in 0..opcode.x {
        let pointer = vm.i + v as usize;
        vm.registers[v as usize] = vm.ram[pointer];
    }

    vm.i += (opcode.x + 1) as usize;

    vm.advance();
}
