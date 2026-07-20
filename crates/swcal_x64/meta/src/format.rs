use crate::{generate::*, type_name};

pub mod prefix;
pub use prefix::*;
pub mod opcode;
pub use opcode::*;
pub mod operand;
pub use operand::*;
pub mod modrm;
pub use modrm::*;

pub struct Encode {
    pub modrm: ModRMKind,
    pub operand: OperandEncode,
}

impl std::fmt::Display for Encode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<6} {}", self.modrm, self.operand)
    }
}

pub fn encode(modrm: ModRMKind, operand: OperandEncode) -> Encode {
    Encode { modrm, operand }
}

impl SrcGen for Encode {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        type_name!(self.operand, self.modrm)
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}
pub struct InstFormat {
    pub mnemonic: String,
    pub prefix: Prefix,
    pub opcode: Opcode,
    pub encode: Encode,
}

impl InstFormat {
    /// Creates a new `InstFormat` with the given parameters.
    pub fn new(
        mnemonic: impl Into<String>,
        prefix: Prefix,
        opcode: Opcode,
        encode: Encode,
    ) -> Self {
        Self {
            mnemonic: mnemonic.into(),
            prefix,
            opcode,
            encode,
        }
    }
}

#[macro_export]
macro_rules! instf {
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!()))
    };
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr, $ope: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!($ope)))
    };
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr, $dst: expr, $src: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!($dst, $src)))
    };
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr, $dst: expr, $src: expr, $src_other: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!($dst, $src, $src_other)))
    };
}

impl std::fmt::Display for InstFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<7} {:<8} {:<} {}",
            self.mnemonic,
            self.opcode,
            self.encode,
            self.prefix,
        )
    }
}

impl SrcGen for InstFormat {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        format!(
            "{}{}",
            self.mnemonic.to_uppercase(),
            self.encode.type_name(),
        )
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}

impl Validation for InstFormat {
    fn validation(&self) -> String {
        todo!()
    }
}
