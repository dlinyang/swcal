use super::super::format::*;

pub fn mov() -> Vec<InstFormat> {
    vec![
        mov_rm_imm32(),
        mov_rm_imm64(),
        mov_reg_imm32(),
        mov_reg_imm64(),
        mov_reg_rm64(),
        mov_rm64_reg(),
    ]
}


// 创建 MOV 指令的 InstFormat 实例

/// 创建一个 MOV r/m64, imm32 形式的 MOV 指令 (C7 /0)
pub fn mov_rm_imm32() -> InstFormat {
    InstFormat {
        mnemonic: String::from("mov"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0xC7,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::Digit(0),
        operand_size: 32,
        operand_kind: OperandKind::Imm2RM,
    }
}

/// 创建一个 MOV r64, imm64 形式的 MOV 指令 (B8+ rd io)
pub fn mov_reg_imm64() -> InstFormat {
    InstFormat {
        mnemonic: String::from("mov"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0xB8,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::RegEncOpcode,
        operand_size: 64,
        operand_kind: OperandKind::Imm2RM,
    }
}

/// 创建一个 MOV r64, r/m64 形式的 MOV 指令 (8B /r)
pub fn mov_reg_rm64() -> InstFormat {
    InstFormat {
        mnemonic: String::from("mov"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0x8B,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::ModRM,
        operand_size: 64,
        operand_kind: OperandKind::RM2Reg,
    }
}

/// 创建一个 MOV r/m64, r64 形式的 MOV 指令 (89 /r)
pub fn mov_rm64_reg() -> InstFormat {
    InstFormat {
        mnemonic: String::from("mov"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0x89,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::ModRM,
        operand_size: 64,
        operand_kind: OperandKind::Reg2RM,
    }
}

/// 创建一个 MOV r/m64, imm64 形式的 MOV 指令 (with REX.W prefix)
/// 注意：x86-64 中 C7 /0 只能编码 imm32，如果需要 imm64 需要配合 REX.W 使用
pub fn mov_rm_imm64() -> InstFormat {
    InstFormat {
        mnemonic: String::from("mov"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0xC7,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::Digit(0),
        operand_size: 64,
        operand_kind: OperandKind::Imm2RM,
    }
}

/// 创建一个 MOV r32, imm32 形式的 MOV 指令 (B8+ id)
pub fn mov_reg_imm32() -> InstFormat {
    InstFormat {
        mnemonic: String::from("mov"),
        prefix: Prefix::Legacy,
        opcode: Opcode {
            fst: 0xB8 ,
            snd: None,
            trd: None,
        },
        encode_kind: EncodeKind::RegEncOpcode,
        operand_size: 32,
        operand_kind: OperandKind::Imm2RM,
    }
}
