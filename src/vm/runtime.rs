use rand::{thread_rng, Rng};

use instructions::Opcode;
use vm::VM;
use display::Pixel;
use specs;

pub enum Next {
    Advance(u16),
    Noop,
}

pub fn clear(vm: &mut VM) -> Next {
    for pixel in vm.gfx.iter_mut() {
        *pixel = 0;
    }

    if let Some(ref bus) = vm.display_bus {
        let mut pixels: Vec<Pixel> = vec![];
        for x in 0..specs::DISPLAY_WIDTH {
            for y in 0..specs::DISPLAY_HEIGHT {
                let pixel = Pixel::new(x as i32, y as i32, 0);

                pixels.push(pixel);
            }
        }

        bus.send(pixels).unwrap();
    };

    Next::Advance(1)
}

pub fn ret(vm: &mut VM) -> Next {
    vm.pc = vm.stack[vm.sp] as usize;
    vm.sp -= 1;

    Next::Advance(1)
}

pub fn jump(vm: &mut VM, opcode: Opcode) -> Next {
    vm.pc = opcode.address as usize;

    Next::Noop
}

pub fn call(vm: &mut VM, opcode: Opcode) -> Next {
    vm.sp += 1;
    vm.stack[vm.sp] = vm.pc as u16;
    vm.pc = opcode.address as usize;

    Next::Noop
}

pub fn skip_on_equal_byte(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize];
    if vx == opcode.data {
        Next::Advance(2)
    } else {
        Next::Advance(1)
    }
}

pub fn skip_on_not_equal_byte(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize];
    if vx != opcode.data {
        Next::Advance(2)
    } else {
        Next::Advance(1)
    }
}

pub fn skip_on_equal(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize];
    let vy = vm.registers[opcode.y as usize];
    if vx == vy {
        Next::Advance(2)
    } else {
        Next::Advance(1)
    }
}

pub fn skip_on_not_equal(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize];
    let vy = vm.registers[opcode.y as usize];
    if vx != vy {
        Next::Advance(2)
    } else {
        Next::Advance(1)
    }
}

pub fn set_byte(vm: &mut VM, opcode: Opcode) -> Next {
    vm.registers[opcode.x as usize] = opcode.data;

    Next::Advance(1)
}

pub fn add_byte(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize];
    vm.registers[opcode.x as usize] = vx.wrapping_add(opcode.data);

    Next::Advance(1)
}

pub fn set(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];
    vm.registers[opcode.x as usize] = vy;

    Next::Advance(1)
}

pub fn or(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    vm.registers[opcode.x as usize] = vx | vy;

    Next::Advance(1)
}

pub fn and(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    vm.registers[opcode.x as usize] = vx & vy;

    Next::Advance(1)
}

pub fn xor(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    vm.registers[opcode.x as usize] = vx ^ vy;

    Next::Advance(1)
}

pub fn add(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize] as u16;
    let vx = vm.registers[opcode.x as usize] as u16;
    let add = vx + vy;

    if add > 0xFF {
        vm.registers[0xF] = 1;
    } else {
        vm.registers[0xF] = 0;
    }

    vm.registers[opcode.x as usize] = add as u8;

    Next::Advance(1)
}

pub fn sub_x_y(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    if vx > vy {
        vm.registers[0xF] = 1;
    } else {
        vm.registers[0xF] = 0;
    }

    vm.registers[opcode.x as usize] = vx.wrapping_sub(vy);

    Next::Advance(1)
}

pub fn sub_y_x(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];
    let vx = vm.registers[opcode.x as usize];

    if vy > vx {
        vm.registers[0xF] = 1;
    } else {
        vm.registers[0xF] = 0;
    }

    vm.registers[opcode.x as usize] = vy.wrapping_sub(vx);

    Next::Advance(1)
}

pub fn shift_right(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];

    vm.registers[0xF] = vy & 0x1;
    vm.registers[opcode.x as usize] = vy >> 1;

    Next::Advance(1)
}

