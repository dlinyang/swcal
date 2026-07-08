use crate::inst::format::*;

pub fn add() -> Vec<InstFormat> {
    vec![
        _rm2reg8(),
        _rm2reg_n(16),
        _rm2reg_n(32),
        _rm2reg_n(64),
    ]
}

fn _rm2reg8() -> InstFormat {
    InstFormat {
        mnemonic: String::from("add"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0x22,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::ModRM,
        operand_size: 8,
        operand_kind: OperandKind::RM2Reg,
    }
}

fn _rm2reg_n(size: u8) -> InstFormat {
    InstFormat {
        mnemonic: String::from("add"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0x23,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::ModRM,
        operand_size: size,
        operand_kind: OperandKind::RM2Reg,
    }
}
