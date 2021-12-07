use core::fmt;

#[derive(Clone, Copy, Debug)]
pub enum InstrucionTarget {
    A, B, C, D, E, H, L, HLMem, CMem, U8, A8, I8,           // 8 bit
    AF, BC, DE, HL, SP, BCMem, DEMem, U16, A16, SPi8, PC,   // 16 bit
    ZCond, CCond, NZCond, NCCond,                           // JP conditions 
    Blank
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // LOAD
    LD(InstrucionTarget, InstrucionTarget),
    LDI(InstrucionTarget, InstrucionTarget),
    LDD(InstrucionTarget, InstrucionTarget),
    PUSH(InstrucionTarget),
    POP(InstrucionTarget),
    
    // ARITHMETIC
    ADD(InstrucionTarget, InstrucionTarget),
    ADC(InstrucionTarget, InstrucionTarget),
    SUB(InstrucionTarget, InstrucionTarget),
    SBC(InstrucionTarget, InstrucionTarget),
    INC(InstrucionTarget),
    DEC(InstrucionTarget),

    // LOGIC
    AND(InstrucionTarget, InstrucionTarget),
    XOR(InstrucionTarget, InstrucionTarget),
    OR(InstrucionTarget, InstrucionTarget),
    CP(InstrucionTarget, InstrucionTarget),
    DAA,
    CPL,

    // ROTATE and SHIFT
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC(InstrucionTarget),
    RL(InstrucionTarget),
    RRC(InstrucionTarget),
    RR(InstrucionTarget),
    SLA(InstrucionTarget),
    SWAP(InstrucionTarget),
    SRA(InstrucionTarget),
    SRL(InstrucionTarget),

    // SINGLE-BIT
    BIT(u8, InstrucionTarget),
    SET(u8, InstrucionTarget),
    RES(u8, InstrucionTarget),

    // CPU CONTROL
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,

    // JUMP
    JP(InstrucionTarget, InstrucionTarget),
    JR(InstrucionTarget, InstrucionTarget),
    CALL(InstrucionTarget, InstrucionTarget),
    RET(InstrucionTarget),
    RETI,
    RST(u16),

    PREFIX,
    
    // CUSTOM
    XXX,    // unknown
    TODO,   // to be done
}

