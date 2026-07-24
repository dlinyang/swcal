use crate::inst::base::RegEnc;
use crate::inst::encode::Encode;
use crate::inst::gpr::FixedGpr;
use crate::inst::gpr::Gpr;

use super::reg::*;
use super::mem::*;
use super::imm::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Reg(Reg),
    Mem(Mem),
    Imm(Imm),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::Mem(rm) => write!(f, "{}", rm),
            Operand::Imm(imm) => write!(f, "{}", imm),
        }
    }
}

impl Operand {
    pub fn width(&self) -> u16 {
        match self {
            Operand::Reg(reg) => reg.width(),
            Operand::Mem(mem) => mem.width,
            Operand::Imm(imm) => imm.width(),
        }
    }

    pub fn is_rm(&self) -> bool {
        match self {
            Operand::Mem(_) | Operand::Reg(_) => true,
            _ => false,
        }
    }

    pub fn is_reg(&self) -> bool {
        match self {
            Operand::Reg(_) => true,
            _ => false,
        }
    }

    pub fn is_imm(&self) -> bool {
        match self {
            Operand::Imm(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<Operand> for Gpr {
    type Error = String;

    fn try_from(value: Operand) -> Result<Gpr, Self::Error> {
        match &value {
            Operand::Reg(reg) => {
                // note: recent only gpr
                Ok(Gpr::from_id(reg.id()))
            },
            _ => Err("Not Reg".into()),
        }
    }
}

// impl TryInto<FixedGpr<I>>

impl<R: RegEnc> TryFrom<Operand> for RM<R> {
    type Error = String;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match &value {
            Operand::Reg(reg) => {
                Ok(RM::Reg(R::from_id(reg.id())))
            },
            Operand::Mem(mem) => {
                todo!()
            },
            _ => Err("Not Rm".into()),
        }
    }
}

macro_rules! impl_try_info_imm {
    ($imm: ty) => {
        impl TryFrom<Operand> for $imm {
            type Error = String;
            fn try_from(value: Operand) -> Result<Self, Self::Error> {
                match value {
                    Operand::Imm(imm) => Self::try_from(imm),
                    _ => Err("Not IMM".into())
                }
            }
        }

    };
}

impl_try_info_imm!(Imm8);
impl_try_info_imm!(Imm16);
impl_try_info_imm!(Imm32);
impl_try_info_imm!(Imm64);

impl<const N: u8> TryFrom<Operand> for FixedGpr<N> {
    type Error = String;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Reg(reg) => {
                if reg.id() == N {
                    Ok(FixedGpr::new())
                } else {
                    Err(format!("Not FixedReg {}", N))
                }
            },
            _ => Err("Not Reg for FixedReg".into())
        }
    }
}
