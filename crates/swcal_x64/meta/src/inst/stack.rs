use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn stack() -> Vec<InstFormat> {
    vec![
            // PUSH r/m16 - FF /6 (push 16-bit register/memory)
            instf!("push", Legacy, opcode!(0xFF), digit(6), rm(Gpr, u(16), R)),
            // PUSH r/m32 - FF /6 (push 32-bit register/memory)
            instf!("push", Legacy, opcode!(0xFF), digit(6), rm(Gpr, u(32), R)),
            // PUSH r/m64 - FF /6 (push 64-bit register/memory, REX.W)
            instf!("push", Legacy, opcode!(0xFF), digit(6), rm(Gpr, u(64), R)),

            // PUSH imm8 - 6A ib (push sign-extended 8-bit immediate)
            instf!("push", Legacy, opcode!(0x6A), no_modrm(), imm_i(8)),
            // PUSH imm16 - 68 iw (push 16-bit immediate)
            instf!("push", Legacy, opcode!(0x68), no_modrm(), imm_i(16)),
            // PUSH imm32 - 68 id (push 32-bit immediate, sign-extended to 64-bit in 64-bit mode)
            instf!("push", Legacy, opcode!(0x68), no_modrm(), imm_i(32)),

            // PUSH r16 - 50+rw (push 16-bit register)
            instf!("push", Legacy, opcode!(0x50), modrm_r(), reg(Gpr, u(16), R)),
            // PUSH r32 - 50+rd (push 32-bit register)
            instf!("push", Legacy, opcode!(0x50), modrm_r(), reg(Gpr, u(32), R)),
            // PUSH r64 - 50+rd (push 64-bit register, REX.W)
            instf!("push", Legacy, opcode!(0x50), modrm_r(), reg(Gpr, u(64), R)),

            // PUSHFQ - 9C (push RFLAGS, 64-bit operand size)
            instf!("pushfq", Legacy, opcode!(0x9C), no_modrm()),

            // POP r/m16 - 8F /0 (pop 16-bit register/memory)
            instf!("pop", Legacy, opcode!(0x8F), digit(0), rm(Gpr, u(16), W)),
            // POP r/m32 - 8F /0 (pop 32-bit register/memory)
            instf!("pop", Legacy, opcode!(0x8F), digit(0), rm(Gpr, u(32), W)),
            // POP r/m64 - 8F /0 (pop 64-bit register/memory, REX.W)
            instf!("pop", Legacy, opcode!(0x8F), digit(0), rm(Gpr, u(64), W)),

            // POP r16 - 58+rw (pop 16-bit register)
            instf!("pop", Legacy, opcode!(0x58), modrm_r(), reg(Gpr, u(16), W)),
            // POP r32 - 58+rd (pop 32-bit register)
            instf!("pop", Legacy, opcode!(0x58), modrm_r(), reg(Gpr, u(32), W)),
            // POP r64 - 58+rd (pop 64-bit register, REX.W)
            instf!("pop", Legacy, opcode!(0x58), modrm_r(), reg(Gpr, u(64), W)),

        // POPFQ - 9D (pop RFLAGS, 64-bit operand size)
        instf!("popfq", Legacy, opcode!(0x9D), no_modrm()),
    ]
}
