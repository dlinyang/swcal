pub fn ret() -> Vec<InstFormat> {
    vec![
        // RET instruction - Return from procedure
        // RET (near return) - C3
        instf!("ret", Legacy, opcode!(0xC3), no_modrm()),
        // RET imm16 (near return with stack adjustment) - C2 iw
        instf!("ret", Legacy, opcode!(0xC2), no_modrm(), imm_u(16)),
        // RET (far return) - CB
        instf!("retf", Legacy, opcode!(0xCB), no_modrm()),
        // RET imm16 (far return with stack adjustment) - CA iw
        instf!("retf", Legacy, opcode!(0xCA), no_modrm(),imm_u(16)),
    ]
}
