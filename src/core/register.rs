// TODO: implement https://doc.rust-lang.org/core/ops/
use std::fmt;

pub struct Register {
    pub value: u16,
}

impl Register {
    pub fn new(v: u16) -> Register {
        Register {
            value: v,
        }
    }

    pub fn hi(&self) -> u8 {
        let _t = self.value & 0xFF00;
        (_t >> 8) as u8
    }
    
    pub fn lo(&self) -> u8 {
        let _t = self.value & 0x00FF;
        _t as u8
    }
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Register")
         .field("value", &self.value)
         .field("high value", &self.hi())
         .field("lower value", &self.lo())
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let r = Register::new(0xB668);
        assert_eq!(r.value, 0xB668);
        assert_eq!(r.hi(), 0xB6);
        assert_eq!(r.lo(), 0x68);
    }
}