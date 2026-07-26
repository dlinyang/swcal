use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn jmp() -> Vec<InstFormat> {
    vec![
        // not support jmp rel8 rel16 that i can scan once and generate code
        // JMP rel8 - EB cb (short jump, 8-bit relative offset)
        instf!("jmp", Legacy, opcode!(0xEB), no_modrm(), rel(8)),
        // JMP rel16 - E9 cw/cd (near jump, 16-bit relative offset)
        instf!("jmp", Legacy, opcode!(0xE9), no_modrm(), rel(16)),
        // JMP rel32 - E9 cw/cd (near jump, 32-bit relative offset)
        instf!("jmp", Legacy, opcode!(0xE9), no_modrm(), rel(32)),
        // JMP r/m16 - FF /4 (near jump, absolute indirect via register/memory)
        instf!("jmp", Legacy, opcode!(0xFF), digit(4), rm(Gpr, u(16), W)),
        // JMP r/m32 - FF /4 (near jump, absolute indirect via register/memory)
        instf!("jmp", Legacy, opcode!(0xFF), digit(4), rm(Gpr, u(32), W)),
        // JMP r/m64 - FF /4 (near jump, absolute indirect via register/memory, REX.W)
        instf!("jmp", Legacy, opcode!(0xFF), digit(4), rm(Gpr, u(64), W)),
        // // JMP far ptr16:16 - EA ip (far jump, direct)
        // instf!("jmp", Legacy, opcode!(0xEA), no_modrm(), imm_u(16), imm_u(16)),
        // // JMP far ptr16:32 - EA ip (far jump, direct)
        // instf!("jmp", Legacy, opcode!(0xEA), no_modrm(), imm_u(32), imm_u(16)),
        // JMP far m16:16 - FF /5 (far jump, indirect via memory)
        instf!("jmp", Legacy, opcode!(0xFF), digit(5), rm(Gpr, u(16), W)),
        // JMP far m16:32 - FF /5 (far jump, indirect via memory)
        instf!("jmp", Legacy, opcode!(0xFF), digit(5), rm(Gpr, u(32), W)),


        // JE rel8 - 74 cb (short jump, 8-bit relative offset)
        instf!("je", Legacy, opcode!(0x74), no_modrm(), rel(8)),
        // JE rel16 - 0F 84 cw/cd (near jump, 16-bit relative offset)
        instf!("je", Legacy, opcode!(0x0F, 0x84), no_modrm(), rel(16)),
        // JE rel32 - 0F 84 cw/cd (near jump, 32-bit relative offset)
        instf!("je", Legacy, opcode!(0x0F, 0x84), no_modrm(), rel(32)),


        // JNZ rel8 - 75 cb (short jump, 8-bit relative offset)
        instf!("jnz", Legacy, opcode!(0x75), no_modrm(), rel(8)),
        // JNZ rel16 - 0F 85 cw/cd (near jump, 16-bit relative offset)
        instf!("jnz", Legacy, opcode!(0x0F, 0x85), no_modrm(), rel(16)),
        // JNZ rel32 - 0F 85 cw/cd (near jump, 32-bit relative offset)
        instf!("jnz", Legacy, opcode!(0x0F, 0x85), no_modrm(), rel(32)),


        // JL rel8 - 7C cb (short jump, 8-bit relative offset)
        instf!("jl", Legacy, opcode!(0x7C), no_modrm(), rel(8)),
        // JL rel16 - 0F 8C cw/cd (near jump, 16-bit relative offset)
        instf!("jl", Legacy, opcode!(0x0F, 0x8C), no_modrm(), rel(16)),
        // JL rel32 - 0F 8C cw/cd (near jump, 32-bit relative offset)
        instf!("jl", Legacy, opcode!(0x0F, 0x8C), no_modrm(), rel(32)),


        // JGE rel8 - 7D cb (short jump, 8-bit relative offset)
        instf!("jge", Legacy, opcode!(0x7D), no_modrm(), rel(8)),
        // JGE rel16 - 0F 8D cw/cd (near jump, 16-bit relative offset)
        instf!("jge", Legacy, opcode!(0x0F, 0x8D), no_modrm(), rel(16)),
        // JGE rel32 - 0F 8D cw/cd (near jump, 32-bit relative offset)
        instf!("jge", Legacy, opcode!(0x0F, 0x8D), no_modrm(), rel(32)),


    // CMP instruction forms comparing register/memory with register, immediate, and accumulator
    // All standard CMP opcodes and operand types for 8/16/32/64-bit modes
        // CMP r/m8, imm8 - 80 /7 ib
        instf!("cmp", Legacy, opcode!(0x80), digit(7), rm(Gpr, u(8), W), imm_u(8)),
        // CMP r/m16, imm16 - 81 /7 iw
        instf!("cmp", Legacy, opcode!(0x81), digit(7), rm(Gpr, u(16), W), imm_u(16)),
        // CMP r/m32, imm32 - 81 /7 id
        instf!("cmp", Legacy, opcode!(0x81), digit(7), rm(Gpr, u(32), W), imm_u(32)),
        // CMP r/m64, imm32 - 81 /7 id (sign-extended to 64-bit)
        instf!("cmp", Legacy, opcode!(0x81), digit(7), rm(Gpr, i(64), W), imm_i(32)),

        // CMP r/m16, imm8 - 83 /7 ib (sign-extended)
        instf!("cmp", Legacy, opcode!(0x83), digit(7), rm(Gpr, u(16), W), imm_i(8)),
        // CMP r/m32, imm8 - 83 /7 ib (sign-extended)
        instf!("cmp", Legacy, opcode!(0x83), digit(7), rm(Gpr, u(32), W), imm_i(8)),
        // CMP r/m64, imm8 - 83 /7 ib (sign-extended)
        instf!("cmp", Legacy, opcode!(0x83), digit(7), rm(Gpr, i(64), W), imm_i(8)),

        // CMP AL, imm8 - 3C ib
        instf!("cmp", Legacy, opcode!(0x3C), modrm_r(), reg(Gpr, u(8), W), imm_u(8)),
        // CMP AX, imm16 - 3D iw
        instf!("cmp", Legacy, opcode!(0x3D), modrm_r(), reg(Gpr, u(16), W), imm_u(16)),
        // CMP EAX, imm32 - 3D id
        instf!("cmp", Legacy, opcode!(0x3D), modrm_r(), reg(Gpr, u(32), W), imm_u(32)),
        // CMP RAX, imm32 - 3D id (sign-extended to 64-bit, REX.W)
        instf!("cmp", Legacy, opcode!(0x3D), modrm_r(), reg(Gpr, i(64), W), imm_i(32)),

        // CMP r/m8, r8 - 38 /r
        instf!("cmp", Legacy, opcode!(0x38), modrm(), rm(Gpr, u(8), W), reg(Gpr, u(8), R)),
        // CMP r/m16, r16 - 39 /r
        instf!("cmp", Legacy, opcode!(0x39), modrm(), rm(Gpr, u(16), W), reg(Gpr, u(16), R)),
        // CMP r/m32, r32 - 39 /r
        instf!("cmp", Legacy, opcode!(0x39), modrm(), rm(Gpr, u(32), W), reg(Gpr, u(32), R)),
        // CMP r/m64, r64 - 39 /r (REX.W)
        instf!("cmp", Legacy, opcode!(0x39), modrm(), rm(Gpr, u(64), W), reg(Gpr, u(64), R)),

        // CMP r8, r/m8 - 3A /r
        instf!("cmp", Legacy, opcode!(0x3A), modrm(), reg(Gpr, u(8), W), rm(Gpr, u(8), R)),
        // CMP r16, r/m16 - 3B /r
        instf!("cmp", Legacy, opcode!(0x3B), modrm(), reg(Gpr, u(16), W), rm(Gpr, u(16), R)),
        // CMP r32, r/m32 - 3B /r
        instf!("cmp", Legacy, opcode!(0x3B), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(32), R)),
        // CMP r64, r/m64 - 3B /r (REX.W)
        instf!("cmp", Legacy, opcode!(0x3B), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(64), R)),


        // LOOP rel8 - E2 cb (decrement count, jump if RCX/ECX/CX != 0)
        instf!("loop", Legacy, opcode!(0xE2), no_modrm(), rel(8)),
        // LOOPE rel8 - E1 cb (decrement count, jump if RCX/ECX/CX != 0 and ZF=1)
        instf!("loope", Legacy, opcode!(0xE1), no_modrm(), rel(8)),
        // // LOOPNE rel8 - E0 cb (decrement count, jump if RCX/ECX/CX != 0 and ZF=0)
        // instf!("loopne", Legacy, opcode!(0xE0), no_modrm(), imm_i(8)),
        // // // LOOPW rel8 - 67 E2 cb (decrement CX, 16-bit address size override)
        // // instf!("loopw", Legacy, prefix!(0x67), opcode!(0xE2), no_modrm(), imm_i(8)),
        // // // LOOPD rel8 - 67 E2 cb (decrement ECX, 32-bit address size override)
        // // instf!("loopd", Legacy, prefix!(0x67), opcode!(0xE2), no_modrm(), imm_i(8)),
        // // // LOOPQ rel8 - E2 cb (decrement RCX, 64-bit default in x86-64)
        // // instf!("loopq", Legacy, opcode!(0xE2), no_modrm(), imm_i(8)),
        // // JRCXZ rel8 - E3 cb (jump if RCX=0)
        // instf!("jrcxz", Legacy, opcode!(0xE3), no_modrm(), imm_i(8)),
        // // JECXZ rel8 - 67 E3 cb (jump if ECX=0, address size override)
        // instf!("jecxz", Legacy, opcode!(0xE3), no_modrm(), imm_i(8)),
        // // JCXZ rel8 - 67 E3 cb (jump if CX=0, address size override)
        // instf!("jcxz", Legacy, opcode!(0xE3), no_modrm(), imm_i(8)),
    ]
}
