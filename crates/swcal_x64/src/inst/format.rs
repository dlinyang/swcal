use std::fmt;

pub enum  Prefix {
    Legacy,
    EvexVex,
}

pub struct Opcode {
    pub fst: u8,
    pub snd: Option<u8>,
    pub trd: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandKind {
    NoOperand,
    // Reg,
    // Imm,
    RM,
    //
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

#[derive(Debug, PartialEq, Eq)]
pub enum EncodeKind {
    ModRM,
    Digit(u8),
    RegEncOpcode,
    RegInOpcode(u8),
}

impl Default for EncodeKind {
    fn default() -> Self {
        EncodeKind::ModRM
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

#[derive(Debug)]
pub struct BinInst {
    pub len: u8,
    pub data: [u8; 15],
}

impl BinInst {
    pub fn new() -> Self {
        Self {
            len: 0,
            data: [0u8;15],
        }
    }

    pub fn push(&mut self, byte: u8) {
        if (self.len as usize) < self.data.len() {
            self.data[self.len as usize] = byte;
            self.len += 1;
        } else {
            panic!("BinInst buffer overflow: cannot push more than {} bytes", self.data.len());
        }
    }
}

impl fmt::Display for BinInst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BinInst {{ len: {}, data: [", self.len)?;
        for i in 0..self.len as usize {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:#04x}", self.data[i])?;
        }
        write!(f, "] }}")
    }
}
