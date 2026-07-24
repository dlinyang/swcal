use crate::format::*;
use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn add() -> Vec<InstFormat> {
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
    ]
}
