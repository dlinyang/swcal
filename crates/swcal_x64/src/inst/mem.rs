use crate::inst::base::RegEnc;

use super::reg::*;
use super::disp::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mem {
    Mem {
        reg: Reg,
        disp: Option<Disp>
    },
    Index{
        base: Reg,
        index: Reg,
        scale: u8,
        disp: Option<Disp>
    },
    RIPDisp{
        disp32: i32
    },
}

impl Mem {

    pub fn rex_b(&self) -> bool {
        match self {
            Mem::Mem {reg, ..} => reg.is_extended(),
            Mem::Index {base, ..} => base.is_extended(),
            Mem::RIPDisp{..} => false,
        }
    }

    pub fn rex_x(&self) -> bool {
        match self {
            Mem::Mem {..} => false,
            Mem::Index {base: _, index, .. } => index.is_extended(),
            Mem::RIPDisp{..} => false,
        }
    }
}

impl std::fmt::Display for Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mem::Mem { reg, disp } => {
                let disp = disp.map(|x| format!(" + {}", x)).unwrap_or_default();
                write!(f, "[{}{}]", reg, disp)
            },
            Mem::Index { base, index, scale, disp } => {
                let disp = disp.map(|x| format!("+{}", x)).unwrap_or_default();
                write!(f,"[{} + {} * {}{}]", base, index, scale, disp)
            },
            Mem::RIPDisp { disp32 } => {
                write!(f, "[{}]", disp32)
            },
        }
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
