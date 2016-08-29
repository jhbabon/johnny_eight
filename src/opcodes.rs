#[derive(PartialEq,Debug)]
pub enum Opcode {
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SE(u8, u8),
    SNE(u8, u8),
}

pub fn decode(bytes: u16) -> Option<Opcode> {
    match bytes {
        0x00E0 => Some(Opcode::CLS),
        0x00EE => Some(Opcode::RET),
        // I don't know if is better to use a range or this bitwise check
        arg if (arg & 0xF000) == 0x1000 => Some(Opcode::JP(0x1000 ^ arg)),
        arg if (arg & 0xF000) == 0x2000 => Some(Opcode::CALL(0x2000 ^ arg)),
        arg if (arg & 0xF000) == 0x3000 => {
            let register = ((0x0F00 & arg) >> 8) as u8;
            let value = (0x00FF & arg) as u8;

            Some(Opcode::SE(register, value))
        },
        arg if (arg & 0xF000) == 0x4000 => {
            let register = ((0x0F00 & arg) >> 8) as u8;
            let value = (0x00FF & arg) as u8;

            Some(Opcode::SNE(register, value))
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_CLS() {
        let bytes: u16 = 0x00E0;
        let opcode = decode(bytes).unwrap();

        assert_eq!(Opcode::CLS, opcode);
    }

    #[test]
    fn it_decodes_RET() {
        let bytes: u16 = 0x00EE;
        let opcode = decode(bytes).unwrap();

        assert_eq!(Opcode::RET, opcode);
    }

    #[test]
    fn it_decodes_JP() {
        let bytes: u16 = 0x1A1E;
        let opcode = decode(bytes).unwrap();

        let expected = Opcode::JP(0x0A1E);

        assert_eq!(expected, opcode);
    }

    #[test]
    fn it_decodes_CALL() {
        let bytes: u16 = 0x2A1E;
        let opcode = decode(bytes).unwrap();

        let expected = Opcode::CALL(0x0A1E);

        assert_eq!(expected, opcode);
    }

    #[test]
    fn it_decodes_SE() {
        let bytes: u16 = 0x3122;
        let opcode = decode(bytes).unwrap();

        let expected = Opcode::SE(0x01, 0x22);

        assert_eq!(expected, opcode);
    }

    #[test]
    fn it_decodes_SNE() {
        let bytes: u16 = 0x4122;
        let opcode = decode(bytes).unwrap();

        let expected = Opcode::SNE(0x01, 0x22);

        assert_eq!(expected, opcode);
    }
}
