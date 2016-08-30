const ADDRESS_MASK: u16 = 0x0FFF;
const VX_MASK: u16      = 0x0F00;
const VY_MASK: u16      = 0x00F0;
const DATA_MASK: u16    = 0x00FF;
const NIBBLE_MASK: u16  = 0x000F;
const ID_MASK: u16      = 0xF000;

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Opcode {
    bytes: u16,
    address: u16,
    x: u8,
    y: u8,
    data: u8,
    nibble: u8,
    id: u8,
}

impl Opcode {
    pub fn new(bytes: u16) -> Opcode {
        Opcode {
            bytes:   bytes,
            address: ADDRESS_MASK & bytes,
            x:       ((VX_MASK & bytes) >> 8) as u8,
            y:       ((VY_MASK & bytes) >> 4) as u8,
            data:    (DATA_MASK & bytes) as u8,
            nibble:  (NIBBLE_MASK & bytes) as u8,
            id:      ((ID_MASK & bytes) >> 12) as u8,
        }
    }
}

#[derive(PartialEq,Debug)]
pub enum Instruction {
    Clear,
    Return,
    Jump(Opcode),
    Call(Opcode),
    SkipEqualByte(Opcode),
    SkipNotEqualByte(Opcode),
    SkipEqual(Opcode),
    LoadByte(Opcode),
    AddByte(Opcode),
    Load(Opcode),
    Or(Opcode),
    And(Opcode),
    Xor(Opcode),
    Add(Opcode),
    SubXY(Opcode),
    ShiftRight(Opcode),
    SubYX(Opcode),
    ShiftLeft(Opcode),
    SkipNotEqual(Opcode),
    LoadI(Opcode),
    JumpPlus(Opcode),
    RandomMask(Opcode),
    Draw(Opcode),
    SkipOnKeyPressed(Opcode),
    SkipOnKeyNotPressed(Opcode),
    SaveDelayTimer(Opcode),
    WaitKey(Opcode),
    SetDelayTimer(Opcode),
    SetSoundTimer(Opcode),
    AddI(Opcode),
    LoadFont(Opcode),
    Bcd(Opcode),
    Write(Opcode),
    Read(Opcode),
}

