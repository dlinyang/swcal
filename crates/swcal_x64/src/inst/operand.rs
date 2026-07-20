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
    pub fn width(&self) -> u8 {
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
