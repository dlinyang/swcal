use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn sub() -> Vec<InstFormat> {
    // SUB instruction forms covering register, memory, and immediate variants
    // All standard SUB opcodes and operand types for 16/32/64-bit modes
    vec![
        // SUB r/m8, imm8 - 80 /5 ib
        instf!("sub", Legacy, opcode!(0x80), digit(5), rm(Gpr, u(8), RW), imm_u(8)),
        // SUB r/m16, imm16 - 81 /5 iw
        instf!("sub", Legacy, opcode!(0x81), digit(5), rm(Gpr, u(16), RW), imm_u(16)),
        // SUB r/m32, imm32 - 81 /5 id
        instf!("sub", Legacy, opcode!(0x81), digit(5), rm(Gpr, u(32), RW), imm_u(32)),
        // SUB r/m64, imm32 - 81 /5 id (sign-extended to 64-bit)
        instf!("sub", Legacy, opcode!(0x81), digit(5), rm(Gpr, i(64), RW), imm_i(32)),

        // SUB r/m16, imm8 - 83 /5 ib (sign-extended)
        instf!("sub", Legacy, opcode!(0x83), digit(5), rm(Gpr, i(16), RW), imm_i(8)),
        // SUB r/m32, imm8 - 83 /5 ib (sign-extended)
        instf!("sub", Legacy, opcode!(0x83), digit(5), rm(Gpr, i(32), RW), imm_i(8)),
        // SUB r/m64, imm8 - 83 /5 ib (sign-extended)
        instf!("sub", Legacy, opcode!(0x83), digit(5), rm(Gpr, i(64), RW), imm_i(8)),

        // SUB r/m8, r8 - 28 /r
        instf!("sub", Legacy, opcode!(0x28), modrm(), rm(Gpr, u(8), RW), reg(Gpr, u(8), R)),
        // SUB r/m16, r16 - 29 /r
        instf!("sub", Legacy, opcode!(0x29), modrm(), rm(Gpr, u(16), RW), reg(Gpr, u(16), R)),
        // SUB r/m32, r32 - 29 /r
        instf!("sub", Legacy, opcode!(0x29), modrm(), rm(Gpr, u(32), RW), reg(Gpr, u(32), R)),
        // SUB r/m64, r64 - 29 /r (REX.W)
        instf!("sub", Legacy, opcode!(0x29), modrm(), rm(Gpr, u(64), RW), reg(Gpr, u(64), R)),

        // SUB r8, r/m8 - 2A /r
        instf!("sub", Legacy, opcode!(0x2A), modrm(), reg(Gpr, u(8), RW), rm(Gpr, u(8), R)),
        // SUB r16, r/m16 - 2B /r
        instf!("sub", Legacy, opcode!(0x2B), modrm(), reg(Gpr, u(16), RW), rm(Gpr, u(16), R)),
        // SUB r32, r/m32 - 2B /r
        instf!("sub", Legacy, opcode!(0x2B), modrm(), reg(Gpr, u(32), RW), rm(Gpr, u(32), R)),
        // SUB r64, r/m64 - 2B /r (REX.W)
        instf!("sub", Legacy, opcode!(0x2B), modrm(), reg(Gpr, u(64), RW), rm(Gpr, u(64), R)),

        // // SUB AL, imm8 - 2C ib (accumulator with immediate)
        // InstFormat::new("sub", Legacy, opcode!(0x2C), EncodeKind::ModRM, 1, OperandKind::ImmAcc),
        // // SUB AX, imm16 - 2D iw
        // InstFormat::new("sub", Legacy, opcode!(0x2D), EncodeKind::ModRM, 2, OperandKind::ImmAcc),
        // // SUB EAX, imm32 - 2D id
        // InstFormat::new("sub", Legacy, opcode!(0x2D), EncodeKind::ModRM, 4, OperandKind::ImmAcc),
        // // SUB RAX, imm32 - 2D id (sign-extended to 64-bit, REX.W)
        // InstFormat::new("sub", Legacy, opcode!(0x2D), EncodeKind::ModRM, 4, OperandKind::ImmAcc),
    ]
}