pub fn decode(bytes: u16) -> Option<Instruction> {
    let opcode = Opcode::new(bytes);

    match opcode {
        Opcode { bytes: 0x00E0, .. } => Some(Instruction::Clear),
        Opcode { bytes: 0x00EE, .. } => Some(Instruction::Return),

        Opcode { id: 0x1, .. } => Some(Instruction::Jump(opcode)),
        Opcode { id: 0x2, .. } => Some(Instruction::Call(opcode)),
        Opcode { id: 0x3, .. } => Some(Instruction::SkipEqualByte(opcode)),
        Opcode { id: 0x4, .. } => Some(Instruction::SkipNotEqualByte(opcode)),
        Opcode { id: 0x5, .. } => Some(Instruction::SkipEqual(opcode)),
        Opcode { id: 0x6, .. } => Some(Instruction::LoadByte(opcode)),
        Opcode { id: 0x7, .. } => Some(Instruction::AddByte(opcode)),

        Opcode { id: 0x8, nibble: 0x0, .. } => Some(Instruction::Load(opcode)),
        Opcode { id: 0x8, nibble: 0x1, .. } => Some(Instruction::Or(opcode)),
        Opcode { id: 0x8, nibble: 0x2, .. } => Some(Instruction::And(opcode)),
        Opcode { id: 0x8, nibble: 0x3, .. } => Some(Instruction::Xor(opcode)),
        Opcode { id: 0x8, nibble: 0x4, .. } => Some(Instruction::Add(opcode)),
        Opcode { id: 0x8, nibble: 0x5, .. } => Some(Instruction::SubXY(opcode)),
        Opcode { id: 0x8, nibble: 0x6, .. } => Some(Instruction::ShiftRight(opcode)),
        Opcode { id: 0x8, nibble: 0x7, .. } => Some(Instruction::SubYX(opcode)),
        Opcode { id: 0x8, nibble: 0xE, .. } => Some(Instruction::ShiftLeft(opcode)),

        Opcode { id: 0x9, .. } => Some(Instruction::SkipNotEqual(opcode)),
        Opcode { id: 0xA, .. } => Some(Instruction::LoadI(opcode)),
        Opcode { id: 0xB, .. } => Some(Instruction::JumpPlus(opcode)),
        Opcode { id: 0xC, .. } => Some(Instruction::RandomMask(opcode)),
        Opcode { id: 0xD, .. } => Some(Instruction::Draw(opcode)),

        Opcode { id: 0xE, y: 0x9, nibble: 0xE, .. } => Some(Instruction::SkipOnKeyPressed(opcode)),
        Opcode { id: 0xE, y: 0xA, nibble: 0x1, .. } => Some(Instruction::SkipOnKeyNotPressed(opcode)),

        Opcode { id: 0xF, y: 0x0, nibble: 0x7, .. } => Some(Instruction::SaveDelayTimer(opcode)),
        Opcode { id: 0xF, y: 0x0, nibble: 0xA, .. } => Some(Instruction::WaitKey(opcode)),
        Opcode { id: 0xF, y: 0x1, nibble: 0x5, .. } => Some(Instruction::SetDelayTimer(opcode)),
        Opcode { id: 0xF, y: 0x1, nibble: 0x8, .. } => Some(Instruction::SetSoundTimer(opcode)),
        Opcode { id: 0xF, y: 0x1, nibble: 0xE, .. } => Some(Instruction::AddI(opcode)),
        Opcode { id: 0xF, y: 0x2, nibble: 0x9, .. } => Some(Instruction::LoadFont(opcode)),
        Opcode { id: 0xF, y: 0x3, nibble: 0x3, .. } => Some(Instruction::Bcd(opcode)),
        Opcode { id: 0xF, y: 0x5, nibble: 0x5, .. } => Some(Instruction::Write(opcode)),
        Opcode { id: 0xF, y: 0x6, nibble: 0x5, .. } => Some(Instruction::Read(opcode)),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_extracts_all_information_from_two_bytes() {
        let bytes = 0xF417;
        let opcode = Opcode::new(bytes);

        assert_eq!(bytes, opcode.bytes);
        assert_eq!(0x0417, opcode.address);
        assert_eq!(0x4, opcode.x);
        assert_eq!(0x1, opcode.y);
        assert_eq!(0x17, opcode.data);
        assert_eq!(0x7, opcode.nibble);
        assert_eq!(0xF, opcode.id);
    }

    #[test]
    fn it_decodes_clear() {
        let bytes: u16 = 0x00E0;
        let instruction = decode(bytes).unwrap();

        assert_eq!(Instruction::Clear, instruction);
    }

    #[test]
    fn it_decodes_return() {
        let bytes: u16 = 0x00EE;
        let instruction = decode(bytes).unwrap();

        assert_eq!(Instruction::Return, instruction);
    }

    #[test]
    fn it_decodes_jump() {
        let bytes: u16 = 0x1A1E;
        let instruction = decode(bytes).unwrap();

        let opcode  = Opcode::new(bytes);
        let expected = Instruction::Jump(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_call() {
        let bytes: u16 = 0x2A1E;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Call(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_equal_byte() {
        let bytes: u16 = 0x3122;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SkipEqualByte(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_not_equal_byte() {
        let bytes: u16 = 0x4122;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SkipNotEqualByte(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_equal() {
        let bytes: u16 = 0x51F0;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SkipEqual(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_byte_data() {
        let bytes: u16 = 0x61FA;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::LoadByte(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_add_byte() {
        let bytes: u16 = 0x71FA;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::AddByte(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_register_data() {
        let bytes: u16 = 0x81A0;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Load(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_or() {
        let bytes: u16 = 0x81A1;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Or(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_and() {
        let bytes: u16 = 0x81A2;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::And(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_xor() {
        let bytes: u16 = 0x81A3;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Xor(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_add() {
        let bytes: u16 = 0x81A4;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Add(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_sub_x_y() {
        let bytes: u16 = 0x81A5;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SubXY(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_shift_right() {
        let bytes: u16 = 0x81A6;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::ShiftRight(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_sub_y_x() {
        let bytes: u16 = 0x81A7;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SubYX(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_shift_left() {
        let bytes: u16 = 0x81AE;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::ShiftLeft(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_not_equal() {
        let bytes: u16 = 0x91A0;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SkipNotEqual(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_i() {
        let bytes: u16 = 0xA1AF;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::LoadI(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_jump_plus() {
        let bytes: u16 = 0xB1AF;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::JumpPlus(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_random_mask() {
        let bytes: u16 = 0xC1AF;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::RandomMask(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_draw() {
        let bytes: u16 = 0xD1AF;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Draw(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_on_key_pressed() {
        let bytes: u16 = 0xE29E;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SkipOnKeyPressed(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_on_key_not_pressed() {
        let bytes: u16 = 0xE2A1;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SkipOnKeyNotPressed(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_save_delay_timer() {
        let bytes: u16 = 0xF207;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SaveDelayTimer(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_wait_key() {
        let bytes: u16 = 0xF20A;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::WaitKey(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_set_delay_timer() {
        let bytes: u16 = 0xF215;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SetDelayTimer(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_set_sound_timer() {
        let bytes: u16 = 0xF218;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::SetSoundTimer(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_add_i() {
        let bytes: u16 = 0xF21E;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::AddI(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_font() {
        let bytes: u16 = 0xF229;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::LoadFont(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_bcd() {
        let bytes: u16 = 0xF233;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Bcd(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_write() {
        let bytes: u16 = 0xF255;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Write(opcode);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_read() {
        let bytes: u16 = 0xF265;
        let instruction = decode(bytes).unwrap();

        let opcode = Opcode::new(bytes);
        let expected = Instruction::Read(opcode);

        assert_eq!(expected, instruction);
    }
}
