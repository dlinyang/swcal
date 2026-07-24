use crate::generate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ModRMKind {
    /// None Modrm
    None,
    /// normal ModRM encode
    Normal,
    /// extend opcode in ModRM.reg
    Digit(u8),
    /// None ModRM and encode reg in opcode
    Reg,
}

#[inline]
pub fn no_modrm() -> ModRMKind {
    ModRMKind::None
}

#[inline]
pub fn modrm() -> ModRMKind {
    ModRMKind::Normal
}

#[inline]
pub fn digit(op: u8) -> ModRMKind {
    ModRMKind::Digit(op)
}

#[inline]
pub fn modrm_r() -> ModRMKind {
    ModRMKind::Reg
}

impl std::fmt::Display for ModRMKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModRMKind::None => write!(f, ""),
            ModRMKind::Normal => write!(f, "/r"),
            ModRMKind::Digit(op) => write!(f, "/{}", op),
            ModRMKind::Reg => write!(f, "+r"),
        }
    }
}

impl SrcGen for ModRMKind {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        match self {
            ModRMKind::None => "NoModRM".into(),
            ModRMKind::Normal => "ModRM".into(),
            ModRMKind::Digit(op) => format!("Digit{}",op),
            ModRMKind::Reg => "ModRMReg".into(),
        }
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}
