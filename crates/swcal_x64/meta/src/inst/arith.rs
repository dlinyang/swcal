use crate::format::*;
use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn arith() -> Vec<InstFormat> {
    vec![
        // ADD r/m8, imm8 - 80 /0 ib
        instf!("add", Legacy, opcode!(0x80), digit(0), rm(Gpr, u(8), RW), imm_u(8)),
        // ADD r/m16, imm16 - 81 /0 iw
        instf!("add", Legacy, opcode!(0x81), digit(0), rm(Gpr, u(16), RW), imm_u(16)),
        // ADD r/m32, imm32 - 81 /0 id
        instf!("add", Legacy, opcode!(0x81), digit(0), rm(Gpr, u(32), RW), imm_u(32)),
        // ADD r/m64, imm32 - 81 /0 id (sign-extended to 64-bit)
        instf!("add", Legacy, opcode!(0x81), digit(0), rm(Gpr, i(64), RW), imm_i(32)),

        // ADD r/m16, imm8 - 83 /0 ib (sign-extended)
        instf!("add", Legacy, opcode!(0x83), digit(0), rm(Gpr, u(16), RW), imm_i(8)),
        // ADD r/m32, imm8 - 83 /0 ib (sign-extended)
        instf!("add", Legacy, opcode!(0x83), digit(0), rm(Gpr, u(32), RW), imm_i(8)),
        // ADD r/m64, imm8 - 83 /0 ib (sign-extended to 64-bit)
        instf!("add", Legacy, opcode!(0x83), digit(0), rm(Gpr, i(64), RW), imm_i(8)),

        // ADD AL, imm8 - 04 ib
        instf!("add", Legacy, opcode!(0x04), no_modrm(), reg(Fgr(0), u(8), RW), imm_u(8)),
        // ADD AX, imm16 - 05 iw
        instf!("add", Legacy, opcode!(0x05), no_modrm(), reg(Fgr(0), u(16), RW), imm_u(16)),
        // ADD EAX, imm32 - 05 id
        instf!("add", Legacy, opcode!(0x05), no_modrm(), reg(Fgr(0), u(32), RW), imm_u(32)),
        // ADD RAX, imm32 - 05 id (sign-extended to 64-bit)
        instf!("add", Legacy, opcode!(0x05), no_modrm(), reg(Fgr(0), i(64), RW), imm_i(32)),

        // ADD r/m8, r8 - 00 /r
        instf!("add", Legacy, opcode!(0x00), modrm(), rm(Gpr, u(8), RW), reg(Gpr, u(8), R)),
        // ADD r/m16, r16 - 01 /r
        instf!("add", Legacy, opcode!(0x01), modrm(), rm(Gpr, u(16), RW), reg(Gpr, u(16), R)),
        // ADD r/m32, r32 - 01 /r
        instf!("add", Legacy, opcode!(0x01), modrm(), rm(Gpr, u(32), RW), reg(Gpr, u(32), R)),
        // ADD r/m64, r64 - 01 /r (REX.W)
        instf!("add", Legacy, opcode!(0x01), modrm(), rm(Gpr, u(64), RW), reg(Gpr, u(64), R)),

        // ADD r8, r/m8 - 02 /r
        instf!("add", Legacy, opcode!(0x02), modrm(), reg(Gpr, u(8), RW), rm(Gpr, u(8), R)),
        // ADD r16, r/m16 - 03 /r
        instf!("add", Legacy, opcode!(0x03), modrm(), reg(Gpr, u(16), RW), rm(Gpr, u(16), R)),
        // ADD r32, r/m32 - 03 /r
        instf!("add", Legacy, opcode!(0x03), modrm(), reg(Gpr, u(32), RW), rm(Gpr, u(32), R)),
        // ADD r64, r/m64 - 03 /r (REX.W)
        instf!("add", Legacy, opcode!(0x03), modrm(), reg(Gpr, u(64), RW), rm(Gpr, u(64), R)),

        // SUB instruction forms covering register, memory, and immediate variants
        // All standard SUB opcodes and operand types for 16/32/64-bit modes
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


        // INC instruction forms covering register and memory variants for 8/16/32/64-bit modes
        // All standard INC opcodes and operand types
        // INC r/m8 - FE /0
        instf!("inc", Legacy, opcode!(0xFE), digit(0), rm(Gpr, u(8), RW)),
        // INC r/m16 - FF /0
        instf!("inc", Legacy, opcode!(0xFF), digit(0), rm(Gpr, u(16), RW)),
        // INC r/m32 - FF /0
        instf!("inc", Legacy, opcode!(0xFF), digit(0), rm(Gpr, u(32), RW)),
        // INC r/m64 - FF /0 (REX.W)
        instf!("inc", Legacy, opcode!(0xFF), digit(0), rm(Gpr, u(64), RW)),

        // DEC instruction forms covering register and memory variants for 8/16/32/64-bit modes
        // All standard DEC opcodes and operand types
        // DEC r/m8 - FE /1
        instf!("dec", Legacy, opcode!(0xFE), digit(1), rm(Gpr, u(8), RW)),
        // DEC r/m16 - FF /1
        instf!("dec", Legacy, opcode!(0xFF), digit(1), rm(Gpr, u(16), RW)),
        // DEC r/m32 - FF /1
        instf!("dec", Legacy, opcode!(0xFF), digit(1), rm(Gpr, u(32), RW)),
        // DEC r/m64 - FF /1 (REX.W)
        instf!("dec", Legacy, opcode!(0xFF), digit(1), rm(Gpr, u(64), RW)),

        // MUL instruction forms - unsigned multiply
        // Note: MUL always implicitly uses AL/AX/EAX/RAX as the source and stores the result
        // in AX (8-bit), DX:AX (16-bit), EDX:EAX (32-bit), or RDX:RAX (64-bit)
        // MUL r/m8 - F6 /4 (AX = AL * r/m8)
        instf!("mul", Legacy, opcode!(0xF6), digit(4), rm(Gpr, u(8), R)),
        // MUL r/m16 - F7 /4 (DX:AX = AX * r/m16)
        instf!("mul", Legacy, opcode!(0xF7), digit(4), rm(Gpr, u(16), R)),
        // MUL r/m32 - F7 /4 (EDX:EAX = EAX * r/m32)
        instf!("mul", Legacy, opcode!(0xF7), digit(4), rm(Gpr, u(32), R)),
        // MUL r/m64 - F7 /4 (REX.W) (RDX:RAX = RAX * r/m64)
        instf!("mul", Legacy, opcode!(0xF7), digit(4), rm(Gpr, u(64), R)),

        // IMUL instruction forms - signed multiply
        // Three main forms exist:
        // 1. Single-operand: IMUL r/m (implicit AX/EAX/RAX * r/m)
        // 2. Two-operand: IMUL r, r/m (reg = reg * r/m)
        // 3. Three-operand: IMUL r, r/m, imm (reg = r/m * imm)
        // Single-operand forms (implicit accumulator):
        // IMUL r/m8 - F6 /5 (AX = AL * r/m8, signed)
        instf!("imul", Legacy, opcode!(0xF6), digit(5), rm(Gpr, u(8), R)),
        // IMUL r/m16 - F7 /5 (DX:AX = AX * r/m16, signed)
        instf!("imul", Legacy, opcode!(0xF7), digit(5), rm(Gpr, u(16), R)),
        // IMUL r/m32 - F7 /5 (EDX:EAX = EAX * r/m32, signed)
        instf!("imul", Legacy, opcode!(0xF7), digit(5), rm(Gpr, u(32), R)),
        // IMUL r/m64 - F7 /5 (REX.W) (RDX:RAX = RAX * r/m64, signed)
        instf!("imul", Legacy, opcode!(0xF7), digit(5), rm(Gpr, u(64), R)),

        // Two-operand forms: IMUL r16, r/m16 - 0F AF /r
        instf!("imul", Legacy, opcode!(0x0F, 0xAF), modrm(), reg(Gpr, u(16), RW), rm(Gpr, u(16), R)),
        // IMUL r32, r/m32 - 0F AF /r
        instf!("imul", Legacy, opcode!(0x0F, 0xAF), modrm(), reg(Gpr, u(32), RW), rm(Gpr, u(32), R)),
        // IMUL r64, r/m64 - 0F AF /r (REX.W)
        instf!("imul", Legacy, opcode!(0x0F, 0xAF), modrm(), reg(Gpr, u(64), RW), rm(Gpr, u(64), R)),

        // Three-operand forms with immediate (sign-extended):
        // IMUL r16, r/m16, imm8 - 6B /r ib
        instf!("imul", Legacy, opcode!(0x6B), modrm(), reg(Gpr, u(16), RW), rm(Gpr, u(16), R), imm_i(8)),
        // IMUL r32, r/m32, imm8 - 6B /r ib
        instf!("imul", Legacy, opcode!(0x6B), modrm(), reg(Gpr, u(32), RW), rm(Gpr, u(32), R), imm_i(8)),
        // IMUL r64, r/m64, imm8 - 6B /r ib (REX.W)
        instf!("imul", Legacy, opcode!(0x6B), modrm(), reg(Gpr, u(64), RW), rm(Gpr, u(64), R), imm_i(8)),
        // IMUL r16, r/m16, imm16 - 69 /r iw
        instf!("imul", Legacy, opcode!(0x69), modrm(), reg(Gpr, u(16), RW), rm(Gpr, u(16), R), imm_u(16)),
        // IMUL r32, r/m32, imm32 - 69 /r id
        instf!("imul", Legacy, opcode!(0x69), modrm(), reg(Gpr, u(32), RW), rm(Gpr, u(32), R), imm_u(32)),
        // IMUL r64, r/m64, imm32 - 69 /r id (sign-extended to 64-bit, REX.W)
        instf!("imul", Legacy, opcode!(0x69), modrm(), reg(Gpr, i(64), RW), rm(Gpr, i(64), R), imm_i(32)),

    // DIV instruction forms - unsigned divide
    // Note: DIV always implicitly uses AX/DX:AX/EDX:EAX/RDX:RAX as the dividend and stores
    // the quotient in AL/AX/EAX/RAX and remainder in AH/DX/EDX/RDX

        // DIV r/m8 - F6 /6 (AX = AL / r/m8, AH = remainder)
        instf!("div", Legacy, opcode!(0xF6), digit(6), rm(Gpr, u(8), R)),
        // DIV r/m16 - F7 /6 (AX = DX:AX / r/m16, DX = remainder)
        instf!("div", Legacy, opcode!(0xF7), digit(6), rm(Gpr, u(16), R)),
        // DIV r/m32 - F7 /6 (EAX = EDX:EAX / r/m32, EDX = remainder)
        instf!("div", Legacy, opcode!(0xF7), digit(6), rm(Gpr, u(32), R)),
        // DIV r/m64 - F7 /6 (REX.W) (RAX = RDX:RAX / r/m64, RDX = remainder)
        instf!("div", Legacy, opcode!(0xF7), digit(6), rm(Gpr, u(64), R)),
    ]
}
