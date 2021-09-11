use std::fmt;

pub struct Flags {
    value: u8,
}

impl Flags {
    pub fn new() -> Flags {
        Flags{
            value: 0b00000000,
        }
    }

    fn set(&mut self, i: u8) {
        self.value ^= 1 << (7 - i);
    }

    pub fn set_z(&mut self) -> u8 {
        self.set(0);
        self.z()
    }
    
    pub fn set_n(&mut self) -> u8 {
        self.set(1);
        self.n()
    }
    
    pub fn set_c(&mut self) -> u8 {
        self.set(2);
        self.c()
    }

    pub fn set_h(&mut self) -> u8 {
        self.set(3);
        self.h()
    }

    pub fn z(&self) -> u8 {
        (self.value & 0b10000000) >> 7
    }

    pub fn n(&self) -> u8 {
        (self.value & 0b01000000) >> 6
    }

    pub fn c(&self) -> u8 {
        (self.value & 0b00100000) >> 5
    }
    
    pub fn h(&self) -> u8 {
        (self.value & 0b00010000) >> 4
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flags")
        .field("z", &self.z())
        .field("n", &self.n())
        .field("c", &self.c())
        .field("h", &self.h())
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        let mut flags = Flags::new();
        assert_eq!(flags.z(), 0);
        assert_eq!(flags.n(), 0);
        assert_eq!(flags.c(), 0);
        assert_eq!(flags.h(), 0);

        flags.set_z();
        assert_eq!(flags.z(), 1);
        flags.set_z();
        assert_eq!(flags.z(), 0);
        
        flags.set_n();
        assert_eq!(flags.n(), 1);
        flags.set_n();
        assert_eq!(flags.n(), 0);
        
        flags.set_c();
        assert_eq!(flags.c(), 1);
        flags.set_c();
        assert_eq!(flags.c(), 0);
        
        flags.set_h();
        assert_eq!(flags.h(), 1);
        flags.set_h();
        assert_eq!(flags.h(), 0);
    }
}