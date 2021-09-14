#[derive(Clone, Copy)]
pub enum InstrucionTarget {
    A, B, C, D, E, H, L, HLMem, CMem, D8, A8,   // 8 bit
    BC, DE, HL, SP, BCMem, DEMem, D16, A16                    // 16 bit
}

#[derive(Clone, Copy)]
pub enum Instruction {
    // LOAD
    LD(InstrucionTarget, InstrucionTarget),
    LDI(InstrucionTarget, InstrucionTarget),
    LDD(InstrucionTarget, InstrucionTarget),
    PUSH(InstrucionTarget),
    POP(InstrucionTarget),
    
    // ARITHMETIC
    ADD,
    ADC,
    SUB,
    SBC,

    // LOGIC
    AND,
    XOR,
    OR,
    CP,
    INC,
    DEC,
    DAA,
    CPL,

    // ROTATE and SHIFT
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC,
    RL,
    RRC,
    RR,
    SLA,
    SWAP,
    SRA,
    SRL,

    // SINGLE-BIT
    BIT,
    SET,
    RES,

    // CPU CONTROL
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,

    // JUMP
    JP,
    JR,
    CALL,
    RET,
    RETI,
    RST
}