/// TODO - simplify
/// I'm **sure** there is better way to do this
impl Instruction {
    pub fn to_string(&self) -> String {
        match self {
            Instruction::LD(op1, op2) =>    format!("LD {} {}", op1, op2),
            Instruction::LDI(op1, op2) =>   format!("LDI {} {}", op1, op2),
            Instruction::LDD(op1, op2) =>   format!("LDD {} {}", op1, op2),
            Instruction::PUSH(op) =>                        format!("PUSH {}", op),
            Instruction::POP(op) =>                         format!("POP {}", op),
            Instruction::ADD(op1, op2) =>   format!("ADD {} {}", op1, op2),
            Instruction::ADC(op1, op2) =>   format!("ADC {} {}", op1, op2),
            Instruction::SUB(op1, op2) =>   format!("SUB {} {}", op1, op2),
            Instruction::SBC(op1, op2) =>   format!("SBC {} {}", op1, op2),
            Instruction::INC(op) =>                         format!("INC {}", op),
            Instruction::DEC(op) =>                         format!("DEC {}", op),
            Instruction::AND(op1, op2) =>   format!("AND {} {}", op1, op2),
            Instruction::XOR(op1, op2) =>   format!("XOR {} {}", op1, op2),
            Instruction::OR(op1, op2) =>    format!("OR {} {}", op1, op2),
            Instruction::CP(op1, op2) =>    format!("CP {} {}", op1, op2),
            Instruction::DAA =>                                             format!("DAA"),
            Instruction::CPL =>                                             format!("CPL"),
            Instruction::RLCA =>                                            format!("RLCA"),
            Instruction::RLA =>                                             format!("RLA"),
            Instruction::RRCA =>                                            format!("RRCA"),
            Instruction::RRA =>                                             format!("RRA"),
            Instruction::RLC(op) =>                         format!("RLC {}", op),
            Instruction::RL(op) =>                          format!("RL {}", op),
            Instruction::RRC(op) =>                         format!("RRC {}", op),
            Instruction::RR(op) =>                          format!("RR {}", op),
            Instruction::SLA(op) =>                         format!("SLA {}", op),
            Instruction::SWAP(op) =>                        format!("SWAP {}", op),
            Instruction::SRA(op) =>                         format!("SRA {}", op),
            Instruction::SRL(op) =>                         format!("SRL {}", op),
            Instruction::BIT(op1, op2) =>               format!("BIT {} {}", op1, op2),
            Instruction::SET(op1, op2) =>               format!("SET {} {}", op1, op2),
            Instruction::RES(op1, op2) =>               format!("RES {} {}", op1, op2),
            Instruction::CCF =>                                             format!("CCF"),
            Instruction::SCF =>                                             format!("SCF"),
            Instruction::NOP =>                                             format!("NOP"),
            Instruction::HALT =>                                            format!("HALT"),
            Instruction::STOP =>                                            format!("STOP"),
            Instruction::DI =>                                              format!("DI"),
            Instruction::EI =>                                              format!("EI"),
            Instruction::RETI =>                                            format!("RETI"),
            Instruction::RST(op) =>                                    format!("RST 0x{:X}", op),
            Instruction::PREFIX =>                                          format!("RETI"),
            Instruction::XXX =>                                             format!("???"),
            Instruction::TODO =>                                            format!("TODO"),
            Instruction::JP(op1, op2) => {
                if let InstrucionTarget::Blank = op1 {
                    format!("JP {}", op2)
                } else {
                    format!("JP {} {}", op1, op2)
                }
            },
            Instruction::JR(op1, op2) => {
                if let InstrucionTarget::Blank = op1 {
                    format!("JR {}", op2)
                } else {
                    format!("JR {} {}", op1, op2)
                }
            },
            Instruction::CALL(op1, op2) => {
                if let InstrucionTarget::Blank = op1 {
                    format!("CALL {}", op2)
                } else {
                    format!("CALL {} {}", op1, op2)
                }
            },
            Instruction::RET(op) => {
                if let InstrucionTarget::Blank = op {
                    format!("RET")
                } else {
                    format!("RET {}", op)
                }
            },
        }
    }
}

impl fmt::Display for InstrucionTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstrucionTarget::A =>      write!(f, "A"),
            InstrucionTarget::B =>      write!(f, "B"),
            InstrucionTarget::C =>      write!(f, "C"),
            InstrucionTarget::D =>      write!(f, "D"),
            InstrucionTarget::E =>      write!(f, "E"),
            InstrucionTarget::H =>      write!(f, "H"),
            InstrucionTarget::L =>      write!(f, "L"),
            InstrucionTarget::HLMem =>  write!(f, "[HL]"),
            InstrucionTarget::CMem =>   write!(f, "[$FF00+C]"),
            InstrucionTarget::U8 =>     write!(f, "u8"),
            InstrucionTarget::A8 =>     write!(f, "[$FF00+u8]"),
            InstrucionTarget::AF =>     write!(f, "AF"),
            InstrucionTarget::BC =>     write!(f, "BC"),
            InstrucionTarget::DE =>     write!(f, "DE"),
            InstrucionTarget::HL =>     write!(f, "HL"),
            InstrucionTarget::SP =>     write!(f, "SP"),
            InstrucionTarget::BCMem =>  write!(f, "[BC]"),
            InstrucionTarget::DEMem =>  write!(f, "[DE]"),
            InstrucionTarget::U16 =>    write!(f, "u16"),
            InstrucionTarget::A16 =>    write!(f, "a16"),
            InstrucionTarget::I8 =>     write!(f, "i8"),
            InstrucionTarget::SPi8 =>   write!(f, "SP+i8"),
            InstrucionTarget::PC =>     write!(f, "pc"),
            InstrucionTarget::Blank =>  write!(f, "blank"),
            InstrucionTarget::ZCond =>  write!(f, "Z"),
            InstrucionTarget::CCond =>  write!(f, "C"),
            InstrucionTarget::NZCond => write!(f, "NZ"),
            InstrucionTarget::NCCond => write!(f, "NC"),
        }
    }
}