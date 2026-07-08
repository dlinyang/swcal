use super::super::format::*;
pub fn xchg() -> Vec<InstFormat> {
    vec![
        xchg_reg_reg_n(16),
        xchg_reg_reg_n(32),
        xchg_reg_reg_n(64),
    ]
}

fn xchg_reg_reg_n(size: u8) -> InstFormat {
    InstFormat {
        mnemonic: String::from("xchg"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0x90,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::RegInOpcode(0),
        operand_size: size,
        operand_kind: OperandKind::RM2Reg,
    }
}
