use crate::format::*;
use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

pub fn inc() -> Vec<InstFormat> {
    // INC instruction forms covering register and memory variants for 8/16/32/64-bit modes
    // All standard INC opcodes and operand types
    vec![
        // INC r/m8 - FE /0
        instf!("inc", Legacy, opcode!(0xFE), digit(0), rm(Gpr, u(8), RW)),
        // INC r/m16 - FF /0
        instf!("inc", Legacy, opcode!(0xFF), digit(0), rm(Gpr, u(16), RW)),
        // INC r/m32 - FF /0
        instf!("inc", Legacy, opcode!(0xFF), digit(0), rm(Gpr, u(32), RW)),
        // INC r/m64 - FF /0 (REX.W)
        instf!("inc", Legacy, opcode!(0xFF), digit(0), rm(Gpr, u(64), RW)),
    ]
}
