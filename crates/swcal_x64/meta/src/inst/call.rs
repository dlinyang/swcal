use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

#[must_use]
pub fn call() -> Vec<InstFormat> {
    vec![
        // CALL rel32 - E8 cd (call near, relative offset)
        instf!("call", Legacy, opcode!(0xE8), no_modrm(), imm_i(32)),

        // CALL rel16 - E8 cw (16-bit call near, relative offset - rarely used in 64-bit mode)
        // Note: 16-bit relative call not typically encoded in x86-64; E8 with 32-bit displacement is standard.

        // CALL r/m64 - FF /2 (call near, absolute indirect through register or memory)
        instf!("call", Legacy, opcode!(0xFF), digit(2), rm(Gpr, u(64), R)),

        // CALL far indirect m16:32/64 - FF /3 (call far, indirect)
        // Not commonly needed for 64-bit mode but included for completeness
        // instf!("call", Legacy, opcode!(0xFF), digit(3), rm(Mem, u(64), R)),

        // CALL far direct - 9A cd (call far, direct) - not valid in 64-bit mode
        // Skipped as it's invalid in long mode.

        // Register-indirect call via r64 (already covered by FF /2 with Gpr rm)
    ]
}
