use crate::inst::base::RegEnc;

use super::reg::*;
use super::disp::*;

/// width \[reg + (index * scale)? + (disp)?\]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mem {
    pub width: u16,
    pub reg: Reg,
    pub sib_opt: Option<(Reg, u8)>,
    pub disp_opt: Option<Disp>,
}

impl Mem {

    pub fn rex_b(&self) -> bool {
        self.reg.is_extended()
    }

    pub fn rex_x(&self) -> bool {
        self.sib_opt.is_some_and(|(r,_)| r.is_extended())
    }

    pub fn check_reg_valid(&self) -> bool {
        if let Some((index, _)) = self.sib_opt {
            index.is_extended() == self.reg.is_extended()
        } else {
            true
        }
    }
}

impl std::fmt::Display for Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.width {
            1 => write!(f, "byte")?,
            2 => write!(f, "word")?,
            4 => write!(f, "dword")?,
            8 => write!(f, "qword")?,
            _ => write!(f, "{}bit", self.width)?,
        }
        write!(f, " [")?;
        write!(f, "{}", self.reg)?;
        if let Some((index, scale)) = self.sib_opt {
            write!(f, "+{index}*{scale}")?;
        }
        if let Some(disp) = self.disp_opt {
            write!(f, "+{disp}")?;
        }
        write!(f, "]")
    }
}

pub enum RM<R: RegEnc> {
    Reg(R),
    Mem {
        reg: R,
        disp: Option<Disp>
    },
    Index{
        base: R,
        index: R,
        scale: u8,
        disp: Option<Disp>
    },
    RIPDisp(i32),
}

impl<R: RegEnc + Copy> RM<R> {
    pub fn rex_b(&self) -> bool {
        match self {
            RM::Reg(reg) => reg.is_extend(),
            RM::Mem { reg, ..} => reg.is_extend(),
            RM::Index { base, ..} => base.is_extend(),
            RM::RIPDisp(_) => false,
        }
    }

    pub fn rex_x(&self) -> bool {
        match self {
            RM::Reg(_) => false,
            RM::Mem {..} => false,
            RM::Index { base: _base, index, .. } => index.is_extend(),
            RM::RIPDisp(_) => false,
        }
    }
}
