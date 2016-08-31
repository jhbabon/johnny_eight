// Trying to emulate a chip-8 computer!

const RAM_SIZE: usize = 4096;
const GENERAL_REGISTERS_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

pub struct VM {
    ram: [u8; RAM_SIZE],                     // Memory
    registers: [u8; GENERAL_REGISTERS_SIZE], // V0 - VF registers
    stack: [u16; STACK_SIZE],                // Stack for return addresses of subroutines

    i: u16,                                  // Store memory addresses

    dt: u8,                                  // Delay Timer register
    st: u8,                                  // Sound Timer register

    pc: u16,                                 // Program Counter
    sp: u8,                                  // Stack Pointer
}

impl Default for VM {
    fn default() -> VM {
        VM {
            ram: [0; RAM_SIZE],
            registers: [0; GENERAL_REGISTERS_SIZE],
            stack: [0; STACK_SIZE],
            i: 0,
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

    #[test]
    fn vm_has_4k_of_memory() {
        let vm: VM = Default::default();

        assert_eq!(4096, vm.ram.len());
    }

    #[test]
    fn vm_has_16_general_purpose_registers() {
        let vm: VM = Default::default();

        assert_eq!(16, vm.registers.len());
    }

    #[test]
    fn vm_has_the_i_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u16, vm.i);
    }

    #[test]
    fn vm_has_the_dt_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u8, vm.dt);
    }

    #[test]
    fn vm_has_the_st_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u8, vm.st);
    }

    #[test]
    fn vm_has_the_pc_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u16, vm.pc);
    }

    #[test]
    fn vm_has_the_sp_register() {
        let vm: VM = Default::default();

        assert_eq!(0 as u8, vm.sp);
    }

    #[test]
    fn vm_has_a_stack() {
        let vm: VM = Default::default();

        assert_eq!([0 as u16; 16], vm.stack);
    }
}
