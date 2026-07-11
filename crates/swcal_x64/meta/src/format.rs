use std::fmt;

#[derive(Debug)]
pub enum Prefix {
    Legacy,
    EvexVex,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Legacy => write!(f, "Legacy"),
            Prefix::EvexVex => write!(f, "EvexVex"),
        }
    }
}

#[derive(Debug)]
pub struct Opcode {
    pub fst: u8,
    pub snd: Option<u8>,
    pub trd: Option<u8>,
}

#[macro_export]
macro_rules! opcode {
    ($p1:expr) => {
        Opcode {
            fst: $p1,
            snd: None,
            trd: None,
        }
    };
    ($p1:expr, $p2: expr) => {
        Opcode {
            fst: $p1,
            snd: Some($p2),
            trd: None,
        }
    };
    ($p1:expr, $p2: expr, $p3: expr) => {
        Opcode {
            fst: $p1,
            snd: Some($p2),
            trd: Some($p3),
        }
    };
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.fst)?;
        if let Some(snd) = self.snd {
            write!(f, " {:02x}", snd)?;
        }
        if let Some(trd) = self.trd {
            write!(f, " {:02x}", trd)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandKind {
    NoOperand,
    // Reg,
    // Imm,
    RM,
    // dst = src op dst
    Imm2RM,
    Reg2RM,
    RM2Reg,
    Reg2Reg,
    // tac
    // RMOpImm2Reg,
    // RMOpRM2Reg,
    // RMOpReg2Reg,
}

impl fmt::Display for OperandKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandKind::NoOperand => write!(f, "No"),
            // OperandKind::Reg => write!(f, "r"),
            // OperandKind::Imm => write!(f, "i"),
            OperandKind::RM => write!(f, "rm"),
            // OperandKind::Imm => write!(f, "i2r"),
            OperandKind::Imm2RM => write!(f, "i2rm"),
            OperandKind::Reg2RM => write!(f, "r2rm"),
            OperandKind::RM2Reg => write!(f, "rm2r"),
            OperandKind::Reg2Reg => write!(f, "r2r"),
            // OperandKind::MemOpImm2Reg => write!(f, "mi2r"),
            // OperandKind::RegOpImm2Reg => write!(f, "ri2r"),
            // OperandKind::RegOpReg2Reg => write!(f, "rr2r"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeKind {
    // opcode /r ModRM
    ModRM,
    // IMM,
    // op /n in ModRM
    RegExtOp(u8),
    // op +r
    RegEncOp,
    // op fixed_reg ...
    OpFixedReg(u8),
}

impl Default for EncodeKind {
    fn default() -> Self {
        EncodeKind::ModRM
    }
}

impl fmt::Display for EncodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeKind::ModRM => write!(f, "/r"),
            EncodeKind::RegExtOp(n) => write!(f, "/{}", n),
            EncodeKind::RegEncOp => write!(f, "+r"),
            EncodeKind::OpFixedReg(n) => write!(f, "r{}", n),
        }
    }
}

impl Into<String> for EncodeKind {
    fn into(self) -> String {
        match self {
            EncodeKind::ModRM => "modrm".to_string(),
            EncodeKind::RegExtOp(n) => format!("ext_op_r{}", n),
            EncodeKind::RegEncOp => "enc_r_op".to_string(),
            EncodeKind::OpFixedReg(n) => format!("fixed_r{}", n),
        }
    }
}

pub struct InstFormat {
    pub mnemonic: String,
    pub prefix: Prefix,
    pub opcode: Opcode,
    pub encode_kind: EncodeKind,
    pub operand_size: u8,
    pub operand_kind: OperandKind,
}

impl InstFormat {
    /// Creates a new `InstFormat` with the given parameters.
    pub fn new(
        mnemonic: impl Into<String>,
        prefix: Prefix,
        opcode: Opcode,
        encode_kind: EncodeKind,
        // byte width
        operand_size: u8,
        operand_kind: OperandKind,
    ) -> Self {
        Self {
            mnemonic: mnemonic.into(),
            prefix,
            opcode,
            encode_kind,
            operand_size,
            operand_kind,
        }
    }

    pub fn name(&self) -> String {
        format!(
            "{}_{}_{}_{}",
            self.mnemonic,
            self.operand_kind,
            self.operand_size,
            Into::<String>::into(self.encode_kind)
        )
    }
}

impl fmt::Display for InstFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InstFormat( mnemonic: {}, prefix: {}, opcode: {}, encode_kind: {:?}, operand_size: {}, operand_kind: {} )",
            self.mnemonic,
            self.prefix,
            self.opcode,
            self.encode_kind,
            self.operand_size,
            self.operand_kind
        )
    }
}
