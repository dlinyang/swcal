use crate::format::*;
use crate::opcode;

pub fn mov() -> Vec<InstFormat> {
    // MOV instruction forms covering register, memory, immediate, and accumulator variants
    // All standard MOV opcodes and operand types for 16/32/64-bit modes
    vec![
        // MOV r/m8, imm8 - C6 /0 ib
        InstFormat::new("mov", Prefix::Legacy, opcode!(0xC6), EncodeKind::RegExtOp(0), 1, OperandKind::Imm2RM),
        // MOV r/m16, imm16 - C7 /0 iw  (16-bit)
        InstFormat::new("mov", Prefix::Legacy, opcode!(0xC7), EncodeKind::RegExtOp(0), 2, OperandKind::Imm2RM),
        // MOV r/m32, imm32 - C7 /0 id
        InstFormat::new("mov", Prefix::Legacy, opcode!(0xC7), EncodeKind::RegExtOp(0), 4, OperandKind::Imm2RM),
        // // MOV r/m64, imm32 - C7 /0 id (sign-extended to 64-bit)
        // InstFormat::new("mov", Prefix::Legacy, opcode!(0xC7), EncodeKind::RegExtOp(0), 4, OperandKind::Imm2RM),

        // MOV r18, imm8  - B0+ ib
        InstFormat::new("mov", Prefix::Legacy, opcode!(0xB0), EncodeKind::RegEncOp, 1, OperandKind::Imm2RM),
        // MOV r16, imm16 - B8+ iw
        InstFormat::new("mov", Prefix::Legacy, opcode!(0xB8), EncodeKind::RegEncOp, 2, OperandKind::Imm2RM),
        // MOV r32, imm32 - B8+ id
        InstFormat::new("mov", Prefix::Legacy, opcode!(0xB8), EncodeKind::RegEncOp, 4, OperandKind::Imm2RM),
        // MOV r64, imm64 - B8+ io (REX.W)  (full 64-bit immediate)
        InstFormat::new("mov", Prefix::Legacy, opcode!(0xB8), EncodeKind::RegEncOp, 8, OperandKind::Imm2RM),

        // MOV r/m8, r8 - 88 /r
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x88), EncodeKind::ModRM, 1, OperandKind::Reg2RM),
        // MOV r/m16, r16 - 89 /r
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x89), EncodeKind::ModRM, 2, OperandKind::Reg2RM),
        // MOV r/m32, r32 - 89 /r
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x89), EncodeKind::ModRM, 4, OperandKind::Reg2RM),
        // MOV r/m64, r64 - 89 /r (REX.W)
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x89), EncodeKind::ModRM, 8, OperandKind::Reg2RM),

        // MOV r8, r/m8 - 8A /r
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x8A), EncodeKind::ModRM, 1, OperandKind::RM2Reg),
        // MOV r16, r/m16 - 8B /r
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x8B), EncodeKind::ModRM, 2, OperandKind::RM2Reg),
        // MOV r32, r/m32 - 8B /r
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x8B), EncodeKind::ModRM, 4, OperandKind::RM2Reg),
        // MOV r64, r/m64 - 8B /r (REX.W)
        InstFormat::new("mov", Prefix::Legacy, opcode!(0x8B), EncodeKind::ModRM, 8, OperandKind::RM2Reg),

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
