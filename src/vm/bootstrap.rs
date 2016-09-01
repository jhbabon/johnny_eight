use vm::*;
use vm::specs::*;
use std::io::Read;

pub struct Bootstrap {
    ram: [u8; RAM_SIZE],
}

impl Bootstrap {
    pub fn new() -> Bootstrap {
        println!("Initializing Chip-8 VM");
        Bootstrap {
            ram: [0; RAM_SIZE],
        }
    }

    pub fn load_sprites(mut self) -> Bootstrap {
        println!("Loading stripes into memory");
        let range = SPRITES_ADDR..(SPRITES_ADDR + SPRITES_SIZE);
        for addr in range {
            let index = (addr - SPRITES_ADDR) as usize;
            self.ram[addr] = SPRITES[index];
        }

        self
    }

    pub fn load_rom(mut self, reader: &mut Read) -> Bootstrap {
        println!("Loading ROM into memory");
        let mut rom = Vec::new();
        if let Err(_) = reader.read_to_end(&mut rom) {
            panic!("Error reading ROM");
        }

        let mut addr = PROGRAM_START;
        println!("ROM contents:");
        for byte in &rom {
            println!("{:#X}", *byte);
            self.ram[addr] = *byte;
            addr += 1;
        }

        self
    }

    pub fn finish(self) -> VM {
        println!("VM Loaded");

        VM {
            ram:       self.ram,
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
    use vm::specs::*;
    use std::io::Cursor;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;

    #[test]
    fn bootstrap_loads_an_empty_vm_by_default() {
        let vm = Bootstrap::new().finish();

        assert_eq!(RAM_SIZE, vm.ram.len());
        assert_eq!([0 as u8; GENERAL_REGISTERS_SIZE], vm.registers);
        assert_eq!([0 as u16; STACK_SIZE], vm.stack);
        assert_eq!([0 as u8; KEYPAD_SIZE], vm.keypad);
        assert_eq!(DISPLAY_PIXELS, vm.gfx.len());

        assert_eq!(0 as usize, vm.i);
        assert_eq!(0 as usize, vm.pc);
        assert_eq!(0 as usize, vm.sp);
        assert_eq!(0 as u8, vm.dt);
        assert_eq!(0 as u8, vm.st);
    }

    #[test]
    fn bootstrap_loads_the_sprites() {
        let vm = Bootstrap::new().load_sprites().finish();

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
    fn bootstrap_loads_a_rom() {
        let rom: Vec<u8> = vec![0xA; RAM_SIZE - PROGRAM_START - 10];
        let mut reader = Cursor::new(rom);

        let vm = Bootstrap::new().load_rom(&mut reader).finish();
        let range = PROGRAM_START..(RAM_SIZE - 10);

        assert!(vm.ram[range].iter().all(|&x| x == 0xA));
    }

    #[test]
    fn bootstrap_loads_a_rom_from_a_file() {
        let rom_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/chip_8_picture.rom"
        );
        let mut rom = File::open(rom_path).unwrap();

        let vm = Bootstrap::new().load_rom(&mut rom).finish();

        let txt_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/chip_8_picture.txt"
        );
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
}
