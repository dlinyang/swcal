use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn bool_arith() -> Vec<InstFormat> {
    vec![
    // AND instruction forms covering register, memory, and immediate variants
    // All standard AND opcodes and operand types for 16/32/64-bit modes
        // AND AL, imm8 - 24 ib (accumulator)
        instf!("and", Legacy, opcode!(0x24), no_modrm(), reg(Fgr(0), u(8), W), imm_u(8)),
        // AND AX, imm16 - 25 iw
        instf!("and", Legacy, opcode!(0x25), no_modrm(), reg(Fgr(0), u(16), W), imm_u(16)),
        // AND EAX, imm32 - 25 id
        instf!("and", Legacy, opcode!(0x25), no_modrm(), reg(Fgr(0), u(32), W), imm_u(32)),
        // AND RAX, imm32 - 25 id (REX.W)
        instf!("and", Legacy, opcode!(0x25), no_modrm(), reg(Fgr(0), u(64), W), imm_i(32)),

        // AND r/m8, imm8 - 80 /4 ib
        instf!("and", Legacy, opcode!(0x80), digit(4), rm(Gpr, u(8), W), imm_u(8)),
        // AND r/m16, imm16 - 81 /4 iw
        instf!("and", Legacy, opcode!(0x81), digit(4), rm(Gpr, u(16), W), imm_u(16)),
        // AND r/m32, imm32 - 81 /4 id
        instf!("and", Legacy, opcode!(0x81), digit(4), rm(Gpr, u(32), W), imm_u(32)),
        // AND r/m64, imm32 - 81 /4 id (sign-extended to 64-bit)
        instf!("and", Legacy, opcode!(0x81), digit(4), rm(Gpr, i(64), W), imm_i(32)),

        // AND r/m16, imm8 (short form) - 83 /4 ib
        instf!("and", Legacy, opcode!(0x83), digit(4), rm(Gpr, i(16), W), imm_i(8)),
        // AND r/m32, imm8 (short form) - 83 /4 ib
        instf!("and", Legacy, opcode!(0x83), digit(4), rm(Gpr, i(32), W), imm_i(8)),
        // AND r/m64, imm8 (short form) - 83 /4 ib
        instf!("and", Legacy, opcode!(0x83), digit(4), rm(Gpr, i(64), W), imm_i(8)),

        // AND r/m8, r8 - 20 /r
        instf!("and", Legacy, opcode!(0x20), modrm(), rm(Gpr, u(8), W), reg(Gpr, u(8), R)),
        // AND r/m16, r16 - 21 /r
        instf!("and", Legacy, opcode!(0x21), modrm(), rm(Gpr, u(16), W), reg(Gpr, u(16), R)),
        // AND r/m32, r32 - 21 /r
        instf!("and", Legacy, opcode!(0x21), modrm(), rm(Gpr, u(32), W), reg(Gpr, u(32), R)),
        // AND r/m64, r64 - 21 /r (REX.W)
        instf!("and", Legacy, opcode!(0x21), modrm(), rm(Gpr, u(64), W), reg(Gpr, u(64), R)),

        // AND r8, r/m8 - 22 /r
        instf!("and", Legacy, opcode!(0x22), modrm(), reg(Gpr, u(8), W), rm(Gpr, u(8), R)),
        // AND r16, r/m16 - 23 /r
        instf!("and", Legacy, opcode!(0x23), modrm(), reg(Gpr, u(16), W), rm(Gpr, u(16), R)),
        // AND r32, r/m32 - 23 /r
        instf!("and", Legacy, opcode!(0x23), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(32), R)),
        // AND r64, r/m64 - 23 /r (REX.W)
        instf!("and", Legacy, opcode!(0x23), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(64), R)),


    // OR instruction forms covering register, memory, and immediate variants
    // All standard OR opcodes and operand types for 16/32/64-bit modes
        // OR AL, imm8 - 0C ib (accumulator)
        instf!("or", Legacy, opcode!(0x0C), no_modrm(), reg(Fgr(0), u(8), W), imm_u(8)),
        // OR AX, imm16 - 0D iw
        instf!("or", Legacy, opcode!(0x0D), no_modrm(), reg(Fgr(0), u(16), W), imm_u(16)),
        // OR EAX, imm32 - 0D id
        instf!("or", Legacy, opcode!(0x0D), no_modrm(), reg(Fgr(0), u(32), W), imm_u(32)),
        // OR RAX, imm32 - 0D id (REX.W)
        instf!("or", Legacy, opcode!(0x0D), no_modrm(), reg(Fgr(0), u(64), W), imm_i(32)),

        // OR r/m8, imm8 - 80 /1 ib
        instf!("or", Legacy, opcode!(0x80), digit(1), rm(Gpr, u(8), W), imm_u(8)),
        // OR r/m16, imm16 - 81 /1 iw
        instf!("or", Legacy, opcode!(0x81), digit(1), rm(Gpr, u(16), W), imm_u(16)),
        // OR r/m32, imm32 - 81 /1 id
        instf!("or", Legacy, opcode!(0x81), digit(1), rm(Gpr, u(32), W), imm_u(32)),
        // OR r/m64, imm32 - 81 /1 id (sign-extended to 64-bit)
        instf!("or", Legacy, opcode!(0x81), digit(1), rm(Gpr, i(64), W), imm_i(32)),

        // OR r/m16, imm8 (short form) - 83 /1 ib
        instf!("or", Legacy, opcode!(0x83), digit(1), rm(Gpr, i(16), W), imm_i(8)),
        // OR r/m32, imm8 (short form) - 83 /1 ib
        instf!("or", Legacy, opcode!(0x83), digit(1), rm(Gpr, i(32), W), imm_i(8)),
        // OR r/m64, imm8 (short form) - 83 /1 ib
        instf!("or", Legacy, opcode!(0x83), digit(1), rm(Gpr, i(64), W), imm_i(8)),

        // OR r/m8, r8 - 08 /r
        instf!("or", Legacy, opcode!(0x08), modrm(), rm(Gpr, u(8), W), reg(Gpr, u(8), R)),
        // OR r/m16, r16 - 09 /r
        instf!("or", Legacy, opcode!(0x09), modrm(), rm(Gpr, u(16), W), reg(Gpr, u(16), R)),
        // OR r/m32, r32 - 09 /r
        instf!("or", Legacy, opcode!(0x09), modrm(), rm(Gpr, u(32), W), reg(Gpr, u(32), R)),
        // OR r/m64, r64 - 09 /r (REX.W)
        instf!("or", Legacy, opcode!(0x09), modrm(), rm(Gpr, u(64), W), reg(Gpr, u(64), R)),

        // OR r8, r/m8 - 0A /r
        instf!("or", Legacy, opcode!(0x0A), modrm(), reg(Gpr, u(8), W), rm(Gpr, u(8), R)),
        // OR r16, r/m16 - 0B /r
        instf!("or", Legacy, opcode!(0x0B), modrm(), reg(Gpr, u(16), W), rm(Gpr, u(16), R)),
        // OR r32, r/m32 - 0B /r
        instf!("or", Legacy, opcode!(0x0B), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(32), R)),
        // OR r64, r/m64 - 0B /r (REX.W)
        instf!("or", Legacy, opcode!(0x0B), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(64), R)),


    // NOT instruction forms - bitwise NOT (one's complement)
    // All standard NOT opcodes and operand types for 8/16/32/64-bit modes
        // NOT r/m8 - F6 /2
        instf!("not", Legacy, opcode!(0xF6), digit(2), rm(Gpr, u(8), RW)),
        // NOT r/m16 - F7 /2
        instf!("not", Legacy, opcode!(0xF7), digit(2), rm(Gpr, u(16), RW)),
        // NOT r/m32 - F7 /2
        instf!("not", Legacy, opcode!(0xF7), digit(2), rm(Gpr, u(32), RW)),
        // NOT r/m64 - F7 /2 (REX.W)
        instf!("not", Legacy, opcode!(0xF7), digit(2), rm(Gpr, u(64), RW)),
    ]
}
