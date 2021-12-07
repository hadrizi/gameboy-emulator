use std::fmt;
use indexmap::IndexMap;

use crate::core::{
    register::Register, 
    memory::Memory, 
    opcodes::InstrucionTarget, 
    opcodes::Instruction, 
    table_builder::{build_table, build_prefix, TABLE_SIZE},
};

pub struct CPU {
    // General purpose registers
    pub reg_af: Register,
    pub reg_bc: Register,
    pub reg_de: Register,
    pub reg_hl: Register,
    
    // Special registers
    pub stack_pointer: Register,
    pub pc: Register,
    pub ime: bool,

    // Halt and Stop registers
    pub halted: bool,
    pub stopped: bool,

    pub memory: Memory,
    
    pub cycles: i32,

    pub opcode_table: [(Instruction, i32); TABLE_SIZE],
    pub prefix_table: [(Instruction, i32); TABLE_SIZE],
}

pub enum Flag {
    Z, N ,C, H
}

// TODO: implement Deafault trait
impl CPU {    
    pub fn new() -> CPU {
        let cpu = CPU{
            reg_af:         Register::new(0x01B0),
            reg_bc:         Register::new(0x0013),
            reg_de:         Register::new(0x00D8),
            reg_hl:         Register::new(0x014D),
            pc:             Register::new(0x0100),
            stack_pointer:  Register::new(0xFFFE),

            ime: true,
            
            memory: Memory::new(),
            
            cycles: 0,
            opcode_table: build_table(),
            prefix_table: build_prefix(),

            halted: false,
            stopped: false
        };
        cpu
    }
    
    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let op = self.memory[self.pc];
            self.pc.inc();

