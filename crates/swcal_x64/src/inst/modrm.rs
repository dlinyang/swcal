use super::encode::*;

#[inline]
pub fn gen_modrm(mode: u8, reg: u8, rm: u8) -> u8 {
    ((mode & 0b11) << 6) | ((reg & 0b111) << 3) | rm & 0b111
}

#[inline]
pub fn gen_sib(base: u8, index: u8, scale: u8) -> u8 {
    (scale & 0b11) << 6 | (index & 0b111) << 3 | base & 0b111
}

/// ModRM.mod field encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModRMMode {
    /// [register] - direct memory, no displacement
    Mem = 0b00,
    /// [register + disp8] - memory with 8-bit displacement
    Disp8 = 0b01,
    /// [register + disp32] - memory with 32-bit displacement
    Disp32 = 0b10,
    /// register (not memory)
    Reg = 0b11,
}

pub struct ModRM {
    pub mode: u8,
    pub reg: u8,
    pub rm: u8,
}

impl ModRM {
    pub fn new() -> Self {
        Self {
            mode: 0,
            reg: 0,
            rm: 0,
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        ModRM {
            mode: (byte >> 6) & 0b11,
            reg: (byte >> 3) & 0b111,
            rm: byte & 0b111,
        }
    }

    pub fn byte(&self) -> u8 {
        (self.mode << 6) | (self.reg << 3) | self.rm
    }

    pub fn encode(&self, buf: &mut impl CodeSink) {
        buf.putb(self.byte());
    }
}
