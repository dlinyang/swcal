#[must_use]
pub fn mov() -> Vec<InstFormat> {
    vec![
        // MOV instruction forms covering register, memory, immediate, and accumulator variants
        // All standard MOV opcodes and operand types for 16/32/64-bit modes
        // MOV r/m8, imm8 - C6 /0 ib
        instf!("mov", Legacy, opcode!(0xC6), digit(0), rm(Gpr, u(8), W), imm_u(8)),
        // MOV r/m16, imm16 - C7 /0 iw  (16-bit)
        instf!("mov", Legacy, opcode!(0xC7), digit(0), rm(Gpr, u(16), W), imm_u(16)),
        // MOV r/m32, imm32 - C7 /0 id
        instf!("mov", Legacy, opcode!(0xC7), digit(0), rm(Gpr, u(32), W), imm_u(32)),
        // MOV r/m64, imm32 - C7 /0 id (sign-extended to 64-bit)
        instf!("mov", Legacy, opcode!(0xC7), digit(0), rm(Gpr, i(64), W), imm_i(32)),

        // MOV r18, imm8  - B0+ ib
        instf!("mov", Legacy, opcode!(0xB0), modrm_r(), reg(Gpr, u(8), W), imm_u(8)),
        // MOV r16, imm16 - B8+ iw
        instf!("mov", Legacy, opcode!(0xB8), modrm_r(), reg(Gpr, u(16), W), imm_u(16)),
        // MOV r32, imm32 - B8+ id
        instf!("mov", Legacy, opcode!(0xB8), modrm_r(), reg(Gpr, u(32), W), imm_u(32)),
        // MOV r64, imm64 - B8+ io (REX.W)  (full 64-bit immediate)
        instf!("mov", Legacy, opcode!(0xB8), modrm_r(), reg(Gpr, u(64), W), imm_u(64)),

        // MOV r/m8, r8 - 88 /r
        instf!("mov", Legacy, opcode!(0x88), modrm(), rm(Gpr, u(8), W), reg(Gpr, u(8), R)),
        // MOV r/m16, r16 - 89 /r
        instf!("mov", Legacy, opcode!(0x89), modrm(), rm(Gpr, u(16), W), reg(Gpr, u(16), R)),
        // MOV r/m32, r32 - 89 /r
        instf!("mov", Legacy, opcode!(0x89), modrm(), rm(Gpr, u(32), W), reg(Gpr, u(32), R)),
        // MOV r/m64, r64 - 89 /r (REX.W)
        instf!("mov", Legacy, opcode!(0x89), modrm(), rm(Gpr, u(64), W), reg(Gpr, u(64), R)),

        // MOV r8, r/m8 - 8A /r
        instf!("mov", Legacy, opcode!(0x8A), modrm(), reg(Gpr, u(8), W), rm(Gpr, u(8), R)),
        // MOV r16, r/m16 - 8B /r
        instf!("mov", Legacy, opcode!(0x8B), modrm(), reg(Gpr, u(16), W), rm(Gpr, u(16), R)),
        // MOV r32, r/m32 - 8B /r
        instf!("mov", Legacy, opcode!(0x8B), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(32), R)),
        // MOV r64, r/m64 - 8B /r (REX.W)
        instf!("mov", Legacy, opcode!(0x8B), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(64), R)),

        // // MOV AL, m8 (memory direct) - A0 (accumulator load)
        // instf!("mov", Legacy, opcode!(0xA0), modrm_r(), 8, reg(Fgr(0), u(8), W), imm_u(8)),
        // // MOV AX, m16 - A1
        // instf!("mov", Legacy, opcode!(0xA1), modrm_r(), 16, reg(Fgr(0), u(16), W), imm_u(16)),
        // // MOV EAX, m32 - A1
        // instf!("mov", Legacy, opcode!(0xA1), modrm_r(), 32, reg(Fgr(0), u(16), W), imm_u(32)),
        // // MOV RAX, m64 - A1 (REX.W)
        // instf!("mov", Legacy, opcode!(0xA1), modrm_r(), 64, reg(Fgr(0), u(32), W), imm_u(64)),

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


        // MOVSX instruction formats
        // MOVSX r16, r/m8  - 0F BE /r
        instf!("movsx", Legacy, opcode!(0x0F, 0xBE), modrm(), reg(Gpr, u(16), W), rm(Gpr, i(8), R)),
        // MOVSX r32, r/m8  - 0F BE /r
        instf!("movsx", Legacy, opcode!(0x0F, 0xBE), modrm(), reg(Gpr, u(32), W), rm(Gpr, i(8), R)),
        // MOVSX r64, r/m8  - 0F BE /r (REX.W)
        instf!("movsx", Legacy, opcode!(0x0F, 0xBE), modrm(), reg(Gpr, u(64), W), rm(Gpr, i(8), R)),
        // MOVSX r32, r/m16 - 0F BF /r
        instf!("movsx", Legacy, opcode!(0x0F, 0xBF), modrm(), reg(Gpr, u(32), W), rm(Gpr, i(16), R)),
        // MOVSX r64, r/m16 - 0F BF /r (REX.W)
        instf!("movsx", Legacy, opcode!(0x0F, 0xBF), modrm(), reg(Gpr, u(64), W), rm(Gpr, i(16), R)),


        // MOVZX instruction formats
        // MOVZX r16, r/m8  - 0F B6 /r
        instf!("movzx", Legacy, opcode!(0x0F, 0xB6), modrm(), reg(Gpr, u(16), W), rm(Gpr, u(8), R)),
        // MOVZX r32, r/m8  - 0F B6 /r
        instf!("movzx", Legacy, opcode!(0x0F, 0xB6), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(8), R)),
        // MOVZX r64, r/m8  - 0F B6 /r (REX.W)
        instf!("movzx", Legacy, opcode!(0x0F, 0xB6), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(8), R)),
        // MOVZX r32, r/m16 - 0F B7 /r
        instf!("movzx", Legacy, opcode!(0x0F, 0xB7), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(16), R)),
        // MOVZX r64, r/m16 - 0F B7 /r (REX.W)
        instf!("movzx", Legacy, opcode!(0x0F, 0xB7), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(16), R)),
    ]
}
