use crate::inst::base::RegEnc;
use crate::inst::base::width_as_str;
// use crate::inst::encode::Encode;
use crate::inst::gpr::FixedGpr;
use crate::inst::gpr::Gpr;

use super::reg::*;
use super::mem::*;
use super::imm::*;
use super::rel::*;

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Reg(Reg),
    Mem {
        width: u16,
        mem: Mem,
    },
    Imm(Imm),
    /// Label is a placeholder for address
    Label,
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::Mem{width, mem} => write!(f, "{} {}", width_as_str(*width), mem),
            Operand::Imm(imm) => write!(f, "{}", imm),
            Operand::Label => write!(f, "label:"),
        }
    }
}

impl Operand {
    pub fn width(&self) -> u16 {
        match self {
            Operand::Reg(reg) => reg.width(),
            Operand::Mem{width,..} => *width,
            Operand::Imm(imm) => imm.width(),
            // label may abs label: mov rax, label
            // or rel label addr: jmp label
            Operand::Label => 64,
        }
    }

    pub fn is_rm(&self) -> bool {
        match self {
            Operand::Mem{..} | Operand::Reg(_) => true,
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
            Operand::Mem{width: _, mem} => {
                let mem = match mem {
                    Mem::Mem { reg, disp } => RM::Mem { reg: R::from_id(reg.id()), disp: disp.clone() },
                    Mem::Index { base, index, scale, disp } => {
                        RM::Index {
                            base: R::from_id(base.id()),
                            index: R::from_id(index.id()),
                            scale: *scale,
                            disp: disp.clone()
                        }
                    },
                    Mem::RIPDisp { disp32 } => RM::RIPDisp(*disp32),
                };
                Ok(mem)
            },
            _ => Err("Not Rm".into()),
        }
    }
}

macro_rules! impl_try_into_imm {
    ($imm: ty) => {
        impl TryFrom<Operand> for $imm {
            type Error = String;
            fn try_from(value: Operand) -> Result<Self, Self::Error> {
                match value {
                    Operand::Imm(imm) => Self::try_from(imm),
                    Operand::Label => Ok(Self::new(0)),
                    _ => Err("Not IMM".into())
                }
            }
        }

    };
}

impl_try_into_imm!(Imm8);
impl_try_into_imm!(Imm16);
impl_try_into_imm!(Imm32);
impl_try_into_imm!(Imm64);

macro_rules! impl_try_into_rel {
    ($rel: ty) => {
        impl TryFrom<Operand> for $rel {
            type Error = String;
            fn try_from(value: Operand) -> Result<Self, Self::Error> {
                match value {
                    Operand::Imm(imm) => Self::try_from(imm),
                    Operand::Label => Ok(Self::new(0)),
                    _ => Err("Not Rel".into())
                }
            }
        }
    };
}

impl_try_into_rel!(Rel8);
impl_try_into_rel!(Rel16);
impl_try_into_rel!(Rel32);

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
