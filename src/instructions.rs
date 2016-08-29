const ADDRESS_MASK: u16 = 0x0FFF;
const VX_MASK: u16      = 0x0F00;
const VY_MASK: u16      = 0x00F0;
const DATA_MASK: u16    = 0x00FF;
const NIBBLE_MASK: u16  = 0x000F;
const ID_MASK: u16      = 0xF000;

#[derive(Debug,Copy,Clone)]
struct Opcode {
    byte: u16,
    address: u16,
    x: u8,
    y: u8,
    data: u8,
    nibble: u8,
    id: u16,
}

impl Opcode {
    pub fn new(byte: u16) -> Opcode {
        Opcode {
            byte:    byte,
            address: ADDRESS_MASK & byte,
            x:       ((VX_MASK & byte) >> 8) as u8,
            y:       ((VY_MASK & byte) >> 4) as u8,
            data:    (DATA_MASK & byte) as u8,
            nibble:  (NIBBLE_MASK & byte) as u8,
            id:      ID_MASK & byte,
        }
    }
}

#[derive(PartialEq,Debug)]
pub enum Instruction {
    Clear,
    Return,
    Jump { address: u16 },
    Call { address: u16 },
    SkipEqualByte { vx: u8, byte: u8 },
    SkipNotEqualByte { vx: u8, byte: u8 },
    SkipEqual { vx: u8, vy: u8 },
    LoadByte { vx: u8, byte: u8 },
    AddByte { vx: u8, byte: u8 },
    Load { vx: u8, vy: u8 },
    Or { vx: u8, vy: u8 },
    And { vx: u8, vy: u8 },
    Xor { vx: u8, vy: u8 },
    Add { vx: u8, vy: u8 },
    SubXY { vx: u8, vy: u8 },
    ShiftRight { vx: u8, vy: u8 },
    SubYX { vx: u8, vy: u8 },
    ShiftLeft { vx: u8, vy: u8 },
    SkipNotEqual { vx: u8, vy: u8 },
    LoadI { address: u16 },
    JumpPlus { address: u16 },
    RandomMask { vx: u8, byte: u8 },
    Draw { vx: u8, vy: u8, nibble: u8 },
    SkipOnKeyPressed { vx: u8 },
    SkipOnKeyNotPressed { vx: u8 },
    SaveDelayTimer { vx: u8 },
    WaitKey { vx: u8 },
    SetDelayTimer { vx: u8 },
    SetSoundTimer { vx: u8 },
    AddI { vx: u8 },
    LoadFont { vx: u8 },
    Bcd { vx: u8 },
    Write { vx: u8 },
    Read { vx: u8 },
}

