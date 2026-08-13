pub mod base;
pub mod encode;
pub mod rex;
pub mod vex;
pub mod operand;
pub mod reg;
pub mod gpr;
pub mod mem;
pub mod imm;
pub mod disp;
pub mod modrm;
pub mod rel;

use operand::*;

#[derive(Debug)]
pub struct AsmInst {
    pub mnemonic: String,
    pub dst: Option<Operand>,
    pub src: Option<Operand>,
    pub src_ext: Option<Operand>,
}

impl AsmInst {
    pub fn name(&self) -> String {
        format!("{}", self.mnemonic)
    }
}

#[inline]
pub fn operand_is_rm(operand_opt: &Option<Operand>) -> bool {
    operand_opt.is_some_and(|x| x.is_rm())
}

#[inline]
pub fn operand_is_reg(operand_opt: &Option<Operand>) -> bool {
    operand_opt.is_some_and(|x| x.is_reg())
}

#[inline]
pub fn operand_is_imm(operand_opt: &Option<Operand>) -> bool {
    operand_opt.is_some_and(|x| x.is_imm())
}

impl std::fmt::Display for AsmInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mnemonic)?;
        if let Some(dst) = self.dst {
            write!(f, " {}", dst)?;
        }
        if let Some(src) = self.src {
            write!(f, ", {}", src)?;
        }
        if let Some(src_ext) = self.src_ext {
            write!(f, ", {}", src_ext)?;
        }
        Ok(())
    }
}
