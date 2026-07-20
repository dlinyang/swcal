use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn mov() -> Vec<InstFormat> {
    // MOV instruction forms covering register, memory, immediate, and accumulator variants
    // All standard MOV opcodes and operand types for 16/32/64-bit modes
    vec![
        // MOV r/m8, imm8 - C6 /0 ib
        instf!("mov", Legacy, opcode!(0xC6), digit(0), rm(Gpr, u(8), RW), imm_u(8)),
        // MOV r/m16, imm16 - C7 /0 iw  (16-bit)
        instf!("mov", Legacy, opcode!(0xC6), digit(0), rm(Gpr, u(16), RW), imm_u(16)),
        // MOV r/m32, imm32 - C7 /0 id
        instf!("mov", Legacy, opcode!(0xC6), digit(0), rm(Gpr, u(32), RW), imm_u(32)),
        // MOV r/m64, imm32 - C7 /0 id (sign-extended to 64-bit)
        instf!("mov", Legacy, opcode!(0xC6), digit(0), rm(Gpr, i(64), RW), imm_i(32)),

        // MOV r18, imm8  - B0+ ib
        instf!("mov", Legacy, opcode!(0xB0), modrm_r(), reg(Gpr, u(8), RW), imm_u(8)),
        // MOV r16, imm16 - B8+ iw
        instf!("mov", Legacy, opcode!(0xB8), modrm_r(), reg(Gpr, u(16), RW), imm_u(16)),
        // MOV r32, imm32 - B8+ id
        instf!("mov", Legacy, opcode!(0xB8), modrm_r(), reg(Gpr, u(32), RW), imm_u(32)),
        // MOV r64, imm64 - B8+ io (REX.W)  (full 64-bit immediate)
        instf!("mov", Legacy, opcode!(0xB8), modrm_r(), reg(Gpr, i(64), RW), imm_i(32)),

        // MOV r/m8, r8 - 88 /r
        instf!("mov", Legacy, opcode!(0x88), modrm(), rm(Gpr, u(8), RW), reg(Gpr, u(8), R)),
        // MOV r/m16, r16 - 89 /r
        instf!("mov", Legacy, opcode!(0x89), modrm(), rm(Gpr, u(16), RW), reg(Gpr, u(16), R)),
        // MOV r/m32, r32 - 89 /r
        instf!("mov", Legacy, opcode!(0x89), modrm(), rm(Gpr, u(32), RW), reg(Gpr, u(32), R)),
        // MOV r/m64, r64 - 89 /r (REX.W)
        instf!("mov", Legacy, opcode!(0x89), modrm(), rm(Gpr, u(64), RW), reg(Gpr, u(64), R)),

        // MOV r8, r/m8 - 8A /r
        instf!("mov", Legacy, opcode!(0x8A), modrm(), reg(Gpr, u(8), RW), rm(Gpr, u(8), R)),
        // MOV r16, r/m16 - 8B /r
        instf!("mov", Legacy, opcode!(0x8B), modrm(), reg(Gpr, u(16), RW), rm(Gpr, u(16), R)),
        // MOV r32, r/m32 - 8B /r
        instf!("mov", Legacy, opcode!(0x8B), modrm(), reg(Gpr, u(32), RW), rm(Gpr, u(32), R)),
        // MOV r64, r/m64 - 8B /r (REX.W)
        instf!("mov", Legacy, opcode!(0x8B), modrm(), reg(Gpr, u(64), RW), rm(Gpr, u(64), R)),

        // // MOV AL, m8 (memory direct) - A0 (accumulator load)
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA0), EncodeKind::DirectAddr, 8, OperandKind::Acc2Mem),
        // // MOV AX, m16 - A1
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA1), EncodeKind::DirectAddr, 16, OperandKind::Acc2Mem),
        // // MOV EAX, m32 - A1
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA1), EncodeKind::DirectAddr, 32, OperandKind::Acc2Mem),
        // // MOV RAX, m64 - A1 (REX.W)
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA1), EncodeKind::DirectAddr, 64, OperandKind::Acc2Mem),

        // // MOV m8, AL - A2 (accumulator store)
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA2), EncodeKind::DirectAddr, 8, OperandKind::Mem2Acc),
        // // MOV m16, AX - A3
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA3), EncodeKind::DirectAddr, 16, OperandKind::Mem2Acc),
        // // MOV m32, EAX - A3
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA3), EncodeKind::DirectAddr, 32, OperandKind::Mem2Acc),
        // // MOV m64, RAX - A3 (REX.W)
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xA3), EncodeKind::DirectAddr, 64, OperandKind::Mem2Acc),

        // // MOV r/m16, sreg - 8C /r (segment register to r/m)
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0x8C), EncodeKind::ModRM, 16, OperandKind::Reg2RM),
        // // MOV sreg, r/m16 - 8E /r (r/m to segment register)
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0x8E), EncodeKind::ModRM, 16, OperandKind::RM2Reg),
    ]
}
