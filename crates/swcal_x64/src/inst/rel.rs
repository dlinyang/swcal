use crate::inst::encode::*;
use crate::inst::imm::*;

#[derive(Debug, Clone, Copy)]
pub enum Rel {
    Rel8(i8),
    Rel16(i16),
    Rel32(i32),
}

impl Rel {
    pub fn width(&self) -> u16 {
        match self {
            Rel::Rel8(_) => 8,
            Rel::Rel16(_) => 16,
            Rel::Rel32(_) => 32,
        }
    }
}

impl std::fmt::Display for Rel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rel::Rel8(val) => write!(f, "{}i8", val),
            Rel::Rel16(val) => write!(f, "{}i16", val),
            Rel::Rel32(val) => write!(f, "{}i32", val),
        }
    }
}

macro_rules! rel_n {
    ($name: ident, $t: ty, $ct: ty, $f:ident, $imm_field:ident) => {
        pub struct $name {
            val: $t,
        }

        impl $name {
            #[inline]
            pub fn new( val: $t) -> Self {
                Self { val }
            }

            pub fn value(&self) -> $t {
                self.val
            }

            pub fn encode(&self, buf: &mut impl CodeSink) {
                buf.$f(self.val as $ct)
            }
        }

        impl TryFrom<Rel> for $name {
            type Error = String;
            fn try_from(value: Rel) -> Result<Self, Self::Error> {
                match value {
                    Rel::$name(val) => Ok(Self::new(val)),
                    _ => Err("Unmatched imm type".into()),
                }
            }
        }

        impl TryFrom<Imm> for $name {
            type Error = String;
            fn try_from(value: Imm) -> Result<Self, Self::Error> {
                match value {
                    Imm::$imm_field(val) => Ok(Self::new(val as $t)),
                    _ => Err("Unmatched imm type".into()),
                }
            }
        }
    };
}

rel_n!(Rel8, i8, u8, putb, Imm8);
rel_n!(Rel16, i16, u16, putw, Imm16);
rel_n!(Rel32, i32, u32, putd, Imm32);
