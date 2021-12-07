use std::{ops::{Index, IndexMut}, path::Path, fs::File, io::Read};
use crate::core::register::Register;

const ROM_SIZE: u64 = 32 * 1024;

pub struct Memory {
    _mem: [u8; 64*1024],
}

impl Memory {
    pub fn new() -> Memory {
        Memory{
            _mem: [0; 64*1024],
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