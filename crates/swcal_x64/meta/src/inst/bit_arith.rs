use crate::format::*;
use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn xor() -> Vec<InstFormat> {
    vec![
        // XOR r/m8, imm8 - 80 /6 ib
        instf!("xor", Legacy, opcode!(0x80), digit(6), rm(Gpr, u(8), W), imm_u(8)),
        // XOR r/m16, imm16 - 81 /6 iw
        instf!("xor", Legacy, opcode!(0x81), digit(6), rm(Gpr, u(16), W), imm_u(16)),
        // XOR r/m32, imm32 - 81 /6 id
        instf!("xor", Legacy, opcode!(0x81), digit(6), rm(Gpr, u(32), W), imm_u(32)),
        // XOR r/m64, imm32 - 81 /6 id (sign-extended)
        instf!("xor", Legacy, opcode!(0x81), digit(6), rm(Gpr, i(64), W), imm_i(32)),

        // XOR r/m16, imm8 - 83 /6 ib
        instf!("xor", Legacy, opcode!(0x83), digit(6), rm(Gpr, u(16), W), imm_i(8)),
        // XOR r/m32, imm8 - 83 /6 ib
        instf!("xor", Legacy, opcode!(0x83), digit(6), rm(Gpr, u(32), W), imm_i(8)),
        // XOR r/m64, imm8 - 83 /6 ib (sign-extended)
        instf!("xor", Legacy, opcode!(0x83), digit(6), rm(Gpr, i(64), W), imm_i(8)),

        // XOR r/m8, r8 - 30 /r
        instf!("xor", Legacy, opcode!(0x30), modrm(), rm(Gpr, u(8), W), reg(Gpr, u(8), R)),
        // XOR r/m16, r16 - 31 /r
        instf!("xor", Legacy, opcode!(0x31), modrm(), rm(Gpr, u(16), W), reg(Gpr, u(16), R)),
        // XOR r/m32, r32 - 31 /r
        instf!("xor", Legacy, opcode!(0x31), modrm(), rm(Gpr, u(32), W), reg(Gpr, u(32), R)),
        // XOR r/m64, r64 - 31 /r (REX.W)
        instf!("xor", Legacy, opcode!(0x31), modrm(), rm(Gpr, u(64), W), reg(Gpr, u(64), R)),

        // XOR r8, r/m8 - 32 /r
        instf!("xor", Legacy, opcode!(0x32), modrm(), reg(Gpr, u(8), W), rm(Gpr, u(8), R)),
        // XOR r16, r/m16 - 33 /r
        instf!("xor", Legacy, opcode!(0x33), modrm(), reg(Gpr, u(16), W), rm(Gpr, u(16), R)),
        // XOR r32, r/m32 - 33 /r
        instf!("xor", Legacy, opcode!(0x33), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(32), R)),
        // XOR r64, r/m64 - 33 /r (REX.W)
        instf!("xor", Legacy, opcode!(0x33), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(64), R)),

        // XOR AL, imm8 - 34 ib
        instf!("xor", Legacy, opcode!(0x34), modrm_r(), reg(Gpr, u(8), W), imm_u(8)),
        // XOR AX, imm16 - 35 iw
        instf!("xor", Legacy, opcode!(0x35), modrm_r(), reg(Gpr, u(16), W), imm_u(16)),
        // XOR EAX, imm32 - 35 id
        instf!("xor", Legacy, opcode!(0x35), modrm_r(), reg(Gpr, u(32), W), imm_u(32)),
        // XOR RAX, imm32 - 35 id (REX.W)
        instf!("xor", Legacy, opcode!(0x35), modrm_r(), reg(Gpr, u(64), W), imm_i(32)),
    ]
}

pub fn shl() -> Vec<InstFormat> {
    vec![
        // SHL r/m8, imm8 - C0 /4 ib
        instf!("shl", Legacy, opcode!(0xC0), digit(4), rm(Gpr, u(8), W), imm_u(8)),
        // SHL r/m16, imm8 - C1 /4 ib
        instf!("shl", Legacy, opcode!(0xC1), digit(4), rm(Gpr, u(16), W), imm_u(8)),
        // SHL r/m32, imm8 - C1 /4 ib
        instf!("shl", Legacy, opcode!(0xC1), digit(4), rm(Gpr, u(32), W), imm_u(8)),
        // SHL r/m64, imm8 - C1 /4 ib (REX.W)
        instf!("shl", Legacy, opcode!(0xC1), digit(4), rm(Gpr, u(64), W), imm_u(8)),

        // SHL r/m8, 1 - D0 /4
        instf!("shl", Legacy, opcode!(0xD0), digit(4), rm(Gpr, u(8), W)),
        // SHL r/m16, 1 - D1 /4
        instf!("shl", Legacy, opcode!(0xD1), digit(4), rm(Gpr, u(16), W)),
        // SHL r/m32, 1 - D1 /4
        instf!("shl", Legacy, opcode!(0xD1), digit(4), rm(Gpr, u(32), W)),
        // SHL r/m64, 1 - D1 /4 (REX.W)
        instf!("shl", Legacy, opcode!(0xD1), digit(4), rm(Gpr, u(64), W)),

        // SHL r/m8, CL - D2 /4
        instf!("shl", Legacy, opcode!(0xD2), digit(4), rm(Gpr, u(8), W), reg(Fgr(1), u(8), R)),
        // SHL r/m16, CL - D3 /4
        instf!("shl", Legacy, opcode!(0xD3), digit(4), rm(Gpr, u(16), W), reg(Fgr(1), u(16), R)),
        // SHL r/m32, CL - D3 /4
        instf!("shl", Legacy, opcode!(0xD3), digit(4), rm(Gpr, u(32), W), reg(Fgr(1), u(32), R)),
        // SHL r/m64, CL - D3 /4 (REX.W)
        instf!("shl", Legacy, opcode!(0xD3), digit(4), rm(Gpr, u(64), W), reg(Fgr(1), u(64), R)),
    ]
}

