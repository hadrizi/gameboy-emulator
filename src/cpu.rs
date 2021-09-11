use std::fmt;
use crate::core::{register::Register, flags::Flags, memory::Memory};

pub struct CPU {
    // ################### REGISTERS #######################

    // General purpose registers
    pub reg_af: Register,
    pub reg_bc: Register,
    pub reg_de: Register,
    pub reg_hl: Register,
    
    // Special registers
    pub stack_pointer: Register,
    pub pc: Register,
    // #####################################################

    // ################### FLAGS ###########################
    pub flags: Flags,
    // #####################################################

    // ################### MEMORY ##########################
    pub memory: Memory,
    // #####################################################
    
    // ################### UTILS ###########################
    cycles: i32,
    // #####################################################
}

// TODO: implement Deafault trait
impl CPU {
    pub fn new() -> CPU {
        CPU{
            reg_af:         Register::new(0x0100),
            reg_bc:         Register::new(0xFF13),
            reg_de:         Register::new(0x00C1),
            reg_hl:         Register::new(0x8403),
            pc:             Register::new(0x0100),
            stack_pointer:  Register::new(0xFFFE),

            flags: Flags::new(),
            memory: Memory::new(),

            cycles: 0,
        }
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let op = self.memory[self.pc];
            self.pc += 0x0001 as u16; // maybe just inc fn?
            self.cycles = 8; // testing 8 cycles; to be removed
        }

        self.cycles -= 1;
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
         .field("AF", &self.reg_af)
         .field("BC", &self.reg_bc)
         .field("DE", &self.reg_de)
         .field("HL", &self.reg_hl)
         .field("PC", &self.pc)
         .field("SP", &self.stack_pointer)
         .field("Flags", &self.flags)
        //  .field("name: &str", value: &dyn fmt::Debug)
         .finish()
    }
}