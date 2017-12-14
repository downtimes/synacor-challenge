use std::ops::{Add, Mul, Rem, BitAnd, BitOr, Not, Sub};
use std::convert::From;
use std::fmt;

pub const MAX: u16 = MATH_MODULO-1;
const MATH_MODULO: u16 = 1 << 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct U15(u16);

impl fmt::Display for U15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

//Didn't want to implement this but since standard literals are assumed to be
//i32 and I'm to lazy for writing u16 behind every literal I just implemented this
//conversion
impl From<i32> for U15 {
    fn from(num: i32) -> U15 {
        debug_assert!(0 <= num && num <= i32::from(MAX));
        U15(num as u16)
    }
}

impl From<u8> for U15 {
    fn from(num: u8) -> U15 {
        U15(u16::from(num))
    }
}

impl From<u16> for U15 {
    fn from(num: u16) -> U15 {
        debug_assert!(num <= MAX);
        U15(num)
    }
}

impl U15 {
    pub fn to_char(&self) -> char {
        debug_assert!(self.0 <= u16::from(std::u8::MAX));
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
        U15::from(self.0 % other.0)
    }
}

impl Add<U15> for U15 {
    type Output = U15;

    //Wrapping behaviour as per specification
    fn add(self, other: U15) -> U15 {
        U15::from((self.0 + other.0) % MATH_MODULO)
    }
}

impl Add<u16> for U15 {
    type Output = U15;

    //Wrapping behaviour as per specification
    fn add(self, other: u16) -> U15 {
        U15::from((self.0 + other) % MATH_MODULO)
    }
}

impl Mul<U15> for U15 {
    type Output = U15;

    //Wrapping behaviour as per specification
    fn mul(self, other: U15) -> U15 {
        U15::from(((u32::from(self.0) * u32::from(other.0)) % u32::from(MATH_MODULO)) as u16)
    }
}

impl Sub<U15> for U15 {
    type Output = U15;

    fn sub(self, other:U15) -> U15 {
        U15::from(self.0 - other.0)
    }
}

impl BitAnd<U15> for U15 {
    type Output = U15;

    fn bitand(self, other: U15) -> U15 {
        U15::from(self.0 & other.0)
    }
}

impl BitOr<U15> for U15 {
    type Output = U15;

    fn bitor(self, other: U15) -> U15 {
        U15::from(self.0 | other.0)
    }
}

impl Not for U15 {
    type Output = U15;

    //Implement not a bit differently according to the specification
    //of the arch
    fn not(self) -> U15 {
        U15::from(1<<15 ^ !self.0)
    }
}