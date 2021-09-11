use crate::core::{register::Register, flags::Flags};

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
}