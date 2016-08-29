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
    Jump(u16),
    Call(u16),
    SkipEqualByte { vx: u8, byte: u8 },
    SkipNotEqualByte { vx: u8, byte: u8 },
    SkipEqual { vx: u8, vy: u8 },
    LoadByteData { vx: u8, byte: u8 },
    AddByte { vx: u8, byte: u8 },
}

pub fn decode(byte: u16) -> Option<Instruction> {
    let opcode = Opcode::new(byte);

    match opcode {
        Opcode { byte: 0x00E0, .. }        => Some(Instruction::Clear),
        Opcode { byte: 0x00EE, .. }        => Some(Instruction::Return),
        Opcode { id: 0x1000, address, .. } => Some(Instruction::Jump(address)),
        Opcode { id: 0x2000, address, .. } => Some(Instruction::Call(address)),
        Opcode { id: 0x3000, x, data, .. } => Some(Instruction::SkipEqualByte { vx: x, byte: data }),
        Opcode { id: 0x4000, x, data, .. } => Some(Instruction::SkipNotEqualByte { vx: x, byte: data }),
        Opcode { id: 0x5000, x, y, .. }    => Some(Instruction::SkipEqual { vx: x, vy: y }),
        Opcode { id: 0x6000, x, data, .. } => Some(Instruction::LoadByteData { vx: x, byte: data }),
        Opcode { id: 0x7000, x, data, .. } => Some(Instruction::AddByte { vx: x, byte: data }),
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

        let expected = Instruction::Jump(0x0A1E);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_call() {
        let opcode: u16 = 0x2A1E;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::Call(0x0A1E);

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

        let expected = Instruction::LoadByteData { vx: 0x01, byte: 0xFA };

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_add_byte() {
        let opcode: u16 = 0x71FA;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::AddByte { vx: 0x01, byte: 0xFA };

        assert_eq!(expected, instruction);
    }
}
