use std::ops::{Index, IndexMut};
use crate::core::register::Register;

pub struct Memory {
    _mem: [u8; 64*1024],
}

impl Memory {
    pub fn new() -> Memory {
        Memory{
            _mem: [0; 64*1024],
        }
    }

    // seems stupid actually, may be a beteer way to do it
    // probably slices?
    pub fn load(&mut self, payload: Vec<u8>) {
        let mut i = 0x0100;
        for n in payload.iter() {
            self[i] = *n;
            i += 1;
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
        let mut mem = Memory::new();
        assert_eq!(mem[0x0000], 0);

        mem[0x0000] = 0xFF;
        assert_eq!(mem[0x0000], 0xFF);
    }
}