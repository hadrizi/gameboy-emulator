use std::fmt;

pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub c: bool,
    pub h: bool
}

impl Flags {
    pub fn new() -> Flags {
        Flags{
            z: false,
            n: false,
            c: false,
            h: false,
        }
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flags")
        .field("z", &self.z)
        .field("n", &self.n)
        .field("c", &self.c)
        .field("h", &self.h)
         .finish()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}