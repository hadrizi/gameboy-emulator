// TODO: implement https://doc.rust-lang.org/core/ops/
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
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

impl Add<Register> for Register {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<u16> for Register {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        Self {
            value: self.value + other,
        }
    }
}

impl Add<u8> for Register {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        Self {
            value: self.value + other as u16,
        }
    }
}

impl AddAssign<Register> for Register {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value + other.value,
        };
    }
}

impl AddAssign<u16> for Register {
    fn add_assign(&mut self, other: u16) {
        *self = Self {
            value: self.value + other,
        };
    }
}

impl AddAssign<u8> for Register {
    fn add_assign(&mut self, other: u8) {
        *self = Self {
            value: self.value + other as u16,
        };
    }
}

impl Sub<Register> for Register {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Sub<u16> for Register {
    type Output = Self;

    fn sub(self, other: u16) -> Self {
        Self {
            value: self.value - other,
        }
    }
}

impl Sub<u8> for Register {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        Self {
            value: self.value - other as u16,
        }
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

    #[test]
    fn test_register_add() {
        let mut r1 = Register::new(0x0000);
        let r2     = Register::new(0x0005);
        
        r1 = r1 + r2;
        assert_eq!(r1.value, 0x0005);

        r1 = r1 + 0x0005 as u16;
        assert_eq!(r1.value, 0x000A);

        r1 = r1 + 0x05 as u8;
        assert_eq!(r1.value, 0x000F);
    }

    #[test]
    fn test_register_sub() {
        let mut r1 = Register::new(0x000F);
        let r2     = Register::new(0x0005);

        r1 = r1 - r2;
        assert_eq!(r1.value, 0x000A);

        r1 = r1 - 0x0005 as u16;
        assert_eq!(r1.value, 0x0005);

        r1 = r1 - 0x05 as u8;
        assert_eq!(r1.value, 0x0000);
    }
}