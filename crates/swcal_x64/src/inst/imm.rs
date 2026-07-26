use std::usize;

use crate::inst::encode::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imm {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
    Imm64(u64),
}

impl Imm {
    pub fn width(&self) -> u16 {
        match self {
            Imm::Imm8(_) => 8,
            Imm::Imm16(_) => 16,
            Imm::Imm32(_) => 32,
            Imm::Imm64(_) => 64,
        }
    }

    pub fn fit_val(val: usize) -> Self {
        if val <= u8::MAX as usize {
            Self::Imm8(val as u8)
        }
        else if val <= u16::MAX as usize {
            Self::Imm16(val as u16)
        }
        else if val <= u32::MAX as usize {
            Self::Imm32(val as u32)
        }
        else {
            Self::Imm64(val as u64)
        }
    }

    // pub fn is_w64(&self) -> bool {
    //     self.width() == 64
    // }

    pub fn try_from_width(width: u16) -> Self {
        match width {
            8 => Self::Imm8(0),
            16 => Self::Imm16(0),
            32 => Self::Imm32(0),
            64 => Self::Imm64(0),
            _ => panic!("not support width"),
        }
    }

    pub fn encode(&self, buf: &mut impl CodeSink){
        match self {
            Imm::Imm8(imm) => buf.putb(*imm),
            Imm::Imm16(imm) => buf.putw(*imm),
            Imm::Imm32(imm) => buf.putd(*imm),
            Imm::Imm64(imm) => buf.putq(*imm),
        }
    }
}

impl std::fmt::Display for Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Imm::Imm8(v) => write!(f, "byte {}", v),
            Imm::Imm16(v) => write!(f, "word {}", v),
            Imm::Imm32(v) => write!(f, "dword {}", v),
            Imm::Imm64(v) => write!(f, "qword {}", v),
        }
    }
}

macro_rules! imm_n {
    ($name: ident, $t: ty, $f:ident) => {
        pub struct $name {
            val: $t,
        }

        impl $name {
            #[inline]
            pub fn new(val: $t) -> Self {
                Self { val }
            }

            pub fn value(&self) -> $t {
                self.val
            }

            pub fn encode(&self, buf: &mut impl CodeSink) {
                buf.$f(self.val)
            }
        }
    };
}

imm_n!(Imm8, u8, putb);
imm_n!(Imm16, u16, putw);
imm_n!(Imm32, u32, putd);
imm_n!(Imm64, u64, putq);

impl TryFrom<Imm> for Imm8 {
    type Error = String;
    fn try_from(value: Imm) -> Result<Self, Self::Error> {
        match value {
            Imm::Imm8(val) => Ok(Self::new(val)),
            _ => Err("Unmatched imm type".into()),
        }
    }
}

impl TryFrom<Imm> for Imm16 {
    type Error = String;
    fn try_from(value: Imm) -> Result<Self, Self::Error> {
        match value {
            Imm::Imm8(val) => Ok(Self::new(val as u16)),
            Imm::Imm16(val) => Ok(Self::new(val)),
            _ => Err("Unmatched imm type".into()),
        }
    }
}

impl TryFrom<Imm> for Imm32 {
    type Error = String;
    fn try_from(value: Imm) -> Result<Self, Self::Error> {
        match value {
            Imm::Imm8(val) => Ok(Self::new(val as u32)),
            Imm::Imm16(val) => Ok(Self::new(val as u32)),
            Imm::Imm32(val) => Ok(Self::new(val)),
            _ => Err("Unmatched imm type".into()),
        }
    }
}

impl TryFrom<Imm> for Imm64 {
    type Error = String;
    fn try_from(value: Imm) -> Result<Self, Self::Error> {
        match value {
            Imm::Imm8(val) => Ok(Self::new(val as u64)),
            Imm::Imm16(val) => Ok(Self::new(val as u64)),
            Imm::Imm32(val) => Ok(Self::new(val as u64)),
            Imm::Imm64(val) => Ok(Self::new(val)),
        }
    }
}
