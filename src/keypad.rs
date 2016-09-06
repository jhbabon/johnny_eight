#[derive(Debug,Clone,Copy)]
pub enum Key {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Key {
    pub fn as_usize(&self) -> usize {
        match *self {
            Key::Num0 => 0x0,
            Key::Num1 => 0x1,
            Key::Num2 => 0x2,
            Key::Num3 => 0x3,
            Key::Num4 => 0x4,
            Key::Num5 => 0x5,
            Key::Num6 => 0x6,
            Key::Num7 => 0x7,
            Key::Num8 => 0x8,
            Key::Num9 => 0x9,
            Key::A    => 0xA,
            Key::B    => 0xB,
            Key::C    => 0xC,
            Key::D    => 0xD,
            Key::E    => 0xE,
            Key::F    => 0xF,
        }
    }
}
