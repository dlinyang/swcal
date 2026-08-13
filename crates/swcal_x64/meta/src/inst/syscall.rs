#[must_use]
pub fn syscall() -> Vec<InstFormat> {
    // SYSCALL instruction - 0F 05
    // Fast system call (64-bit mode)
    // Takes no operands; RCX = RIP, R11 = RFLAGS, jumps to IA32_LSTAR MSR
    vec![
        instf!("syscall", Legacy, opcode!(0x0F, 0x05), no_modrm()),
    // SYSRET instruction - 0F 07
    // Return from fast system call (64-bit mode)
    // Takes no operands; RIP = RCX, RFLAGS = R11, uses IA32_STAR MSR
        instf!("sysret", Legacy, opcode!(0x0F, 0x07), no_modrm()),
    // INT n - CD ib (software interrupt)
        // INT imm8 - CD ib
        instf!("int", Legacy, opcode!(0xCD), no_modrm(), imm_u(8)),
    // INT3 - CC (breakpoint, 1-byte instruction)
        instf!("int3", Legacy, opcode!(0xCC), no_modrm()),
    // IRET/IRETD/IRETQ - CF
    // Return from interrupt (operand size determines behavior)
        // IRET (16-bit) - CF
        instf!("iret", Legacy, opcode!(0xCF), no_modrm()),
        // IRETD (32-bit) - CF (default in 32-bit mode)
        instf!("iretd", Legacy, opcode!(0xCF), no_modrm()),
        // IRETQ (64-bit) - CF (REX.W)
        instf!("iretq", Legacy, opcode!(0xCF), no_modrm()),

    // SYSENTER - 0F 34
    // Fast system call entry (legacy, primarily 32-bit)
        instf!("sysenter", Legacy, opcode!(0x0F, 0x34), no_modrm()),

    // SYSEXIT - 0F 35
    // Fast system call return (legacy, primarily 32-bit)
        instf!("sysexit", Legacy, opcode!(0x0F, 0x35), no_modrm()),
    ]
}
