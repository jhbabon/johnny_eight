#[derive(PartialEq,Debug)]
pub enum Instruction {
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SE(u8, u8),
    SNE(u8, u8),
}

pub fn decode(opcode: u16) -> Option<Instruction> {
    match opcode {
        0x00E0 => Some(Instruction::CLS),
        0x00EE => Some(Instruction::RET),
        // I don't know if is better to use a range or this bitwise check
        arg if (arg & 0xF000) == 0x1000 => Some(Instruction::JP(0x1000 ^ arg)),
        arg if (arg & 0xF000) == 0x2000 => Some(Instruction::CALL(0x2000 ^ arg)),
        arg if (arg & 0xF000) == 0x3000 => {
            let register = ((0x0F00 & arg) >> 8) as u8;
            let value = (0x00FF & arg) as u8;

            Some(Instruction::SE(register, value))
        },
        arg if (arg & 0xF000) == 0x4000 => {
            let register = ((0x0F00 & arg) >> 8) as u8;
            let value = (0x00FF & arg) as u8;

            Some(Instruction::SNE(register, value))
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_CLS() {
        let opcode: u16 = 0x00E0;
        let instruction = decode(opcode).unwrap();

        assert_eq!(Instruction::CLS, instruction);
    }

    #[test]
    fn it_decodes_RET() {
        let opcode: u16 = 0x00EE;
        let instruction = decode(opcode).unwrap();

        assert_eq!(Instruction::RET, instruction);
    }

    #[test]
    fn it_decodes_JP() {
        let opcode: u16 = 0x1A1E;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::JP(0x0A1E);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_CALL() {
        let opcode: u16 = 0x2A1E;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::CALL(0x0A1E);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_SE() {
        let opcode: u16 = 0x3122;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SE(0x01, 0x22);

        assert_eq!(expected, instruction);
    }

    #[test]
    fn it_decodes_SNE() {
        let opcode: u16 = 0x4122;
        let instruction = decode(opcode).unwrap();

        let expected = Instruction::SNE(0x01, 0x22);

        assert_eq!(expected, instruction);
    }
}
