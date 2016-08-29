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
}