pub fn decode(byte: u16) -> Option<Instruction> {
    let opcode = Opcode::new(byte);

    match opcode {
        Opcode { byte: 0x00E0, .. } => Some(Instruction::Clear),
        Opcode { byte: 0x00EE, .. } => Some(Instruction::Return),

        Opcode { id: 0x1000, address, .. } => Some(Instruction::Jump { address: address }),
        Opcode { id: 0x2000, address, .. } => Some(Instruction::Call { address: address }),
        Opcode { id: 0x3000, x, data, .. } => Some(Instruction::SkipEqualByte { vx: x, byte: data }),
        Opcode { id: 0x4000, x, data, .. } => Some(Instruction::SkipNotEqualByte { vx: x, byte: data }),
        Opcode { id: 0x5000, x, y, .. }    => Some(Instruction::SkipEqual { vx: x, vy: y }),
        Opcode { id: 0x6000, x, data, .. } => Some(Instruction::LoadByte { vx: x, byte: data }),
        Opcode { id: 0x7000, x, data, .. } => Some(Instruction::AddByte { vx: x, byte: data }),

        Opcode { id: 0x8000, nibble: 0x00, x, y, .. } => Some(Instruction::Load { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x01, x, y, .. } => Some(Instruction::Or { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x02, x, y, .. } => Some(Instruction::And { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x03, x, y, .. } => Some(Instruction::Xor { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x04, x, y, .. } => Some(Instruction::Add { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x05, x, y, .. } => Some(Instruction::SubXY { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x06, x, y, .. } => Some(Instruction::ShiftRight { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x07, x, y, .. } => Some(Instruction::SubYX { vx: x, vy: y }),
        Opcode { id: 0x8000, nibble: 0x0E, x, y, .. } => Some(Instruction::ShiftLeft { vx: x, vy: y }),

        Opcode { id: 0x9000, x, y, .. }         => Some(Instruction::SkipNotEqual { vx: x, vy: y }),
        Opcode { id: 0xA000, address, .. }      => Some(Instruction::LoadI { address: address }),
        Opcode { id: 0xB000, address, .. }      => Some(Instruction::JumpPlus { address: address }),
        Opcode { id: 0xC000, x, data, .. }      => Some(Instruction::RandomMask { vx: x, byte: data }),
        Opcode { id: 0xD000, x, y, nibble, .. } => Some(Instruction::Draw { vx: x, vy: y, nibble: nibble }),

        Opcode { id: 0xE000, y: 0x09, nibble: 0x0E, x, .. } => Some(Instruction::SkipOnKeyPressed { vx: x }),
        Opcode { id: 0xE000, y: 0x0A, nibble: 0x01, x, .. } => Some(Instruction::SkipOnKeyNotPressed { vx: x }),

        Opcode { id: 0xF000, y: 0x00, nibble: 0x07, x, .. } => Some(Instruction::SaveDelayTimer { vx: x }),
        Opcode { id: 0xF000, y: 0x00, nibble: 0x0A, x, .. } => Some(Instruction::WaitKey { vx: x }),
        Opcode { id: 0xF000, y: 0x01, nibble: 0x05, x, .. } => Some(Instruction::SetDelayTimer { vx: x }),
        Opcode { id: 0xF000, y: 0x01, nibble: 0x08, x, .. } => Some(Instruction::SetSoundTimer { vx: x }),
        Opcode { id: 0xF000, y: 0x01, nibble: 0x0E, x, .. } => Some(Instruction::AddI { vx: x }),
        Opcode { id: 0xF000, y: 0x02, nibble: 0x09, x, .. } => Some(Instruction::LoadFont { vx: x }),
        Opcode { id: 0xF000, y: 0x03, nibble: 0x03, x, .. } => Some(Instruction::Bcd { vx: x }),
        Opcode { id: 0xF000, y: 0x05, nibble: 0x05, x, .. } => Some(Instruction::Write { vx: x }),
        Opcode { id: 0xF000, y: 0x06, nibble: 0x05, x, .. } => Some(Instruction::Read { vx: x }),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_clear() {
        let opcode: u16 = 0x00E0;
        let instruction = decode(opcode).unwrap();

        assert_eq!(Instruction::Clear, instruction);
    }

    #[test]
    fn it_decodes_return() {
        let opcode: u16 = 0x00EE;
        let instruction = decode(opcode).unwrap();

        assert_eq!(Instruction::Return, instruction);
    }

    #[test]
    fn it_decodes_jump() {
        let opcode: u16 = 0x1A1E;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Jump { address: 0x0A1E };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_call() {
        let opcode: u16 = 0x2A1E;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Call { address: 0x0A1E };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_equal_byte() {
        let opcode: u16 = 0x3122;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SkipEqualByte { vx: 0x01, byte: 0x22 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_not_equal_byte() {
        let opcode: u16 = 0x4122;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SkipNotEqualByte { vx: 0x01, byte: 0x22 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_equal() {
        let opcode: u16 = 0x51F0;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SkipEqual { vx: 0x01, vy: 0x0F };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_byte_data() {
        let opcode: u16 = 0x61FA;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::LoadByte { vx: 0x01, byte: 0xFA };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_add_byte() {
        let opcode: u16 = 0x71FA;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::AddByte { vx: 0x01, byte: 0xFA };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_register_data() {
        let opcode: u16 = 0x81A0;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Load { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_or() {
        let opcode: u16 = 0x81A1;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Or { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_and() {
        let opcode: u16 = 0x81A2;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::And { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_xor() {
        let opcode: u16 = 0x81A3;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Xor { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_add() {
        let opcode: u16 = 0x81A4;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Add { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_sub_x_y() {
        let opcode: u16 = 0x81A5;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SubXY { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_shift_right() {
        let opcode: u16 = 0x81A6;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::ShiftRight { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_sub_y_x() {
        let opcode: u16 = 0x81A7;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SubYX { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_shift_left() {
        let opcode: u16 = 0x81AE;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::ShiftLeft { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_not_equal() {
        let opcode: u16 = 0x91A0;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SkipNotEqual { vx: 0x01, vy: 0x0A };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_i() {
        let opcode: u16 = 0xA1AF;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::LoadI { address: 0x01AF };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_jump_plus() {
        let opcode: u16 = 0xB1AF;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::JumpPlus { address: 0x01AF };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_random_mask() {
        let opcode: u16 = 0xC1AF;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::RandomMask { vx: 0x01, byte: 0xAF };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_draw() {
        let opcode: u16 = 0xD1AF;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Draw { vx: 0x01, vy: 0x0A, nibble: 0x0F };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_on_key_pressed() {
        let opcode: u16 = 0xE29E;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SkipOnKeyPressed { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_skip_on_key_not_pressed() {
        let opcode: u16 = 0xE2A1;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SkipOnKeyNotPressed { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_save_delay_timer() {
        let opcode: u16 = 0xF207;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SaveDelayTimer { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_wait_key() {
        let opcode: u16 = 0xF20A;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::WaitKey { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_set_delay_timer() {
        let opcode: u16 = 0xF215;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SetDelayTimer { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_set_sound_timer() {
        let opcode: u16 = 0xF218;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SetSoundTimer { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_add_i() {
        let opcode: u16 = 0xF21E;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::AddI { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_load_font() {
        let opcode: u16 = 0xF229;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::LoadFont { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_bcd() {
        let opcode: u16 = 0xF233;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Bcd { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_write() {
        let opcode: u16 = 0xF255;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Write { vx: 0x02 };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_read() {
        let opcode: u16 = 0xF265;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Read { vx: 0x02 };

        assert_eq!(expected, instruction);
    }
}