pub fn shr() -> Vec<InstFormat> {
    vec![
        // SHR r/m8, imm8 - C0 /5 ib
        instf!("shr", Legacy, opcode!(0xC0), digit(5), rm(Gpr, u(8), W), imm_u(8)),
        // SHR r/m16, imm8 - C1 /5 ib
        instf!("shr", Legacy, opcode!(0xC1), digit(5), rm(Gpr, u(16), W), imm_u(8)),
        // SHR r/m32, imm8 - C1 /5 ib
        instf!("shr", Legacy, opcode!(0xC1), digit(5), rm(Gpr, u(32), W), imm_u(8)),
        // SHR r/m64, imm8 - C1 /5 ib (REX.W)
        instf!("shr", Legacy, opcode!(0xC1), digit(5), rm(Gpr, u(64), W), imm_u(8)),

        // SHR r/m8, 1 - D0 /5
        instf!("shr", Legacy, opcode!(0xD0), digit(5), rm(Gpr, u(8), W)),
        // SHR r/m16, 1 - D1 /5
        instf!("shr", Legacy, opcode!(0xD1), digit(5), rm(Gpr, u(16), W)),
        // SHR r/m32, 1 - D1 /5
        instf!("shr", Legacy, opcode!(0xD1), digit(5), rm(Gpr, u(32), W)),
        // SHR r/m64, 1 - D1 /5 (REX.W)
        instf!("shr", Legacy, opcode!(0xD1), digit(5), rm(Gpr, u(64), W)),

        // SHR r/m8, CL - D2 /5
        instf!("shr", Legacy, opcode!(0xD2), digit(5), rm(Gpr, u(8), W), reg(Fgr(1), u(8), R)),
        // SHR r/m16, CL - D3 /5
        instf!("shr", Legacy, opcode!(0xD3), digit(5), rm(Gpr, u(16), W), reg(Fgr(1), u(16), R)),
        // SHR r/m32, CL - D3 /5
        instf!("shr", Legacy, opcode!(0xD3), digit(5), rm(Gpr, u(32), W), reg(Fgr(1), u(32), R)),
        // SHR r/m64, CL - D3 /5 (REX.W)
        instf!("shr", Legacy, opcode!(0xD3), digit(5), rm(Gpr, u(64), W), reg(Fgr(1), u(64), R)),
    ]
}

pub fn sar() -> Vec<InstFormat> {
    vec![
        // SAR r/m8, imm8 - C0 /7 ib
        instf!("sar", Legacy, opcode!(0xC0), digit(7), rm(Gpr, u(8), W), imm_u(8)),
        // SAR r/m16, imm8 - C1 /7 ib
        instf!("sar", Legacy, opcode!(0xC1), digit(7), rm(Gpr, u(16), W), imm_u(8)),
        // SAR r/m32, imm8 - C1 /7 ib
        instf!("sar", Legacy, opcode!(0xC1), digit(7), rm(Gpr, u(32), W), imm_u(8)),
        // SAR r/m64, imm8 - C1 /7 ib (REX.W)
        instf!("sar", Legacy, opcode!(0xC1), digit(7), rm(Gpr, u(64), W), imm_u(8)),

        // SAR r/m8, 1 - D0 /7
        instf!("sar", Legacy, opcode!(0xD0), digit(7), rm(Gpr, u(8), W)),
        // SAR r/m16, 1 - D1 /7
        instf!("sar", Legacy, opcode!(0xD1), digit(7), rm(Gpr, u(16), W)),
        // SAR r/m32, 1 - D1 /7
        instf!("sar", Legacy, opcode!(0xD1), digit(7), rm(Gpr, u(32), W)),
        // SAR r/m64, 1 - D1 /7 (REX.W)
        instf!("sar", Legacy, opcode!(0xD1), digit(7), rm(Gpr, u(64), W)),

        // SAR r/m8, CL - D2 /7
        instf!("sar", Legacy, opcode!(0xD2), digit(7), rm(Gpr, u(8), W), reg(Fgr(1), u(8), R)),
        // SAR r/m16, CL - D3 /7
        instf!("sar", Legacy, opcode!(0xD3), digit(7), rm(Gpr, u(16), W), reg(Fgr(1), u(16), R)),
        // SAR r/m32, CL - D3 /7
        instf!("sar", Legacy, opcode!(0xD3), digit(7), rm(Gpr, u(32), W), reg(Fgr(1), u(32), R)),
        // SAR r/m64, CL - D3 /7 (REX.W)
        instf!("sar", Legacy, opcode!(0xD3), digit(7), rm(Gpr, u(64), W), reg(Fgr(1), u(64), R)),
    ]
}