            let instruction = self.opcode_table[op as usize];
            self.execute(instruction.0);
            self.cycles = instruction.1;
        }
        
        self.cycles -= 1;
    }

    pub fn disassemble(&self) -> IndexMap<u16, String> {
        let mut disas: IndexMap<u16, String> = IndexMap::new();

        let mut i: usize = 0;
        while i < 0xFFFF {
            let op = self.memory[i];
            let current = i;
            let instruction = self.opcode_table[op as usize];
            let mut inst = instruction.0.to_string();
            if i < 0x0100 { 
                disas.insert(i as u16, format!("${:04X} {}", i,String::from("???")));
                i += 1;
                continue;
            };
            
            if inst.contains("u8") {
                i += 1;
                let v = self.memory[i];
                inst = inst.replace("u8", format!("${:02X}", v).as_str()); 
            };
            if inst.contains("i8") {
                i += 1;
                let v = self.memory[i];
                inst = inst.replace("i8", format!("${:02X}", v).as_str()); 
            };
            if inst.contains("u16") || inst.contains("a16") {
                i += 1;
                let mut v: u16 = self.memory[i] as u16;
                i += 1;
                v |= (self.memory[i] as u16) << 8;
                if inst.contains(" u16") {
                    inst = inst.replace("u16", format!("${:04X}", v).as_str()); 
                } else {
                    inst = inst.replace("a16", format!("[${:04X}]", v).as_str()); 
                }
            };
            disas.insert(current as u16, format!("${:04X} {}", current, inst));
            i += 1;
        }
        disas
    } 

    /// ## PREFIX wrapper
    /// Routes prefixed instuctions
    fn prefix(&mut self) {
        let op = self.get_imm_u8();

        let instruction = self.prefix_table[op as usize];
        self.cycles += instruction.1;
        self.execute(instruction.0)
    }

    pub fn check_table(&self) {
        let mut c = 0;
        
        println!("Opcode table");
        for (index, item) in self.opcode_table.iter().enumerate() {
            match item.0 {
                Instruction::TODO => { println!("{:X}: not done", index); c += 1; },
                _ => continue
            }
        }
        if c == 0 { println!("Opcode table is finished!"); c = 0; }

        println!("\nPrefix table");
        for (index, item) in self.prefix_table.iter().enumerate() {
            match item.0 {
                Instruction::TODO => { println!("{:X}: not done", index); c += 1 },
                _ => continue
            }
        }
        if c == 0 { println!("Prefix table is finished!"); c = 0; }
    }

    fn _push(&mut self, value: Register) {
        self.memory[self.stack_pointer] = value.hi();
        self.stack_pointer.dec();
        self.memory[self.stack_pointer] = value.lo();
        self.stack_pointer.dec();
    }

    fn _pop(&mut self) -> Register {
        let mut v = Register::new(0);
        self.stack_pointer.inc();
        v.write_lo(self.memory[self.stack_pointer]);
        self.stack_pointer.inc();
        v.write_hi(self.memory[self.stack_pointer]);
        v
    }
    
    /// Returns u8 value representing Flag register in accordance with the provided conditions
    /// ### Example
    /// ```
    /// self.reg_af.write_lo(self.build_flag(
    ///     value == 0,
    ///     false, 
    ///     ((target & 0xF) + (source & 0xF)) > 0xF, 
    ///     (target + source) > 0xFF
    /// ));
    /// ```
    fn build_flag(&self, z: bool, n: bool, h: bool, c: bool) -> u8 {
        let mut flag: u8 = 0;
        if z {
            flag |= 0x80;
        }
        if n {
            flag |= 0x40;
        }
        if h {
            flag |= 0x20;
        }
        if c {
            flag |= 0x10;
        }
        flag
    }
    
    /// Returns true if specified flag is set(corresponding bit is `1`), false if unset
    pub fn get_flag(&self, target: Flag) -> bool {
        let flag = self.reg_af.lo();
        let v = match target {
            Flag::Z => flag & 0x80,
            Flag::N => flag & 0x40,
            Flag::C => flag & 0x20,
            Flag::H => flag & 0x10,
        };
        if v == 0 {
            return false;
        }
        true
    }

    /// Sets specified flag
    fn set_flag(&mut self, target: Flag) {
        let f = self.reg_af.lo();
        match target {
            Flag::Z => self.reg_af.write_lo(f | 0x80),
            Flag::N => self.reg_af.write_lo(f | 0x40),
            Flag::C => self.reg_af.write_lo(f | 0x20),
            Flag::H => self.reg_af.write_lo(f | 0x10),
        }
    }

    /// Returns immediate unsigned 8 bit value
    fn get_imm_u8(&mut self) -> u8 {
        let v = self.memory[self.pc];
        self.pc.inc();
        return v
    }

    /// Returns immediate signed 8 bit value
    fn get_imm_i8(&mut self) -> i8 {
        let v = self.memory[self.pc];
        self.pc.inc();
        return v as i8
    }
    
    /// Returns immediate 16 bit value
    fn get_imm_16(&mut self) -> u16 {
        // let v: u16 = (self.memory[self.pc] << 8) as u16;
        let mut v: u16 = self.memory[self.pc] as u16;
        self.pc.inc();
        v |= (self.memory[self.pc] as u16) << 8;
        self.pc.inc();
        return v
    }
    
    /// Writes `value` to specified source `target`.
    fn write_target(&mut self, target: InstrucionTarget, value: u16) {
        match target {
            InstrucionTarget::A => self.reg_af.write_hi(value as u8),
            InstrucionTarget::B => self.reg_bc.write_hi(value as u8),
            InstrucionTarget::C => self.reg_bc.write_lo(value as u8),
            InstrucionTarget::D => self.reg_de.write_hi(value as u8),
            InstrucionTarget::E => self.reg_de.write_lo(value as u8),
            InstrucionTarget::H => self.reg_hl.write_hi(value as u8),
            InstrucionTarget::L => self.reg_hl.write_lo(value as u8),
            InstrucionTarget::HLMem => self.memory[self.reg_hl] = value as u8,
            InstrucionTarget::CMem => self.memory[0xFF00+self.reg_bc.lo() as u16] = value as u8,
            InstrucionTarget::A8 => {
                let offset = self.get_imm_u8() as u16;
                self.memory[0xFF00 + offset] = value as u8;
            },
            InstrucionTarget::AF => self.reg_af.value = value,
            InstrucionTarget::BC => self.reg_bc.value = value,
            InstrucionTarget::DE => self.reg_de.value = value,
            InstrucionTarget::HL => self.reg_hl.value = value,
            InstrucionTarget::SP => self.stack_pointer.value = value,
            InstrucionTarget::BCMem => self.memory[self.reg_bc] = value as u8,
            InstrucionTarget::DEMem => self.memory[self.reg_de] = value as u8,
            InstrucionTarget::A16 => {
                let addr = self.get_imm_16();
                self.memory[addr] = value as u8;
            },
            InstrucionTarget::PC => self.pc.value = value,
            _ => panic!("Unhandled write_target to {}", target)
        }
    }

    /// Returns value of specified source `target` as u16
    #[allow(overflowing_literals)]
    fn read_target(&mut self, target: InstrucionTarget) -> u16 {
        match target {
            InstrucionTarget::A => self.reg_af.hi() as u16,
            InstrucionTarget::B => self.reg_bc.hi() as u16,
            InstrucionTarget::C => self.reg_bc.lo() as u16,
            InstrucionTarget::D => self.reg_de.hi() as u16,
            InstrucionTarget::E => self.reg_de.lo() as u16,
            InstrucionTarget::H => self.reg_hl.hi() as u16,
            InstrucionTarget::L => self.reg_hl.lo() as u16,
            InstrucionTarget::HLMem => self.memory[self.reg_hl.value as usize] as u16,
            InstrucionTarget::CMem => self.memory[0xFF00 + self.reg_bc.lo() as u16] as u16,
            InstrucionTarget::U8 => self.get_imm_u8() as u16,
            InstrucionTarget::A8 => {
                let offset = self.get_imm_u8() as u16;
                self.memory[0xFF00 + offset] as u16
            },
            InstrucionTarget::AF => self.reg_af.value,
            InstrucionTarget::BC => self.reg_bc.value,
            InstrucionTarget::DE => self.reg_de.value,
            InstrucionTarget::HL => self.reg_hl.value,
            InstrucionTarget::SP => self.stack_pointer.value,
            InstrucionTarget::BCMem => self.memory[self.reg_bc.value] as u16,
            InstrucionTarget::DEMem => self.memory[self.reg_de.value] as u16,
            InstrucionTarget::U16 => self.get_imm_16(),
            InstrucionTarget::A16 => {
                let addr = self.get_imm_16();
                self.memory[addr] as u16
            },
            InstrucionTarget::I8 => {
                let v = self.get_imm_i8();
                v as u16
            }
            InstrucionTarget::SPi8 => {
                self.add(InstrucionTarget::SP, InstrucionTarget::I8, false);
                self.stack_pointer.value
            },
            InstrucionTarget::PC => self.pc.value,
            InstrucionTarget::ZCond => {
                if self.get_flag(Flag::Z) { return 1; }
                0
            },
            InstrucionTarget::CCond => {
                if self.get_flag(Flag::C) { return 1; }
                0
            },
            InstrucionTarget::NCCond => {
                if !self.get_flag(Flag::C) { return 1; }
                0
            },
            InstrucionTarget::NZCond => {
                if !self.get_flag(Flag::Z) { return 1; }
                0
            },
            _ => panic!("Unhandled read_target from {}", target)
        }
    }
    
    /// Returns bitness of target
    fn target_bitness(&mut self, target: InstrucionTarget) -> i32 {
        match target {
            InstrucionTarget::A | 
            InstrucionTarget::B | 
            InstrucionTarget::C | 
            InstrucionTarget::D | 
            InstrucionTarget::E | 
            InstrucionTarget::H | 
            InstrucionTarget::L | 
            InstrucionTarget::HLMem | 
            InstrucionTarget::CMem | 
            InstrucionTarget::U8 | 
            InstrucionTarget::A8 | 
            InstrucionTarget::I8 | 
            InstrucionTarget::A16 | 
            InstrucionTarget::BCMem | 
            InstrucionTarget::DEMem => 8,
            InstrucionTarget::AF | 
            InstrucionTarget::BC | 
            InstrucionTarget::DE | 
            InstrucionTarget::HL | 
            InstrucionTarget::SP | 
            InstrucionTarget::SPi8 |
            InstrucionTarget::PC |
            InstrucionTarget::U16 => 16,
            InstrucionTarget::ZCond |
            InstrucionTarget::CCond |
            InstrucionTarget::NZCond |
            InstrucionTarget::NCCond => 1,
            InstrucionTarget::Blank => 0,
        }
    }

    /// ## LD wrapper
    /// Loads target `from` into target `to`
    /// ### Flags
    /// unaffected
    fn load(&mut self, to: InstrucionTarget, from: InstrucionTarget) {
            let value = self.read_target(from);
            self.write_target(to, value);
        }
        
    /// ## ADD/ADC wrapper
    /// Adds `from` to `to` and stores the result to target `to`
    /// ### Flags
    /// z - set if result value is zero \
    /// n - reset \
    /// h - set if (result & 0xF) is greater than 0xF \
    /// c - set if result is greater than 0xFF
    fn add(&mut self, to: InstrucionTarget, from: InstrucionTarget, carry: bool){
        let c = if carry && self.get_flag(Flag::C) { 1 } else { 0 };
        let (target, source) = (self.read_target(to), self.read_target(from));
        let value = target.wrapping_add(source + c);
        self.write_target(to, value);
        let result = self.read_target(to);
        self.reg_af.write_lo(self.build_flag(
            match to {
                InstrucionTarget::HL => self.get_flag(Flag::Z),
                InstrucionTarget::SP => false,
                _ => result == 0
            },
            false, 
            ((target & 0xF) + (source & 0xF) + c) > 0xF, 
            (target + source + c) > 0xFF
        ));
    }

    /// ## SUB/SBC wrapper
    /// Subtracts `from` from `to` and stores the result to target `to`
    /// ### Flags
    /// z - set if result value is zero \
    /// n - set \
    /// h - set if (result & 0xF) is less than 0x0 \
    /// c - set if result is less than 0x00
    fn sub(&mut self, to: InstrucionTarget, from: InstrucionTarget, carry: bool){
        let c = if carry && self.get_flag(Flag::C) { 1 } else { 0 };
        let (target, source) = (self.read_target(to), self.read_target(from));
        let value = target.wrapping_sub(source + c);
        self.write_target(to, value);
        let result = self.read_target(to);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            true, 
            (target & 0xF) < (source & 0xF) + c, 
            target < source + c
        ));
    }

    /// ## AND wrapper
    /// Bitwise `and` between `from` and `to`. Stores the result to target `to`
    /// ### Flags
    /// z - set if result value is zero \
    /// n - reset \
    /// h - set \
    /// c - reset
    fn and(&mut self, to: InstrucionTarget, from: InstrucionTarget) {
        let (target, source) = (self.read_target(to), self.read_target(from));
        self.write_target(to, target & source);
        let result = self.read_target(to);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            false, 
            true, 
            false
        ));
    }

    /// ## XOR wrapper
    /// Bitwise `Xor` between `from` and `to`. Stores the result to target `to`
    /// ### Flags
    /// z - set if result value is zero \
    /// n - reset \
    /// h - reset \
    /// c - reset
    fn xor(&mut self, to: InstrucionTarget, from: InstrucionTarget) {
        let (target, source) = (self.read_target(to), self.read_target(from));
        self.write_target(to, target ^ source);
        let result = self.read_target(to);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            false, 
            false, 
            false
        ));
    }

    /// ## OR wrapper
    /// Bitwise `or` between `from` and `to`. Stores the result to target `to`
    /// ### Flags
    /// z - set if result value is zero \
    /// n - reset \
    /// h - reset \
    /// c - reset
    fn or(&mut self, to: InstrucionTarget, from: InstrucionTarget) {
        let (target, source) = (self.read_target(to), self.read_target(from));
        self.write_target(to, target | source);
        let result = self.read_target(to);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            false, 
            false, 
            false
        ));
    }

    /// ## CP wrapper
    /// Compares `op1` with `op2` - see Flags below.
    /// ### Flags
    /// z - set if op1 == op2 \
    /// n - set \
    /// h - set if (op1 & 0xF) < (op2 & 0xF) \
    /// c - set if op1 < op2
    fn cp(&mut self, op1: InstrucionTarget, op2: InstrucionTarget) {
        let (frist, second) = (self.read_target(op1), self.read_target(op2));
        self.reg_af.write_lo(self.build_flag(
            frist == second,
            true, 
            frist & 0xF < second & 0xF, 
            frist < second
        ));
    }

    /// ## PUSH wrapper
    /// Pushes `from` to stack
    /// ### Flags
    /// unaffected
    fn push(&mut self, from: InstrucionTarget) {
        let r = Register::new(self.read_target(from));
        self._push(r);
    }

    /// ## POP wrapper
    /// Pops value from stack into `to`
    /// ### Flags
    /// unaffected
    fn pop(&mut self, to: InstrucionTarget) {
        let v = self._pop().value;
        self.write_target(to, v);
    }

    /// ## INC wrapper
    /// Increments `op`
    /// ### Flags
    /// z - set if result value is zero \
    /// n - reset \
    /// h - set if carry from bit 3 \
    /// c - unaffected
    fn inc(&mut self, op: InstrucionTarget) {
        let v = self.read_target(op);
        self.write_target(op, v.wrapping_add(1));
        if self.target_bitness(op) == 8 {
            let result = self.read_target(op);
            self.reg_af.write_lo(self.build_flag(
                result == 0,
                false,
                v & 0xF == 0xF, 
                self.get_flag(Flag::C)
            ));
        }
    }

    /// ## DEC wrapper
    /// Decrements `op`
    /// ### Flags
    /// z - set if result value is zero \
    /// n - set \
    /// h - set if no borrow from bit 4 \
    /// c - unaffected
    fn dec(&mut self, op: InstrucionTarget) {
        let v = self.read_target(op);
        self.write_target(op, v.wrapping_sub(1));
        if self.target_bitness(op) == 8 {
            let result = self.read_target(op);
            self.reg_af.write_lo(self.build_flag(
                result == 0,
                true,
                v & 0xF == 0, 
                self.get_flag(Flag::C)
            ));
        }
    }

    /// ## CPL wrapper
    /// Flips A register bits
    /// ### Flags
    /// z - unaffected \
    /// n - set \
    /// h - set \
    /// c - unaffected
    fn cpl(&mut self) {
        let a = !self.reg_af.hi();
        self.reg_af.write_hi(a);
        self.reg_af.write_lo(self.build_flag(
            self.get_flag(Flag::Z),
            true,
            true, 
            self.get_flag(Flag::C)
        ));
    }

    /// ## DAA wrapper
    /// wtf...
    /// ### Flags
    /// z - set if result value is zero \
    /// n - unaffected \
    /// h - reset \
    /// c - depends
    fn daa(&mut self) {
        let mut v = self.reg_af.hi();
        if self.get_flag(Flag::N) {
            if self.get_flag(Flag::C) { v = v.wrapping_sub(0x60); }
            if self.get_flag(Flag::H) { v = v.wrapping_sub(0x6);; }
        } else {
            if self.get_flag(Flag::C) || v > 0x99 { v = v.wrapping_add(0x60); self.set_flag(Flag::C)}
            if self.get_flag(Flag::H) || (v & 0x0f) > 0x09 { v = v.wrapping_add(0x6); }
        }
        self.reg_af.write_hi(v);
        self.reg_af.write_lo(self.build_flag(
            v == 0,
            self.get_flag(Flag::N),
            false, 
            self.get_flag(Flag::C)
        ));
    }

    /// ## RLC wrapper
    /// **Rotates** `op` left
    /// ### Flags
    /// z - reset \
    /// n - reset \
    /// h - reset \
    /// c - old bit 7
    fn rlc(&mut self, op: InstrucionTarget) {
        let t = self.read_target(op) as u8;
        let c = (t & 0x80) >> 7;
        let v = t.rotate_left(1);
        self.write_target(op, v as u16);
        self.reg_af.write_lo(self.build_flag(
            false,
            false,
            false, 
            c == 1
        )); 
    }

    /// ## RL wrapper
    /// **Rotates** `op` left with carry flag
    /// ### Flags
    /// z - reset \
    /// n - reset \
    /// h - reset \
    /// c - old bit 7
    fn rl(&mut self, op: InstrucionTarget) {
        let t = self.read_target(op) as u8;
        let c = (t & 0x80) >> 7;
        let mut v = t.rotate_left(1);
        if self.get_flag(Flag::C) { v |= 1; }
        self.write_target(op, v as u16);
        self.reg_af.write_lo(self.build_flag(
            false,
            false,
            false, 
            c == 1
        ));
    }

    /// ## RRC wrapper
    /// **Rotates** `op` right
    /// ### Flags
    /// z - reset \
    /// n - reset \
    /// h - reset \
    /// c - old bit 0
    fn rrc(&mut self, op: InstrucionTarget) {
        let t = self.read_target(op) as u8;
        let c = t & 1;
        let v = t.rotate_right(1);
        self.write_target(op, v as u16);
        self.reg_af.write_lo(self.build_flag(
            false,
            false,
            false, 
            c == 1
        )); 
    }
    
    /// ## RR wrapper
    /// **Rotates** A register right with carry flag
    /// ### Flags
    /// z - reset \
    /// n - reset \
    /// h - reset \
    /// c - old bit 0
    fn rr(&mut self, op: InstrucionTarget) {
        let t = self.read_target(op) as u8;
        let c = t & 1;
        let mut v = t.rotate_right(1);
        if self.get_flag(Flag::C) { v |= 0x80; }
        self.write_target(op, v as u16);
        self.reg_af.write_lo(self.build_flag(
            false,
            false,
            false, 
            c == 1
        ));
    }

    /// ## JP wrapper
    /// Jump to address `dest`
    /// ### Flags
    /// unaffected
    fn jp(&mut self, cond: InstrucionTarget, dest: InstrucionTarget, relative: bool) {
        let destination = if relative {
            let offset = self.read_target(dest);
            self.pc.value.wrapping_add(offset) 
        } else {
            self.read_target(dest)
        };
        match cond {
            InstrucionTarget::ZCond |
            InstrucionTarget::CCond |
            InstrucionTarget::NZCond |
            InstrucionTarget::NCCond => {
                if self.read_target(cond) == 1 {
                    self.pc.value = destination;
                    self.cycles += 4;
                }
            },
            InstrucionTarget::Blank => self.pc.value = destination,
            _ => panic!("unhandled jump condition at {:X}", self.pc.value),
        }
    }

    /// ## CALL wrapper
    /// Pushes address of next instruction onto stack then jumps to address `dest`
    /// ### Flags
    /// unaffected
    fn call(&mut self, cond: InstrucionTarget, dest: InstrucionTarget) {
        let destination = self.read_target(dest);
        match cond {
            InstrucionTarget::ZCond |
            InstrucionTarget::CCond |
            InstrucionTarget::NZCond |
            InstrucionTarget::NCCond => {
                if self.read_target(cond) == 1 {
                    self._push(self.pc);
                    self.pc.value = destination;
                    self.cycles += 12;
                }
            },
            InstrucionTarget::Blank => {
                self._push(self.pc);
                self.pc.value = destination
            },
            _ => panic!("unhandled call condition at {:X}", self.pc.value),
        }
    }

    /// ## RST wrapper
    /// `CALL`s to 0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38
    /// ### Flags
    /// unaffected
    fn rst(&mut self, offset: u16) {
        self._push(self.pc);
        self.pc.value = 0x0000 + offset;
    }

    /// ## RET/RETI wrapper
    /// Returns to address stored on stack. Enables interrupts if `ei` is `true`
    /// ### Flags
    /// unaffected
    fn ret(&mut self, cond: InstrucionTarget, ei: bool) {
        let destination = self._pop().value;
        match cond {
            InstrucionTarget::ZCond |
            InstrucionTarget::CCond |
            InstrucionTarget::NZCond |
            InstrucionTarget::NCCond => {
                if self.read_target(cond) == 1 {
                    self.pc.value = destination;
                    self.cycles += 12;
                }
            },
            InstrucionTarget::Blank => self.pc.value = destination,
            _ => panic!("unhandled return condition at {:X}", self.pc.value),
        };
        if ei { self.ei(); }
    }

    /// ## DI wrapper
    /// Disables interrupts(`ime` = `false`)
    /// ### Flags
    /// unaffected
    fn di(&mut self) {
        self.ime = false;
    }
    
    /// ## EI wrapper
    /// Enables interrupts(`ime` = `true`)
    /// ### Flags
    /// unaffected
    fn ei(&mut self) {
        self.ime = true;
    }

    /// ## SCF wrapper
    /// Sets carry flag
    /// ### Flags
    /// z - unaffected \
    /// n - reset \
    /// h - reset \
    /// c - set
    fn scf(&mut self) {
        self.reg_af.write_lo(self.build_flag(
            self.get_flag(Flag::Z),
            false,
            false, 
            true
        ));
    }

    /// ## CCF wrapper
    /// Flips carry flag
    /// ### Flags
    /// z - unaffected \
    /// n - reset \
    /// h - reset \
    /// c - flip
    fn ccf(&mut self) {
        self.reg_af.write_lo(self.build_flag(
            self.get_flag(Flag::Z),
            false,
            false, 
            !self.get_flag(Flag::C)
        ));
    }

    /// ## STOP wrapper
    /// Stops CPU
    /// ### Flags
    /// unaffected
    fn stop(&mut self) {
        self.stopped = true;
    }

    /// ## HALT wrapper
    /// Halts CPU
    /// ### Flags
    /// unaffected
    fn halt(&mut self) {
        self.halted = true;
    }

    /// ## SLA wrapper
    /// Shifts `op` left. Least significant bit is set to 0
    /// ### Flags
    /// z - set if result is zero \
    /// n - reset \
    /// h - reset \
    /// c - old bit 7
    fn sla(&mut self, op: InstrucionTarget) {
        let v = self.read_target(op) as u8;
        let c = (v & 0x80) >> 7;
        self.write_target(op, (v.wrapping_shl(1) & 0xFE) as u16 );
        let result = self.read_target(op);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            false,
            false, 
            c == 1
        ));
    }

    /// ## SRA wrapper
    /// Shifts `op` right
    /// ### Flags
    /// z - set if result is zero \
    /// n - reset \
    /// h - reset \
    /// c - old bit 0
    fn sra(&mut self, op: InstrucionTarget) {
        let v = self.read_target(op) as u8;
        let c = v & 1;
        self.write_target(op, v.wrapping_shr(1) as u16);
        let result = self.read_target(op);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            false,
            false, 
            c == 1
        ));
    }

    /// ## SRL wrapper
    /// Shifts `op` right. Most significant bit is set to 0
    /// ### Flags
    /// z - set if result is zero \
    /// n - reset \
    /// h - reset \
    /// c - old bit 0
    fn srl(&mut self, op: InstrucionTarget) {
        let v = self.read_target(op) as u8;
        let c = v & 1;
        self.write_target(op, (v.wrapping_shr(1) & 0x7F) as u16);
        let result = self.read_target(op);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            false,
            false, 
            c == 1
        ));
    }

    /// ## SWAP wrapper
    /// Swaps upper and lower nibbles of `op`
    /// ### Flags
    /// z - set if result is zero \
    /// n - reset \
    /// h - reset \
    /// c - reset
    fn swap(&mut self, op: InstrucionTarget) {
        let v = self.read_target(op) as u8;
        let low = v & 0x0F;
        let upper = v & 0xF0;
        let new = (low << 4) | upper;
        self.write_target(op, new as u16);
        let result = self.read_target(op);
        self.reg_af.write_lo(self.build_flag(
            result == 0,
            false,
            false, 
            false
        ));
    }

    /// ## BIT wrapper
    /// Checks specified `bit` of `op`
    /// ### Flags
    /// z - set if result is zero \
    /// n - reset \
    /// h - reset \
    /// c - reset
    fn bit(&mut self, bit: u8, op: InstrucionTarget) {
        let v = self.read_target(op) as u8;
        let test = (v >> bit) & 1;
        self.reg_af.write_lo(self.build_flag(
            test == 0,
            false,
            true, 
            self.get_flag(Flag::C)
        ));
    }
    
    /// ## RES wrapper
    /// Restes specified `bit` of `op`
    /// ### Flags
    /// unaffected
    fn res(&mut self, bit: u8, op: InstrucionTarget) {
        let v = (self.read_target(op) as u8) & !(1 << bit);
        self.write_target(op, v as u16);
    }
    
    /// ## SET wrapper
    /// Sets specified `bit` of `op`
    /// ### Flags
    /// unaffected
    fn set(&mut self, bit: u8, op: InstrucionTarget) {
        let v = (self.read_target(op) as u8) | (1 << bit);
        self.write_target(op, v as u16);
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LD(to, from) => self.load(to, from),
            Instruction::LDI(to, from) => {
                self.load(to, from);
                self.reg_hl.value += 1;
            },
            Instruction::LDD(to, from) => {
                self.load(to, from);
                self.reg_hl.value -= 1;
            },
            Instruction::PUSH(from) => self.push(from),
            Instruction::POP(to) => self.pop(to),
            Instruction::ADD(to, from) => self.add(to, from, false),
            Instruction::ADC(to, from) => self.add(to, from, true),
            Instruction::SUB(to, from) => self.sub(to, from, false),
            Instruction::SBC(to, from) => self.sub(to, from, true),
            Instruction::AND(to, from) => self.and(to, from),
            Instruction::XOR(to, from) => self.xor(to, from),
            Instruction::OR(to, from) => self.or(to, from),
            Instruction::CP(op1, op2) => self.cp(op1, op2),
            Instruction::INC(op) => self.inc(op),
            Instruction::DEC(op) => self.dec(op),
            Instruction::DAA => self.daa(),
            Instruction::CPL => self.cpl(),
            Instruction::RLCA => self.rlc(InstrucionTarget::A),
            Instruction::RLA => self.rl(InstrucionTarget::A),
            Instruction::RRCA => self.rrc(InstrucionTarget::A),
            Instruction::RRA => self.rr(InstrucionTarget::A),
            Instruction::RLC(op) => self.rlc(op),
            Instruction::RL(op) => self.rl(op),
            Instruction::RRC(op) => self.rrc(op),
            Instruction::RR(op) => self.rr(op),
            Instruction::SLA(op) => self.sla(op),
            Instruction::SWAP(op) => self.swap(op),
            Instruction::SRA(op) => self.sra(op),
            Instruction::SRL(op) => self.srl(op),
            Instruction::BIT(bit, op) => self.bit(bit, op),
            Instruction::SET(bit, op) => self.set(bit, op),
            Instruction::RES(bit, op) => self.res(bit, op),
            Instruction::CCF => self.ccf(),
            Instruction::SCF => self.scf(),
            Instruction::NOP => (),
            Instruction::HALT => self.halt(),
            Instruction::STOP => self.stop(),
            Instruction::DI => self.di(),
            Instruction::EI => self.ei(),
            Instruction::JP(cond, dest) => self.jp(cond, dest, false),
            Instruction::JR(cond, dest) => self.jp(cond, dest, true),
            Instruction::CALL(cond, dest) => self.call(cond, dest),
            Instruction::RET(cond) => self.ret(cond, false),
            Instruction::RETI => self.ret(InstrucionTarget::Blank, true),
            Instruction::RST(offset) => self.rst(offset),
            Instruction::PREFIX => self.prefix(),
            Instruction::TODO => todo!(),
            Instruction::XXX => panic!("Unhandled instruction {} at {:X}", instruction.to_string(), self.stack_pointer.value),
        }
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
        //  .field("name: &str", value: &dyn fmt::Debug)
         .finish()
    }
}

