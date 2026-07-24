use crate::format::*;
use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn xchg() -> Vec<InstFormat> {
    // XCHG instruction forms for 8/16/32/64-bit modes
    // XCHG exchanges the contents of two operands (atomic by default on x86)
    vec![
        // XCHG r/m8, r8 - 86 /r
        instf!("xchg", Legacy, opcode!(0x86), modrm(), rm(Gpr, u(8), RW), reg(Gpr, u(8), RW)),
        // XCHG r8, r/m8 - 86 /r (same opcode, operand order matters)
        instf!("xchg", Legacy, opcode!(0x86), modrm(), reg(Gpr, u(8), RW), rm(Gpr, u(8), RW)),

        // XCHG r/m16, r16 - 87 /r
        instf!("xchg", Legacy, opcode!(0x87), modrm(), rm(Gpr, u(16), RW), reg(Gpr, u(16), RW)),
        // XCHG r16, r/m16 - 87 /r
        instf!("xchg", Legacy, opcode!(0x87), modrm(), reg(Gpr, u(16), RW), rm(Gpr, u(16), RW)),

        // XCHG r/m32, r32 - 87 /r
        instf!("xchg", Legacy, opcode!(0x87), modrm(), rm(Gpr, u(32), RW), reg(Gpr, u(32), RW)),
        // XCHG r32, r/m32 - 87 /r
        instf!("xchg", Legacy, opcode!(0x87), modrm(), reg(Gpr, u(32), RW), rm(Gpr, u(32), RW)),

        // XCHG r/m64, r64 - 87 /r (REX.W)
        instf!("xchg", Legacy, opcode!(0x87), modrm(), rm(Gpr, u(64), RW), reg(Gpr, u(64), RW)),
        // XCHG r64, r/m64 - 87 /r (REX.W)
        instf!("xchg", Legacy, opcode!(0x87), modrm(), reg(Gpr, u(64), RW), rm(Gpr, u(64), RW)),

        // XCHG r64, r64 (short form: xchg eAX/rax + reg) - 90+rd (REX.W for 64-bit)
        // In 64-bit mode, the 0x90 opcode is xchg rax, r64 (not nop)
        instf!("xchg", Legacy, opcode!(0x90), modrm_r(), reg(Fgr(0), u(32), RW), reg(Gpr, u(32), RW)),
        // XCHG r64, eAX/rax (0x91-0x97, same encoding as 90+rd, for completeness with rax as second operand)
        instf!("xchg", Legacy, opcode!(0x90), modrm_r(), reg(Fgr(0), u(64), RW), reg(Gpr, u(64), RW)),
    ]
}