pub fn shift_left(vm: &mut VM, opcode: Opcode) -> Next {
    let vy = vm.registers[opcode.y as usize];

    vm.registers[0xF] = (vy >> 7) & 0x1;
    vm.registers[opcode.x as usize] = vy << 1;

    Next::Advance(1)
}

pub fn set_i(vm: &mut VM, opcode: Opcode) -> Next {
    vm.i = opcode.address as usize;

    Next::Advance(1)
}

pub fn jump_plus(vm: &mut VM, opcode: Opcode) -> Next {
    let v0 = vm.registers[0] as u16;

    vm.pc = (v0 + opcode.address) as usize;

    Next::Noop
}

pub fn random_mask(vm: &mut VM, opcode: Opcode) -> Next {
    let mut rng = thread_rng();
    let rnd: u16 = rng.gen_range(0, 256);
    let rnd: u8 = rnd as u8;

    vm.registers[opcode.x as usize] = rnd & opcode.data;

    Next::Advance(1)
}

pub fn draw(vm: &mut VM, opcode: Opcode) -> Next {
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

    Next::Advance(1)
}

pub fn skip_on_key_pressed(vm: &mut VM, opcode: Opcode) -> Next {
    let key = vm.registers[opcode.x as usize] as usize;

    if vm.keypad[key] > 0 {
        vm.keypad[key] -= 1;
        Next::Advance(2)
    } else {
        Next::Advance(1)
    }
}

pub fn skip_on_key_not_pressed(vm: &mut VM, opcode: Opcode) -> Next {
    let key = vm.registers[opcode.x as usize] as usize;

    if vm.keypad[key] == 0 {
        Next::Advance(2)
    } else {
        vm.keypad[key] -= 1;
        Next::Advance(1)
    }
}

pub fn store_delay_timer(vm: &mut VM, opcode: Opcode) -> Next {
    vm.registers[opcode.x as usize] = vm.dt;

    Next::Advance(1)
}

pub fn set_delay_timer(vm: &mut VM, opcode: Opcode) -> Next {
    vm.dt = vm.registers[opcode.x as usize];

    Next::Advance(1)
}

pub fn set_sound_timer(vm: &mut VM, opcode: Opcode) -> Next {
    vm.st = vm.registers[opcode.x as usize];

    Next::Advance(1)
}

pub fn wait_key(vm: &mut VM, opcode: Opcode) -> Next {
    let key = vm.keypad.iter().position(|&s| s > 0);
    match key {
        Some(value) => {
            vm.registers[opcode.x as usize] = value as u8;
            vm.keypad[value] -= 1;

            Next::Advance(1)
        }
        None => Next::Noop,
    }
}

pub fn add_i(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize] as u16;
    vm.i += vx as usize;

    Next::Advance(1)
}

pub fn set_sprite(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize] as usize;
    vm.i = specs::SPRITES_ADDR + vx * specs::SPRITE_HEIGHT;

    Next::Advance(1)
}

pub fn bcd(vm: &mut VM, opcode: Opcode) -> Next {
    let vx = vm.registers[opcode.x as usize];

    let b = vx / 100;
    let c = (vx - (b * 100)) / 10;
    let d = vx - (b * 100) - (c * 10);

    vm.ram[vm.i] = b as u8;
    vm.ram[(vm.i + 1)] = c as u8;
    vm.ram[(vm.i + 2)] = d as u8;

    Next::Advance(1)
}

pub fn store(vm: &mut VM, opcode: Opcode) -> Next {
    for v in 0..opcode.x {
        let pointer = vm.i + v as usize;
        vm.ram[pointer] = vm.registers[v as usize];
    }

    vm.i += (opcode.x + 1) as usize;

    Next::Advance(1)
}

pub fn read(vm: &mut VM, opcode: Opcode) -> Next {
    for v in 0..opcode.x {
        let pointer = vm.i + v as usize;
        vm.registers[v as usize] = vm.ram[pointer];
    }

    vm.i += (opcode.x + 1) as usize;

    Next::Advance(1)
}
