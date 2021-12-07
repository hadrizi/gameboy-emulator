use super::opcodes::{Instruction, InstrucionTarget};

pub const TABLE_SIZE: usize = 0xFF + 1;

pub fn build_table() -> [(Instruction, i32); TABLE_SIZE] {
    let mut opcode_table: [(Instruction, i32); TABLE_SIZE] = [(Instruction::TODO, 0); TABLE_SIZE];
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
        jump condition          - flag to be checked
        
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

        cond    - jump condition
    */

    // mnemonics NOP
    opcode_table[0x00] = (Instruction::NOP, 4);

    // ############ load instructions ######################

    // mnemonic LD r, r
    opcode_table[0x78] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x79] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x7A] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x7B] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x7C] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x7D] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x7F] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::A,
    ), 4);
    opcode_table[0x40] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::B
    ), 4);
    opcode_table[0x41] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x42] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x43] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x44] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x45] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x47] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::A,
    ), 4);
    opcode_table[0x48] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x49] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x4A] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x4B] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x4C] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x4D] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x4F] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::A,
    ), 4);
    opcode_table[0x50] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x51] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x52] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x53] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x54] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x55] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x57] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::A,
    ), 4);
    opcode_table[0x58] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x59] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x5A] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x5B] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x5C] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x5D] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x5F] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::A,
    ), 4);
    opcode_table[0x60] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x61] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x62] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x63] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x64] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x65] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x67] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::A,
    ), 4);
    opcode_table[0x68] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x69] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x6A] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x6B] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x6C] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x6D] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x6F] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::A,
    ), 4);
    
    // mnemonic LD r, d8
    opcode_table[0x3E] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::U8,
    ), 8);
    opcode_table[0x06] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::U8,
    ), 8);
    opcode_table[0x0E] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::U8,
    ), 8);
    opcode_table[0x16] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::U8,
    ), 8);
    opcode_table[0x1E] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::U8,
    ), 8);
    opcode_table[0x26] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::U8,
    ), 8);
    opcode_table[0x2E] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::U8,
    ), 8);
    
    // mnemonic LD r, [HL]
    opcode_table[0x7E] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::HLMem,
    ), 8);
    opcode_table[0x46] = (Instruction::LD(
        InstrucionTarget::B, 
        InstrucionTarget::HLMem,
    ), 8);
    opcode_table[0x4E] = (Instruction::LD(
        InstrucionTarget::C, 
        InstrucionTarget::HLMem,
    ), 8);
    opcode_table[0x56] = (Instruction::LD(
        InstrucionTarget::D, 
        InstrucionTarget::HLMem,
    ), 8);
    opcode_table[0x5E] = (Instruction::LD(
        InstrucionTarget::E, 
        InstrucionTarget::HLMem,
    ), 8);
    opcode_table[0x66] = (Instruction::LD(
        InstrucionTarget::H, 
        InstrucionTarget::HLMem,
    ), 8);
    opcode_table[0x6E] = (Instruction::LD(
        InstrucionTarget::L, 
        InstrucionTarget::HLMem,
    ), 8);
    
    // mnemonic LD [HL], r
    opcode_table[0x77] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::A,
    ), 8);
    opcode_table[0x70] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::B,
    ), 8);
    opcode_table[0x71] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::C,
    ), 8);
    opcode_table[0x72] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::D,
    ), 8);
    opcode_table[0x73] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::E,
    ), 8);
    opcode_table[0x74] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::H,
    ), 8);
    opcode_table[0x75] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::L,
    ), 8);
    
    // mnemonic LD [HL], d8
    opcode_table[0x36] = (Instruction::LD(
        InstrucionTarget::HLMem, 
        InstrucionTarget::U8,
    ), 12);
    
    // mnemonic LD A, [rr]; rr excludes AF, HL here
    opcode_table[0x0A] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::BCMem,
    ), 8);
    opcode_table[0x1A] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::DEMem,
    ), 8);
    
    // mnemonic LD A, a16
    opcode_table[0xFA] = (Instruction::LD(
        InstrucionTarget::A, 
        InstrucionTarget::A16,
    ), 16);
    
    // mnemonic LD [rr], A; rr excludes AF, HL here
    opcode_table[0x02] = (Instruction::LD(
        InstrucionTarget::BCMem,
        InstrucionTarget::A,
    ), 8);
    opcode_table[0x12] = (Instruction::LD(
        InstrucionTarget::DEMem,
        InstrucionTarget::A ,
    ), 8);
    
    // mnemonic LD a16, A
    opcode_table[0xEA] = (Instruction::LD(
        InstrucionTarget::A16,
        InstrucionTarget::A,
    ), 16);
    
    // mnemonic LD A, a8
    opcode_table[0xF0] = (Instruction::LD(
        InstrucionTarget::A,
        InstrucionTarget::A8,
    ), 12);
    
    // mnemonic LD a8, A
    opcode_table[0xE0] = (Instruction::LD(
        InstrucionTarget::A8,
        InstrucionTarget::A,
    ), 12);
    
    // mnemonic LD A, [C]
    opcode_table[0xF2] = (Instruction::LD(
        InstrucionTarget::A,
        InstrucionTarget::CMem,
    ), 8);
    
    // mnemonic LD [C], A
    opcode_table[0xE2] = (Instruction::LD(
        InstrucionTarget::CMem,
        InstrucionTarget::A,
    ), 8);
    
    // mnemonic LDI [HL+], A
    opcode_table[0x22] = (Instruction::LDI(
        InstrucionTarget::HLMem,
        InstrucionTarget::A,
    ), 8);
    
    // mnemonic LDI A, [HL+]
    opcode_table[0x2A] = (Instruction::LDI(
        InstrucionTarget::A,
        InstrucionTarget::HLMem,
    ), 8);
    
    // mnemonic LDD [HL-], A 
    opcode_table[0x32] = (Instruction::LDD(
        InstrucionTarget::HLMem,
        InstrucionTarget::A,
    ), 8);

    // mnemonic LDD A, [HL-]
    opcode_table[0x3A] = (Instruction::LDD(
        InstrucionTarget::A,
        InstrucionTarget::HLMem,
    ), 8);
    
    // mnemonic LDD rr, d16
    opcode_table[0x01] = (Instruction::LD(
        InstrucionTarget::BC,
        InstrucionTarget::U16,
    ), 12);
    opcode_table[0x11] = (Instruction::LD(
        InstrucionTarget::DE,
        InstrucionTarget::U16,
    ), 12);
    opcode_table[0x21] = (Instruction::LD(
        InstrucionTarget::HL,
        InstrucionTarget::U16,
    ), 12);
    opcode_table[0x31] = (Instruction::LD(
        InstrucionTarget::SP,
        InstrucionTarget::U16,
    ), 12);
    
    // mnemonics LD a16, SP
    opcode_table[0x08] = (Instruction::LD(
        InstrucionTarget::A16,
        InstrucionTarget::SP,
    ), 20);
    
    // mnemonics LD SP, HL
    opcode_table[0xF9] = (Instruction::LD(
        InstrucionTarget::SP,
        InstrucionTarget::HL,
    ), 8);

    // mnemonics LD HL, SP+i8
    opcode_table[0xF8] = (Instruction::LD(
        InstrucionTarget::HL,
        InstrucionTarget::SPi8,
    ), 8);

    // mnemonics PUSH rr
    opcode_table[0xC5] = (Instruction::PUSH(
        InstrucionTarget::BC,
    ), 16);
    opcode_table[0xD5] = (Instruction::PUSH(
        InstrucionTarget::DE,
    ), 16);
    opcode_table[0xE5] = (Instruction::PUSH(
        InstrucionTarget::HL,
    ), 16);
    opcode_table[0xF5] = (Instruction::PUSH(
        InstrucionTarget::AF,
    ), 16);

    // mnemonics POP rr
    opcode_table[0xC1] = (Instruction::POP(
        InstrucionTarget::BC,
    ), 12);
    opcode_table[0xD1] = (Instruction::POP(
        InstrucionTarget::DE,
    ), 12);
    opcode_table[0xE1] = (Instruction::POP(
        InstrucionTarget::HL,
    ), 12);
    opcode_table[0xF1] = (Instruction::POP(
        InstrucionTarget::AF,
    ), 12);

    // ############ arithmetic instructions ################
    
    // mnemonics ADD A, r
    opcode_table[0x80] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0x81] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0x82] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0x83] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0x84] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0x85] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0x87] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);
    
    // mnemonics ADD A, [HL] 
    opcode_table[0x86] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics ADD A, d8
    opcode_table[0xC6] = (Instruction::ADD(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);

    // mnemonics ADD rr, rr
    opcode_table[0x09] = (Instruction::ADD(
        InstrucionTarget::HL,
        InstrucionTarget::BC
    ), 8);
    opcode_table[0x19] = (Instruction::ADD(
        InstrucionTarget::HL,
        InstrucionTarget::DE
    ), 8);
    opcode_table[0x29] = (Instruction::ADD(
        InstrucionTarget::HL,
        InstrucionTarget::HL
    ), 8);
    opcode_table[0x39] = (Instruction::ADD(
        InstrucionTarget::HL,
        InstrucionTarget::SP
    ), 8);

    // mnemonics ADD SP, i8
    opcode_table[0xE8] = (Instruction::ADD(
        InstrucionTarget::SP,
        InstrucionTarget::I8
    ), 16);

    // mnemonics SUB A, r
    opcode_table[0x90] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0x91] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0x92] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0x93] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0x94] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0x95] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0x97] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);

    // mnemonics SUB A, [HL]
    opcode_table[0x96] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics SUB A, d8
    opcode_table[0xD6] = (Instruction::SUB(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);

    // mnemonics SBC A, r
    opcode_table[0x88] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0x89] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0x8A] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0x8B] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0x8C] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0x8D] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0x8F] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);

    // mnemonics ADC A, [HL]
    opcode_table[0x8E] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics ADC A, d8
    opcode_table[0xCE] = (Instruction::ADC(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);

    // mnemonics SBC A, r
    opcode_table[0x98] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0x99] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0x9A] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0x9B] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0x9C] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0x9D] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0x9F] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);

    // mnemonics SBC A, [HL]
    opcode_table[0x9E] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics SBC A, d8
    opcode_table[0xDE] = (Instruction::SBC(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);
    
    // mnemonics AND A, r
    opcode_table[0xA0] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0xA1] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0xA2] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0xA3] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0xA4] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0xA5] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0xA7] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);

    // mnemonics AND A, [HL]
    opcode_table[0xA6] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics AND A, d8
    opcode_table[0xE6] = (Instruction::AND(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);

    // mnemonics OR A, r
    opcode_table[0xB0] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0xB1] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0xB2] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0xB3] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0xB4] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0xB5] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0xB7] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);

    // mnemonics OR A, [HL]
    opcode_table[0xB6] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics OR A, d8
    opcode_table[0xF6] = (Instruction::OR(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);

    // mnemonics XOR A, r
    opcode_table[0xA8] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0xA9] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0xAA] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0xAB] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0xAC] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0xAD] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0xAF] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);

    // mnemonics XOR A, [HL]
    opcode_table[0xAE] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics XOR A, d8
    opcode_table[0xEE] = (Instruction::XOR(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);

    // mnemonics CP A, r
    opcode_table[0xB8] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::B
    ), 4);
    opcode_table[0xB9] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::C
    ), 4);
    opcode_table[0xBA] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::D
    ), 4);
    opcode_table[0xBB] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::E
    ), 4);
    opcode_table[0xBC] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::H
    ), 4);
    opcode_table[0xBD] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::L
    ), 4);
    opcode_table[0xBF] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::A
    ), 4);

    // mnemonics CP A, [HL]
    opcode_table[0xBE] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::HLMem
    ), 8);

    // mnemonics CP A, d8
    opcode_table[0xFE] = (Instruction::CP(
        InstrucionTarget::A,
        InstrucionTarget::U8
    ), 8);

    // mnemonics INC r
    opcode_table[0x04] = (Instruction::INC(
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x0C] = (Instruction::INC(
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x14] = (Instruction::INC(
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x1C] = (Instruction::INC(
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x24] = (Instruction::INC(
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x2C] = (Instruction::INC(
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x3C] = (Instruction::INC(
        InstrucionTarget::A,
    ), 4);

    // mnemonics INC [HL]
    opcode_table[0x34] = (Instruction::INC(
        InstrucionTarget::HLMem,
    ), 12);

    // mnemonics INC rr
    opcode_table[0x03] = (Instruction::INC(
        InstrucionTarget::BC,
    ), 8);
    opcode_table[0x13] = (Instruction::INC(
        InstrucionTarget::DE,
    ), 8);
    opcode_table[0x23] = (Instruction::INC(
        InstrucionTarget::HL,
    ), 8);
    opcode_table[0x33] = (Instruction::INC(
        InstrucionTarget::SP,
    ), 8);

    // mnemonics DEC r
    opcode_table[0x05] = (Instruction::DEC(
        InstrucionTarget::B,
    ), 4);
    opcode_table[0x0D] = (Instruction::DEC(
        InstrucionTarget::C,
    ), 4);
    opcode_table[0x15] = (Instruction::DEC(
        InstrucionTarget::D,
    ), 4);
    opcode_table[0x1D] = (Instruction::DEC(
        InstrucionTarget::E,
    ), 4);
    opcode_table[0x25] = (Instruction::DEC(
        InstrucionTarget::H,
    ), 4);
    opcode_table[0x2D] = (Instruction::DEC(
        InstrucionTarget::L,
    ), 4);
    opcode_table[0x3D] = (Instruction::DEC(
        InstrucionTarget::A,
    ), 4);

    // mnemonics DEC [HL]
    opcode_table[0x35] = (Instruction::DEC(
        InstrucionTarget::HLMem,
    ), 12);

    // mnemonics DEC rr
    opcode_table[0x0B] = (Instruction::DEC(
        InstrucionTarget::BC,
    ), 8);
    opcode_table[0x1B] = (Instruction::DEC(
        InstrucionTarget::DE,
    ), 8);
    opcode_table[0x2B] = (Instruction::DEC(
        InstrucionTarget::HL,
    ), 8);
    opcode_table[0x3B] = (Instruction::DEC(
        InstrucionTarget::SP,
    ), 8);

    // mnemonics CPL
    opcode_table[0x2F] = (Instruction::CPL, 4);
    
    // mnemonics DAA
    opcode_table[0x27] = (Instruction::DAA, 4);

    // mnemonics RLCA
    opcode_table[0x07] = (Instruction::RLCA, 4);
    
    // mnemonics RLA
    opcode_table[0x17] = (Instruction::RLA, 4);

    //mnemonics RRCA
    opcode_table[0x0F] = (Instruction::RRCA, 4);
    
    // mnemonics RRA
    opcode_table[0x1F] = (Instruction::RRA, 4);

    // mnemonics JP cond, u16
    opcode_table[0xC2] = (Instruction::JP(
        InstrucionTarget::NZCond,
        InstrucionTarget::U16
    ), 12);
    opcode_table[0xD2] = (Instruction::JP(
        InstrucionTarget::NCCond,
        InstrucionTarget::U16
    ), 12);
    opcode_table[0xCA] = (Instruction::JP(
        InstrucionTarget::ZCond,
        InstrucionTarget::U16
    ), 12);
    opcode_table[0xDA] = (Instruction::JP(
        InstrucionTarget::CCond,
        InstrucionTarget::U16
    ), 12);
    
    // mnemonics JP u16
    opcode_table[0xC3] = (Instruction::JP(
        InstrucionTarget::Blank,
        InstrucionTarget::U16
    ), 16);
    
    // mnemonics JP HL
    opcode_table[0xE9] = (Instruction::JP(
        InstrucionTarget::Blank,
        InstrucionTarget::HL
    ), 4);

    // mnemonics JR cond, i8
    opcode_table[0x20] = (Instruction::JR(
        InstrucionTarget::NZCond,
        InstrucionTarget::I8
    ), 8);
    opcode_table[0x30] = (Instruction::JR(
        InstrucionTarget::NCCond,
        InstrucionTarget::I8
    ), 8);
    opcode_table[0x28] = (Instruction::JR(
        InstrucionTarget::ZCond,
        InstrucionTarget::I8
    ), 8);
    opcode_table[0x38] = (Instruction::JR(
        InstrucionTarget::CCond,
        InstrucionTarget::I8
    ), 8);

    // mnemonics JR i8
    opcode_table[0x18] = (Instruction::JR(
        InstrucionTarget::Blank,
        InstrucionTarget::I8
    ), 12);

    // mnemonics CALL cond, u16
    opcode_table[0xC4] = (Instruction::CALL(
        InstrucionTarget::NZCond,
        InstrucionTarget::U16
    ), 12);
    opcode_table[0xD4] = (Instruction::CALL(
        InstrucionTarget::NCCond,
        InstrucionTarget::U16
    ), 12);
    opcode_table[0xCC] = (Instruction::CALL(
        InstrucionTarget::ZCond,
        InstrucionTarget::U16
    ), 12);
    opcode_table[0xDC] = (Instruction::CALL(
        InstrucionTarget::CCond,
        InstrucionTarget::U16
    ), 12);

    // mnemonics CALL u16
    opcode_table[0xCD] = (Instruction::CALL(
        InstrucionTarget::Blank,
        InstrucionTarget::U16
    ), 24);

    // mnemonics RET
    opcode_table[0xC9] = (Instruction::RET(
        InstrucionTarget::Blank
    ), 16);

    // mnemonics RET cond
    opcode_table[0xC0] = (Instruction::RET(
        InstrucionTarget::NZCond
    ), 8);
    opcode_table[0xD0] = (Instruction::RET(
        InstrucionTarget::NCCond
    ), 8);
    opcode_table[0xC8] = (Instruction::RET(
        InstrucionTarget::ZCond
    ), 8);
    opcode_table[0xD8] = (Instruction::RET(
        InstrucionTarget::CCond
    ), 8);

    // mnemonics RETI
    opcode_table[0xD9] = (Instruction::RETI, 16);

    // mnemonics RST
    opcode_table[0xC7] = (Instruction::RST(0x00), 16);
    opcode_table[0xD7] = (Instruction::RST(0x10), 16);
    opcode_table[0xE7] = (Instruction::RST(0x20), 16);
    opcode_table[0xF7] = (Instruction::RST(0x30), 16);
    opcode_table[0xCF] = (Instruction::RST(0x08), 16);
    opcode_table[0xDF] = (Instruction::RST(0x18), 16);
    opcode_table[0xEF] = (Instruction::RST(0x28), 16);
    opcode_table[0xFF] = (Instruction::RST(0x38), 16);

    // mnemonics DI
    opcode_table[0xF3] = (Instruction::DI, 4);

    // mnemonics EI
    opcode_table[0xFB] = (Instruction::EI, 4);

    // mnemonics SCF
    opcode_table[0x37] = (Instruction::SCF, 4);
    
    // mnemonics CCF
    opcode_table[0x3F] = (Instruction::CCF, 4);

    // mnemonics STOP
    opcode_table[0x10] = (Instruction::STOP, 4);
    
    // mnemonics STOP
    opcode_table[0x76] = (Instruction::HALT, 4);

    // mnemonics PREFIX
    opcode_table[0xCB] = (Instruction::PREFIX, 4);

    // unknown opcodes
    opcode_table[0xDD] = (Instruction::XXX, 0);
    opcode_table[0xED] = (Instruction::XXX, 0);
    opcode_table[0xFD] = (Instruction::XXX, 0);
    opcode_table[0xEC] = (Instruction::XXX, 0);
    opcode_table[0xFC] = (Instruction::XXX, 0);
    opcode_table[0xDB] = (Instruction::XXX, 0);
    opcode_table[0xEB] = (Instruction::XXX, 0);
    opcode_table[0xE4] = (Instruction::XXX, 0);
    opcode_table[0xF4] = (Instruction::XXX, 0);
    opcode_table[0xD3] = (Instruction::XXX, 0);
    opcode_table[0xE3] = (Instruction::XXX, 0);

    opcode_table
}

pub fn build_prefix() -> [(Instruction, i32); TABLE_SIZE] {
    let mut preifx_table: [(Instruction, i32); TABLE_SIZE] = [(Instruction::TODO, 0); TABLE_SIZE];

    // mnemonics RLC
    preifx_table[0x00] = (Instruction::RLC(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x01] = (Instruction::RLC(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x02] = (Instruction::RLC(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x03] = (Instruction::RLC(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x04] = (Instruction::RLC(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x05] = (Instruction::RLC(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x06] = (Instruction::RLC(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x07] = (Instruction::RLC(
        InstrucionTarget::A
    ), 8);
    
    // mnemonics RRC
    preifx_table[0x08] = (Instruction::RRC(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x09] = (Instruction::RRC(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x0A] = (Instruction::RRC(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x0B] = (Instruction::RRC(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x0C] = (Instruction::RRC(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x0D] = (Instruction::RRC(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x0E] = (Instruction::RRC(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x0F] = (Instruction::RRC(
        InstrucionTarget::A
    ), 8);
    
    // mnemonics RL
    preifx_table[0x10] = (Instruction::RL(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x11] = (Instruction::RL(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x12] = (Instruction::RL(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x13] = (Instruction::RL(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x14] = (Instruction::RL(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x15] = (Instruction::RL(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x16] = (Instruction::RL(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x17] = (Instruction::RL(
        InstrucionTarget::A
    ), 8);

    // mnemonics RR
    preifx_table[0x18] = (Instruction::RR(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x19] = (Instruction::RR(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x1A] = (Instruction::RR(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x1B] = (Instruction::RR(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x1C] = (Instruction::RR(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x1D] = (Instruction::RR(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x1E] = (Instruction::RR(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x1F] = (Instruction::RR(
        InstrucionTarget::A
    ), 8);

    // mnemonics SLA
    preifx_table[0x20] = (Instruction::SLA(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x21] = (Instruction::SLA(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x22] = (Instruction::SLA(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x23] = (Instruction::SLA(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x24] = (Instruction::SLA(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x25] = (Instruction::SLA(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x26] = (Instruction::SLA(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x27] = (Instruction::SLA(
        InstrucionTarget::A
    ), 8);

    // mnemonics SRA
    preifx_table[0x28] = (Instruction::SRA(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x29] = (Instruction::SRA(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x2A] = (Instruction::SRA(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x2B] = (Instruction::SRA(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x2C] = (Instruction::SRA(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x2D] = (Instruction::SRA(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x2E] = (Instruction::SRA(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x2F] = (Instruction::SRA(
        InstrucionTarget::A
    ), 8);

    // mnemonics SWAP
    preifx_table[0x30] = (Instruction::SWAP(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x31] = (Instruction::SWAP(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x32] = (Instruction::SWAP(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x33] = (Instruction::SWAP(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x34] = (Instruction::SWAP(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x35] = (Instruction::SWAP(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x36] = (Instruction::SWAP(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x37] = (Instruction::SWAP(
        InstrucionTarget::A
    ), 8);

    // mnemonics SRL
    preifx_table[0x38] = (Instruction::SRL(
        InstrucionTarget::B
    ), 8);
    preifx_table[0x39] = (Instruction::SRL(
        InstrucionTarget::C
    ), 8);
    preifx_table[0x3A] = (Instruction::SRL(
        InstrucionTarget::D
    ), 8);
    preifx_table[0x3B] = (Instruction::SRL(
        InstrucionTarget::E
    ), 8);
    preifx_table[0x3C] = (Instruction::SRL(
        InstrucionTarget::H
    ), 8);
    preifx_table[0x3D] = (Instruction::SRL(
        InstrucionTarget::L
    ), 8);
    preifx_table[0x3E] = (Instruction::SRL(
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x3F] = (Instruction::SRL(
        InstrucionTarget::A
    ), 8);

    // mnemonics BIT
    preifx_table[0x40] = (Instruction::BIT(
        0,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x41] = (Instruction::BIT(
        0,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x42] = (Instruction::BIT(
        0,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x43] = (Instruction::BIT(
        0,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x44] = (Instruction::BIT(
        0,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x45] = (Instruction::BIT(
        0,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x46] = (Instruction::BIT(
        0,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x47] = (Instruction::BIT(
        0,
        InstrucionTarget::A
    ), 8);
    preifx_table[0x48] = (Instruction::BIT(
        1,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x49] = (Instruction::BIT(
        1,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x4A] = (Instruction::BIT(
        1,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x4B] = (Instruction::BIT(
        1,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x4C] = (Instruction::BIT(
        1,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x4D] = (Instruction::BIT(
        1,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x4E] = (Instruction::BIT(
        1,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x4F] = (Instruction::BIT(
        1,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x50] = (Instruction::BIT(
        2,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x51] = (Instruction::BIT(
        2,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x52] = (Instruction::BIT(
        2,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x53] = (Instruction::BIT(
        2,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x54] = (Instruction::BIT(
        2,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x55] = (Instruction::BIT(
        2,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x56] = (Instruction::BIT(
        2,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x57] = (Instruction::BIT(
        2,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x58] = (Instruction::BIT(
        3,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x59] = (Instruction::BIT(
        3,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x5A] = (Instruction::BIT(
        3,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x5B] = (Instruction::BIT(
        3,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x5C] = (Instruction::BIT(
        3,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x5D] = (Instruction::BIT(
        3,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x5E] = (Instruction::BIT(
        3,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x5F] = (Instruction::BIT(
        3,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x60] = (Instruction::BIT(
        4,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x61] = (Instruction::BIT(
        4,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x62] = (Instruction::BIT(
        4,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x63] = (Instruction::BIT(
        4,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x64] = (Instruction::BIT(
        4,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x65] = (Instruction::BIT(
        4,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x66] = (Instruction::BIT(
        4,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x67] = (Instruction::BIT(
        4,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x68] = (Instruction::BIT(
        5,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x69] = (Instruction::BIT(
        5,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x6A] = (Instruction::BIT(
        5,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x6B] = (Instruction::BIT(
        5,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x6C] = (Instruction::BIT(
        5,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x6D] = (Instruction::BIT(
        5,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x6E] = (Instruction::BIT(
        5,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x6F] = (Instruction::BIT(
        5,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x70] = (Instruction::BIT(
        6,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x71] = (Instruction::BIT(
        6,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x72] = (Instruction::BIT(
        6,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x73] = (Instruction::BIT(
        6,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x74] = (Instruction::BIT(
        6,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x75] = (Instruction::BIT(
        6,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x76] = (Instruction::BIT(
        6,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x77] = (Instruction::BIT(
        6,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x78] = (Instruction::BIT(
        7,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x79] = (Instruction::BIT(
        7,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x7A] = (Instruction::BIT(
        7,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x7B] = (Instruction::BIT(
        7,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x7C] = (Instruction::BIT(
        7,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x7D] = (Instruction::BIT(
        7,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x7E] = (Instruction::BIT(
        7,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x7F] = (Instruction::BIT(
        7,
        InstrucionTarget::A
    ), 8);

    // mnemonics RES
    preifx_table[0x80] = (Instruction::RES(
        0,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x81] = (Instruction::RES(
        0,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x82] = (Instruction::RES(
        0,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x83] = (Instruction::RES(
        0,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x84] = (Instruction::RES(
        0,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x85] = (Instruction::RES(
        0,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x86] = (Instruction::RES(
        0,
        InstrucionTarget::HLMem
    ), 16);

    preifx_table[0x87] = (Instruction::RES(
        0,
        InstrucionTarget::A
    ), 8);
    preifx_table[0x88] = (Instruction::RES(
        1,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x89] = (Instruction::RES(
        1,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x8A] = (Instruction::RES(
        1,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x8B] = (Instruction::RES(
        1,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x8C] = (Instruction::RES(
        1,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x8D] = (Instruction::RES(
        1,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x8E] = (Instruction::RES(
        1,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x8F] = (Instruction::RES(
        1,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x90] = (Instruction::RES(
        2,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x91] = (Instruction::RES(
        2,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x92] = (Instruction::RES(
        2,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x93] = (Instruction::RES(
        2,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x94] = (Instruction::RES(
        2,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x95] = (Instruction::RES(
        2,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x96] = (Instruction::RES(
        2,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x97] = (Instruction::RES(
        2,
        InstrucionTarget::A
    ), 8);

    preifx_table[0x98] = (Instruction::RES(
        3,
        InstrucionTarget::B
    ), 8);
    preifx_table[0x99] = (Instruction::RES(
        3,
        InstrucionTarget::C
    ), 8);
    preifx_table[0x9A] = (Instruction::RES(
        3,
        InstrucionTarget::D
    ), 8);
    preifx_table[0x9B] = (Instruction::RES(
        3,
        InstrucionTarget::E
    ), 8);
    preifx_table[0x9C] = (Instruction::RES(
        3,
        InstrucionTarget::H
    ), 8);
    preifx_table[0x9D] = (Instruction::RES(
        3,
        InstrucionTarget::L
    ), 8);
    preifx_table[0x9E] = (Instruction::RES(
        3,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0x9F] = (Instruction::RES(
        3,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xA0] = (Instruction::RES(
        4,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xA1] = (Instruction::RES(
        4,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xA2] = (Instruction::RES(
        4,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xA3] = (Instruction::RES(
        4,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xA4] = (Instruction::RES(
        4,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xA5] = (Instruction::RES(
        4,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xA6] = (Instruction::RES(
        4,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xA7] = (Instruction::RES(
        4,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xA8] = (Instruction::RES(
        5,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xA9] = (Instruction::RES(
        5,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xAA] = (Instruction::RES(
        5,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xAB] = (Instruction::RES(
        5,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xAC] = (Instruction::RES(
        5,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xAD] = (Instruction::RES(
        5,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xAE] = (Instruction::RES(
        5,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xAF] = (Instruction::RES(
        5,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xB0] = (Instruction::RES(
        6,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xB1] = (Instruction::RES(
        6,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xB2] = (Instruction::RES(
        6,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xB3] = (Instruction::RES(
        6,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xB4] = (Instruction::RES(
        6,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xB5] = (Instruction::RES(
        6,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xB6] = (Instruction::RES(
        6,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xB7] = (Instruction::RES(
        6,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xB8] = (Instruction::RES(
        7,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xB9] = (Instruction::RES(
        7,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xBA] = (Instruction::RES(
        7,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xBB] = (Instruction::RES(
        7,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xBC] = (Instruction::RES(
        7,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xBD] = (Instruction::RES(
        7,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xBE] = (Instruction::RES(
        7,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xBF] = (Instruction::RES(
        7,
        InstrucionTarget::A
    ), 8);

    // mnemonics SET
    preifx_table[0xC0] = (Instruction::SET(
        0,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xC1] = (Instruction::SET(
        0,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xC2] = (Instruction::SET(
        0,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xC3] = (Instruction::SET(
        0,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xC4] = (Instruction::SET(
        0,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xC5] = (Instruction::SET(
        0,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xC6] = (Instruction::SET(
        0,
        InstrucionTarget::HLMem
    ), 16);

    preifx_table[0xC7] = (Instruction::SET(
        0,
        InstrucionTarget::A
    ), 8);
    preifx_table[0xC8] = (Instruction::SET(
        1,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xC9] = (Instruction::SET(
        1,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xCA] = (Instruction::SET(
        1,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xCB] = (Instruction::SET(
        1,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xCC] = (Instruction::SET(
        1,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xCD] = (Instruction::SET(
        1,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xCE] = (Instruction::SET(
        1,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xCF] = (Instruction::SET(
        1,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xD0] = (Instruction::SET(
        2,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xD1] = (Instruction::SET(
        2,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xD2] = (Instruction::SET(
        2,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xD3] = (Instruction::SET(
        2,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xD4] = (Instruction::SET(
        2,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xD5] = (Instruction::SET(
        2,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xD6] = (Instruction::SET(
        2,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xD7] = (Instruction::SET(
        2,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xD8] = (Instruction::SET(
        3,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xD9] = (Instruction::SET(
        3,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xDA] = (Instruction::SET(
        3,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xDB] = (Instruction::SET(
        3,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xDC] = (Instruction::SET(
        3,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xDD] = (Instruction::SET(
        3,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xDE] = (Instruction::SET(
        3,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xDF] = (Instruction::SET(
        3,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xE0] = (Instruction::SET(
        4,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xE1] = (Instruction::SET(
        4,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xE2] = (Instruction::SET(
        4,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xE3] = (Instruction::SET(
        4,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xE4] = (Instruction::SET(
        4,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xE5] = (Instruction::SET(
        4,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xE6] = (Instruction::SET(
        4,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xE7] = (Instruction::SET(
        4,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xE8] = (Instruction::SET(
        5,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xE9] = (Instruction::SET(
        5,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xEA] = (Instruction::SET(
        5,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xEB] = (Instruction::SET(
        5,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xEC] = (Instruction::SET(
        5,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xED] = (Instruction::SET(
        5,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xEE] = (Instruction::SET(
        5,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xEF] = (Instruction::SET(
        5,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xF0] = (Instruction::SET(
        6,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xF1] = (Instruction::SET(
        6,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xF2] = (Instruction::SET(
        6,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xF3] = (Instruction::SET(
        6,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xF4] = (Instruction::SET(
        6,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xF5] = (Instruction::SET(
        6,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xF6] = (Instruction::SET(
        6,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xF7] = (Instruction::SET(
        6,
        InstrucionTarget::A
    ), 8);

    preifx_table[0xF8] = (Instruction::SET(
        7,
        InstrucionTarget::B
    ), 8);
    preifx_table[0xF9] = (Instruction::SET(
        7,
        InstrucionTarget::C
    ), 8);
    preifx_table[0xFA] = (Instruction::SET(
        7,
        InstrucionTarget::D
    ), 8);
    preifx_table[0xFB] = (Instruction::SET(
        7,
        InstrucionTarget::E
    ), 8);
    preifx_table[0xFC] = (Instruction::SET(
        7,
        InstrucionTarget::H
    ), 8);
    preifx_table[0xFD] = (Instruction::SET(
        7,
        InstrucionTarget::L
    ), 8);
    preifx_table[0xFE] = (Instruction::SET(
        7,
        InstrucionTarget::HLMem
    ), 16);
    preifx_table[0xFF] = (Instruction::SET(
        7,
        InstrucionTarget::A
    ), 8);

    preifx_table
}