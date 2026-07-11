use crate::format::*;

pub fn inc() -> Vec<InstFormat> {
    vec![
        _rm2reg8(),
        _rm2reg_n(16),
        _rm2reg_n(32),
        _rm2reg_n(64),
    ]
}

fn _rm2reg8() -> InstFormat {
    InstFormat {
        mnemonic: String::from("inc"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0xfe,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::RegExtOp(0),
        operand_size: 8,
        operand_kind: OperandKind::RM2Reg,
    }
}

fn _rm2reg_n(size: u8) -> InstFormat {
    InstFormat {
        mnemonic: String::from("ff"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0x23,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::RegExtOp(0),
        operand_size: size,
        operand_kind: OperandKind::RM,
    }
}
