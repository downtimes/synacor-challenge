use std::ops::{Add, Mul, Rem, BitAnd, BitOr, Not, Sub};
use std::fmt;

pub const MAX: u16 = MATH_MODULO-1;
const MATH_MODULO: u16 = 1 << 15;

//TODO: Implement the from trait for this one so we don't have to 
//write Value all the time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct U15(u16);

impl fmt::Display for U15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl U15 {
    pub fn new(val: u16) -> U15 {
        debug_assert!(val <= MAX);
        U15(val)
    }

    pub fn to_char(&self) -> char {
        debug_assert!(self.0 < 255);
        self.0 as u8 as char
    }

    pub fn to_idx(&self) -> usize {
        self.0 as usize
    }

    pub fn to_u16(&self) -> u16 {
        self.0 as u16
    }
}

impl Rem<U15> for U15 {
    type Output = U15;

    fn rem(self, other: U15) -> U15 {
        U15::new((self.0 % other.0))
    }
}

impl Add<U15> for U15 {
    type Output = U15;

    fn add(self, other: U15) -> U15 {
        U15::new((self.0 + other.0) % MATH_MODULO)
    }
}

impl Add<u16> for U15 {
    type Output = U15;

    fn add(self, other: u16) -> U15 {
        U15::new((self.0 + other) % MATH_MODULO)
    }
}

impl Mul<U15> for U15 {
    type Output = U15;

    fn mul(self, other: U15) -> U15 {
        U15::new((((self.0 as u32) * (other.0 as u32)) % (MATH_MODULO as u32)) as u16)
    }
}

impl Sub<U15> for U15 {
    type Output = U15;

    fn sub(self, other:U15) -> U15 {
        U15::new(self.0 - other.0)
    }
}

impl BitAnd<U15> for U15 {
    type Output = U15;

    fn bitand(self, other: U15) -> U15 {
        U15::new(self.0 & other.0)
    }
}

impl BitOr<U15> for U15 {
    type Output = U15;

    fn bitor(self, other: U15) -> U15 {
        U15::new(self.0 | other.0)
    }
}

impl Not for U15 {
    type Output = U15;

    //Implement not a bit differently according to the specification
    //of the arch
    fn not(self) -> U15 {
        U15::new(1<<15 ^ !self.0)
    }
}