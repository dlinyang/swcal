use crate::*;
use Prefix::*;
// use RegKind::*;
// use RWAttr::*;

#[must_use]
pub fn syscall() -> Vec<InstFormat> {
    // SYSCALL instruction - 0F 05
    // Fast system call (64-bit mode)
    // Takes no operands; RCX = RIP, R11 = RFLAGS, jumps to IA32_LSTAR MSR
    vec![
        instf!("syscall", Legacy, opcode!(0x0F, 0x05), no_modrm()),
    ]
}

#[must_use]
pub fn sysret() -> Vec<InstFormat> {
    // SYSRET instruction - 0F 07
    // Return from fast system call (64-bit mode)
    // Takes no operands; RIP = RCX, RFLAGS = R11, uses IA32_STAR MSR
    vec![
        instf!("sysret", Legacy, opcode!(0x0F, 0x07), no_modrm()),
    ]
}

#[must_use]
pub fn int() -> Vec<InstFormat> {
    // INT n - CD ib (software interrupt)
    vec![
        // INT imm8 - CD ib
        instf!("int", Legacy, opcode!(0xCD), no_modrm(), imm_u(8)),
    ]
}

#[must_use]
pub fn int3() -> Vec<InstFormat> {
    // INT3 - CC (breakpoint, 1-byte instruction)
    vec![
        instf!("int3", Legacy, opcode!(0xCC), no_modrm()),
    ]
}

#[must_use]
pub fn iret() -> Vec<InstFormat> {
    // IRET/IRETD/IRETQ - CF
    // Return from interrupt (operand size determines behavior)
    vec![
        // IRET (16-bit) - CF
        instf!("iret", Legacy, opcode!(0xCF), no_modrm()),
        // IRETD (32-bit) - CF (default in 32-bit mode)
        instf!("iretd", Legacy, opcode!(0xCF), no_modrm()),
        // IRETQ (64-bit) - CF (REX.W)
        instf!("iretq", Legacy, opcode!(0xCF), no_modrm()),
    ]
}

#[must_use]
pub fn sysenter() -> Vec<InstFormat> {
    // SYSENTER - 0F 34
    // Fast system call entry (legacy, primarily 32-bit)
    vec![
        instf!("sysenter", Legacy, opcode!(0x0F, 0x34), no_modrm()),
    ]
}

#[must_use]
pub fn sysexit() -> Vec<InstFormat> {
    // SYSEXIT - 0F 35
    // Fast system call return (legacy, primarily 32-bit)
    vec![
        instf!("sysexit", Legacy, opcode!(0x0F, 0x35), no_modrm()),
    ]
}
