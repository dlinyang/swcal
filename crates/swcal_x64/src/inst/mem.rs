use super::reg::*;
use super::disp::*;

/// width \[reg + (index * scale)? + (disp)?\]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mem {
    //width-byte
    pub width: u8,
    pub reg: Reg,
    pub sib_opt: Option<(Reg, u8)>,
    pub disp_opt: Option<Disp>,
}

impl Mem {
    pub fn check_rex(&self) -> bool {
        todo!()
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
            _ => write!(f, "{}bit", self.width * 8)?,
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
