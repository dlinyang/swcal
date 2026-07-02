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
    ZeroOprand,
    Reg,
    Imm,
    RM,
    // ModRM.Mode
    Imm2RM,
    Reg2RM,
    RM2Reg,
    // tac
    RmOpImm2Reg,
}

pub struct InstFormat {
    pub mnemonic: String,
    pub prefix: Prefix,
    pub opcode: Opcode,
    pub ext_opcode: Option<u8>,
    pub operand_size: u8,
    pub operand_kind: OperandKind,
}

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
}
