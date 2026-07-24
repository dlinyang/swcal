use crate::inst::{base::RegEnc, disp::Disp, mem::RM};

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

pub fn encode_modrm<R: RegEnc, M: RegEnc>(reg: &R, rm: &RM<M>, buf: &mut impl CodeSink) {
    let reg = reg.encode();
    match rm {
        RM::Reg(rm) => {
            let rm  = rm.encode();
            buf.putb(gen_modrm(ModRMMode::Reg as u8, reg, rm));
        },
        RM::Mem { reg: rm, disp } => {
            let rm = rm.encode();
            match disp {
                Some(Disp::Disp8(disp)) => {
                    buf.putb(gen_modrm(ModRMMode::Disp8 as u8, reg, rm));
                    buf.putb(*disp as u8);
                },
                Some(Disp::Disp32(disp)) =>{
                    buf.putb(gen_modrm(ModRMMode::Disp32 as u8, reg, rm));
                    buf.putd(*disp as u32);
                },
                None => {
                    buf.putb(gen_modrm(ModRMMode::Mem as u8, reg, rm));
                }
            }
        },
        RM::Index { base, index, scale, disp } => {
            let rm = 0b100;
            match disp {
                Some(Disp::Disp8(disp)) => {
                    buf.putb(gen_modrm(ModRMMode::Disp8 as u8, reg, rm));
                    buf.putb(gen_sib(base.encode(), index.encode(), *scale));
                    buf.putb(*disp as u8);
                },
                Some(Disp::Disp32(disp)) =>{
                    buf.putb(gen_modrm(ModRMMode::Disp32 as u8, reg, rm));
                    buf.putb(gen_sib(base.encode(), index.encode(), *scale));
                    buf.putd(*disp as u32);
                },
                None => {
                    buf.putb(gen_modrm(ModRMMode::Mem as u8, reg, rm));
                    buf.putb(gen_sib(base.encode(), index.encode(), *scale));
                }
            }
        },
        RM::RIPDisp(disp) => {
            buf.putb(gen_modrm(ModRMMode::Mem as u8, reg, 0b101));
            buf.putd(*disp as u32);
        },
    }
}
