extern crate chip_8;

use chip_8::vm::bootstrap::Bootstrap;
use std::fs::File;

fn main() {
    let rom_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/chip_8_picture.rom"
    );
    let mut rom = File::open(rom_path).unwrap();

    // TODO: Move this to a cli module?
    Bootstrap::new()
        .load_sprites()
        .load_rom(&mut rom)
        .finish();
}
