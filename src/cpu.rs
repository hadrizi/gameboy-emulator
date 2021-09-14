use std::fmt;
use crate::core::{
    register::Register, 
    flags::Flags, 
    memory::Memory, 
    opcodes::InstrucionTarget, 
    opcodes::Instruction
};

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

    pub opcode_table: [(Instruction, i32); 0xFF],
}

// TODO: implement Deafault trait
impl CPU {
    // ################### PUBLIC ##########################
    
    pub fn new() -> CPU {
        let mut cpu = CPU{
            reg_af:         Register::new(0x0100),
            reg_bc:         Register::new(0xFF13),
            reg_de:         Register::new(0x00C1),
            reg_hl:         Register::new(0x8403),
            pc:             Register::new(0x0100),
            stack_pointer:  Register::new(0xFFFE),
            
            flags: Flags::new(),
            memory: Memory::new(),
            
            cycles: 0,
            opcode_table: [(Instruction::NOP, 0); 0xFF]
        };
        cpu.load_table();
        cpu
    }
    
    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let op = self.memory[self.pc];
            self.pc += 0x0001 as u16; // maybe just inc fn?
            self.cycles = 8; // testing 8 cycles; to be removed
        }
        
        self.cycles -= 1;
    }

    pub fn load_table(&mut self){
        /*
            # Legend:
            ## terms:
            opcode                  - 8bit code of instruction
            8bit register           - CPU 8bit register; any of A, B, C, D, E, H, L
                                      note that F is not used since it stores CPU flags
            16bit register          - CPU 8bit register; any of AF, BC, DE, HL, SP
            immediate 8bit value    - 8bit value right after opcode; e.g. we have memory layout
                                      .. 3E FF .., so opcode 0x3E is "LD A, d8" it means next byte 0xFF
                                      is our _immediate 8bit value_
            immediate 16bit value   - 16bit value right after opcode; e.g. we have memory layout
                                      .. 01 FF 13 .., so opcode 0x01 is "LD BC, d16" it means next 2 bytes 
                                      0xFF13 is our _immediate 16bit value_

            ## mnemonics:
            nn      - any value

            [nn]    - value at address nn
            [nn+]   - value at address nn; nn++
            [nn-]   - value at address nn; nn--
            
            r       - 8bit register
            rr      - 16bit register
            d8      - immediate 8bit value
            d16     - immediate 16bit value
            a16     - shortening for [d16]
            a8      - shortening for [FF00+d8]
            [C]     - shortening for [FF00+C]
        */

        // ############ load instructions ######################
        
        // mnemonic LD r, r
        self.opcode_table[0x78] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::B
        ), 4);
        self.opcode_table[0x79] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::C
        ), 4);
        self.opcode_table[0x7A] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::D
        ), 4);
        self.opcode_table[0x7B] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::E
        ), 4);
        self.opcode_table[0x7C] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::H
        ), 4);
        self.opcode_table[0x7D] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::L
        ), 4);
        self.opcode_table[0x7F] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::A
        ), 4);
        self.opcode_table[0x40] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::B
        ), 4);
        self.opcode_table[0x41] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::C
        ), 4);
        self.opcode_table[0x42] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::D
        ), 4);
        self.opcode_table[0x43] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::E
        ), 4);
        self.opcode_table[0x44] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::H
        ), 4);
        self.opcode_table[0x45] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::L
        ), 4);
        self.opcode_table[0x47] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::A
        ), 4);
        self.opcode_table[0x48] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::B
        ), 4);
        self.opcode_table[0x49] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::C
        ), 4);
        self.opcode_table[0x4A] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::D
        ), 4);
        self.opcode_table[0x4B] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::E
        ), 4);
        self.opcode_table[0x4C] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::H
        ), 4);
        self.opcode_table[0x4D] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::L
        ), 4);
        self.opcode_table[0x4F] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::A
        ), 4);
        self.opcode_table[0x50] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::B
        ), 4);
        self.opcode_table[0x51] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::C
        ), 4);
        self.opcode_table[0x52] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::D
        ), 4);
        self.opcode_table[0x53] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::E
        ), 4);
        self.opcode_table[0x54] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::H
        ), 4);
        self.opcode_table[0x55] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::L
        ), 4);
        self.opcode_table[0x57] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::A
        ), 4);
        self.opcode_table[0x58] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::B
        ), 4);
        self.opcode_table[0x59] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::C
        ), 4);
        self.opcode_table[0x5A] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::D
        ), 4);
        self.opcode_table[0x5B] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::E
        ), 4);
        self.opcode_table[0x5C] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::H
        ), 4);
        self.opcode_table[0x5D] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::L
        ), 4);
        self.opcode_table[0x5F] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::A
        ), 4);
        self.opcode_table[0x60] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::B
        ), 4);
        self.opcode_table[0x61] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::C
        ), 4);
        self.opcode_table[0x62] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::D
        ), 4);
        self.opcode_table[0x63] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::E
        ), 4);
        self.opcode_table[0x64] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::H
        ), 4);
        self.opcode_table[0x65] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::L
        ), 4);
        self.opcode_table[0x67] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::A
        ), 4);
        self.opcode_table[0x68] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::B
        ), 4);
        self.opcode_table[0x69] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::C
        ), 4);
        self.opcode_table[0x6A] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::D
        ), 4);
        self.opcode_table[0x6B] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::E
        ), 4);
        self.opcode_table[0x6C] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::H
        ), 4);
        self.opcode_table[0x6D] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::L
        ), 4);
        self.opcode_table[0x6F] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::A
        ), 4);
        
        // mnemonic LD r, d8
        self.opcode_table[0x3E] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::D8
        ), 8);
        self.opcode_table[0x06] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::D8
        ), 8);
        self.opcode_table[0x0E] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::D8
        ), 8);
        self.opcode_table[0x16] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::D8
        ), 8);
        self.opcode_table[0x1E] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::D8
        ), 8);
        self.opcode_table[0x26] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::D8
        ), 8);
        self.opcode_table[0x2E] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::D8
        ), 8);
        
        // mnemonic LD r, [HL]
        self.opcode_table[0x7E] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::HLMem
        ), 8);
        self.opcode_table[0x46] = (Instruction::LD(
            InstrucionTarget::B, 
            InstrucionTarget::HLMem
        ), 8);
        self.opcode_table[0x4E] = (Instruction::LD(
            InstrucionTarget::C, 
            InstrucionTarget::HLMem
        ), 8);
        self.opcode_table[0x56] = (Instruction::LD(
            InstrucionTarget::D, 
            InstrucionTarget::HLMem
        ), 8);
        self.opcode_table[0x5E] = (Instruction::LD(
            InstrucionTarget::E, 
            InstrucionTarget::HLMem
        ), 8);
        self.opcode_table[0x66] = (Instruction::LD(
            InstrucionTarget::H, 
            InstrucionTarget::HLMem
        ), 8);
        self.opcode_table[0x6E] = (Instruction::LD(
            InstrucionTarget::L, 
            InstrucionTarget::HLMem
        ), 8);
        
        // mnemonic LD [HL], r
        self.opcode_table[0x77] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::A
        ), 8);
        self.opcode_table[0x70] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::B
        ), 8);
        self.opcode_table[0x71] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::C
        ), 8);
        self.opcode_table[0x72] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::D
        ), 8);
        self.opcode_table[0x73] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::E
        ), 8);
        self.opcode_table[0x74] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::H
        ), 8);
        self.opcode_table[0x75] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::L
        ), 8);
        
        // mnemonic LD [HL], d8
        self.opcode_table[0x36] = (Instruction::LD(
            InstrucionTarget::HLMem, 
            InstrucionTarget::D8
        ), 12);
        
        // mnemonic LD A, [rr]; rr excludes AF, HL here
        self.opcode_table[0x0A] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::BCMem
        ), 8);
        self.opcode_table[0x1A] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::DEMem
        ), 8);
        
        // mnemonic LD A, a16
        self.opcode_table[0xFA] = (Instruction::LD(
            InstrucionTarget::A, 
            InstrucionTarget::A16
        ), 16);
        
        // mnemonic LD [rr], A; rr excludes AF, HL here
        self.opcode_table[0x02] = (Instruction::LD(
            InstrucionTarget::BCMem,
            InstrucionTarget::A
        ), 8);
        self.opcode_table[0x12] = (Instruction::LD(
            InstrucionTarget::DEMem,
            InstrucionTarget::A 
        ), 8);
        
        // mnemonic LD a16, A
        self.opcode_table[0xEA] = (Instruction::LD(
            InstrucionTarget::A16,
            InstrucionTarget::A
        ), 16);
        
        // mnemonic LD A, a8
        self.opcode_table[0xF0] = (Instruction::LD(
            InstrucionTarget::A,
            InstrucionTarget::A8
        ), 12);
        
        // mnemonic LD a8, A
        self.opcode_table[0xE0] = (Instruction::LD(
            InstrucionTarget::A8,
            InstrucionTarget::A
        ), 12);
        
        // mnemonic LD A, [C]
        self.opcode_table[0xF2] = (Instruction::LD(
            InstrucionTarget::A,
            InstrucionTarget::CMem
        ), 8);
        
        // mnemonic LD [C], A
        self.opcode_table[0xE2] = (Instruction::LD(
            InstrucionTarget::CMem,
            InstrucionTarget::A
        ), 8);
        
        // mnemonic LDI [HL+], A
        self.opcode_table[0x22] = (Instruction::LDI(
            InstrucionTarget::HLMem,
            InstrucionTarget::A
        ), 8);
        
        // mnemonic LDI A, [HL+]
        self.opcode_table[0x2A] = (Instruction::LDI(
            InstrucionTarget::A,
            InstrucionTarget::HLMem
        ), 8);
        
        // mnemonic LDD [HL-], A 
        self.opcode_table[0x32] = (Instruction::LDD(
            InstrucionTarget::HLMem,
            InstrucionTarget::A
        ), 8);

        // mnemonic LDD A, [HL-]
        self.opcode_table[0x3A] = (Instruction::LDD(
            InstrucionTarget::A,
            InstrucionTarget::HLMem
        ), 8);

        // #####################################################
        
    }

    // #####################################################
    
    // ################### PRIVATE #########################
    
    
    // #####################################################
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