#[cfg(test)]
mod tests{
    use std::convert::TryInto;

    use super::*;

    fn test_ld(
        cpu: &mut CPU,
        value: u16,
        cycles: i32, 
        to: InstrucionTarget,
        from: InstrucionTarget,
        payload: &Vec<u8>,
        mod_hl: i32,
    ){
        cpu.pc.value = 0x0100;
        cpu.write_target(from, value);
        // cpu.memory.load(payload.to_vec());
        for _ in 0..cycles {
            cpu.clock();
        }

        cpu.reg_hl.value = cpu.reg_hl.value.wrapping_add(mod_hl as u16);
        cpu.pc.value = 0x0101;
        let desired = cpu.read_target(to);
        assert_eq!(
            desired, 
            value, 
            "failed on opcode {:#04X} LD {}, {}",
            cpu.memory[0x0100 as usize],
            to,
            from
        );
        assert_eq!(cpu.cycles, 0);
        cpu.reg_hl.value = cpu.reg_hl.value.wrapping_sub(mod_hl as u16);
    }

    #[test]
    fn test_load_instructions() {
        let mut cpu = CPU::new();
        let table = cpu.opcode_table;
        let value: u16 = 0x13;
        let addr: u16 = 0x1313; // TODO build address with value variable

        for (op, instruction) in table.iter().enumerate() {
            match instruction.0 {
                Instruction::LD(to, from) => {
                    let mut payload: Vec<u8> = vec![];
                    match from {
                        InstrucionTarget::U8 => {
                            payload.push(op as u8);
                            payload.push(value.try_into().unwrap());
                        },
                        InstrucionTarget::U16 => {
                            payload.push(op as u8);
                            payload.push(0x00);
                            payload.push(value.try_into().unwrap());
                        },
                        InstrucionTarget::A8 => {
                            payload.push(op as u8);
                            payload.push(value.try_into().unwrap());
                            cpu.memory[0xFF00+value] = value as u8;
                        },
                        InstrucionTarget::A16 => {
                            payload.push(op as u8);
                            payload.push(value.try_into().unwrap());
                            payload.push(value.try_into().unwrap());
                            cpu.memory[addr as usize] = value as u8;
                        }
                        InstrucionTarget::CMem => {
                            payload.push(op as u8);
                            cpu.reg_bc.write_lo(value.try_into().unwrap());
                            cpu.memory[0xFF00+cpu.reg_bc.lo() as u16] = value as u8;
                        },
                        InstrucionTarget::HLMem => {
                            payload.push(op as u8);
                            cpu.reg_hl.value = addr;
                            cpu.memory[cpu.reg_hl] = value as u8;
                        },
                        InstrucionTarget::BCMem => {
                            payload.push(op as u8);
                            cpu.reg_bc.value = addr;
                            cpu.memory[cpu.reg_hl] = value as u8;
                        },
                        InstrucionTarget::DEMem => {
                            payload.push(op as u8);
                            cpu.reg_de.value = addr;
                            cpu.memory[cpu.reg_hl] = value as u8;
                        },
                        _ => payload.push(op as u8),
                    };
                    test_ld(
                        &mut cpu, 
                        value, 
                        instruction.1, 
                        to, 
                        from, 
                        &payload,
                        0
                    );
                },
                Instruction::LDI(to, from) => {
                    let payload: Vec<u8> = vec![op as u8];
                    cpu.reg_hl.value = addr;
                    cpu.memory[cpu.reg_hl] = value as u8;

                    test_ld(
                        &mut cpu, 
                        value, 
                        instruction.1, 
                        to, 
                        from, 
                        &payload,
                        -1
                    );
                    assert_eq!(cpu.reg_hl.value, 0x1314);
                },
                Instruction::LDD(to, from) => {
                    let payload: Vec<u8> = vec![op as u8];
                    cpu.reg_hl.value = addr;
                    cpu.memory[cpu.reg_hl] = value as u8;

                    test_ld(
                        &mut cpu, 
                        value, 
                        instruction.1, 
                        to, 
                        from, 
                        &payload,
                        1
                    );
                    assert_eq!(cpu.reg_hl.value, 0x1312);
                },
                _ => continue,
            }
            // test_ld(
            //     &mut cpu, 
            //     0x13, 
            //     4, 
            //     InstrucionTarget::A, 
            //     InstrucionTarget::B,
            //     vec![0x78], 
            // );
        }
    }
}