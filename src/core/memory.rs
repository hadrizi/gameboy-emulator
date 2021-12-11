use std::{ops::{Index, IndexMut}, path::Path, fs::File, io::Read};
use crate::core::register::Register;

const ROM_SIZE: u64 = 32 * 1024;

pub struct Memory {
    _mem: [u8; 64*1024],

    div: usize,
    tima: usize,
    tma: usize,
    tac: usize,

    ie: usize,
    iflag: usize
}

impl Memory {
    pub fn new() -> Memory {
        Memory{
            _mem: [0; 64*1024],
            // timers
            div: 0xFF04,
            tima: 0xFF05,
            tma: 0xFF06,
            tac: 0xFF06,

            // interrupts
            ie: 0xFFFF,
            iflag: 0xFF0F,
        }
    }

    pub fn load(&mut self, path: &Path){
        let file = File::open(path).unwrap();
        let mut data = Vec::new();

        let length = file.take(ROM_SIZE).read_to_end(&mut data).unwrap();
        if length <= ROM_SIZE as usize {
            for (place, value) in self._mem.iter_mut().zip(data.iter()) {
                *place = *value
            }
        } else {
            panic!("Invalid ROM size")
        }
    }

    pub fn read(&self, i: usize) -> u8 {
        self._mem[i]
    }

    pub fn write(&mut self, i: usize, v: u8) {
        if i < 0x0800 { return; }
        if (0xFEA0..=0xFEFFusize).contains(&i) { return; }
        if (0xE000..=0xFDFFusize).contains(&i) { return; }

        if (0xC000..=0xDDFFusize).contains(&i) {
            self._mem[i + 0x2000] = v;
        }

        if i == self.div {
            self._mem[i] = 0;
        }

        self._mem[i] = v;
    }

    pub fn reset_hardware_registers(&mut self) {
        self._mem[self.div]  = 0xAB;
        self._mem[self.tima] = 0x00;
        self._mem[self.tma]  = 0x00;
        self._mem[self.tac]  = 0xF8;

        self._mem[self.iflag] = 0xE1;
        self._mem[self.ie] = 0x00;
    }

    // Timers

    /// Divider Register
    pub fn div(&self) -> u8 { self._mem[self.div] }
    pub fn inc_div(&mut self) { self._mem[self.div] = self._mem[self.div].wrapping_add(1); }
    
    /// Timer counter
    pub fn tima(&self) -> u8 { self._mem[self.tima] }
    pub fn inc_tima(&mut self) { self._mem[self.tima] = self._mem[self.tima].wrapping_add(1); }
    pub fn reset_tima(&mut self) { self._mem[self.tima] = self._mem[self.tma]; }

    /// Timer Modulo \
    /// value to be set in TIMA when it overflows
    pub fn tma(&self) -> u8 { self._mem[self.tma] }
    
    /// Timer control \
    /// Bit 2    - Timer enable \
    /// Bits 1-0 - Input Clock select \
    ///            00: each 1024 cycle \
    ///            01: each 16 cycles \
    ///            10: each 64 cycle \
    ///            11: each 256 cycle
    pub fn tac(&self) -> u8 { self._mem[self.tac] }

    // Interrupts

    /// Interrupt Enable
    pub fn is_ie_set(&mut self, id: i32) -> bool { 
        let r = self._mem[self.ie];
        (r >> id) & 1 == 1 
    }

    /// Interrupt Flag
    pub fn reset_iflag_bit(&mut self, id: i32) { self._mem[self.iflag] &= !(1 << id); }
    pub fn is_iflag_set(&mut self, id: i32) -> bool { 
        let r = self._mem[self.iflag];
        (r >> id) & 1 == 1 
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self._mem[i]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self._mem[i]
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, i: u16) -> &Self::Output {
        &self._mem[i as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, i: u16) -> &mut Self::Output {
        &mut self._mem[i as usize]
    }
}

impl Index<Register> for Memory {
    type Output = u8;

    fn index(&self, i: Register) -> &Self::Output {
        &self._mem[i.value as usize]
    }
}

impl IndexMut<Register> for Memory {
    fn index_mut(&mut self, i: Register) -> &mut Self::Output {
        &mut self._mem[i.value as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_index() {
        let addr: u16 = 0x0000;
        let mut mem = Memory::new();
        assert_eq!(mem[addr], 0);

        mem[addr] = 0xFF;
        assert_eq!(mem[addr], 0xFF);
    }
}