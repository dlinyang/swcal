use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;

// lea
#[must_use]
pub fn lea() -> Vec<InstFormat> {
    vec![
        // LEA instruction - Load Effective Address
        // LEA r16, m - 8D /r
        instf!("lea", Legacy, opcode!(0x8D), modrm(), reg(Gpr, u(16), W), rm(Gpr, u(16), R)),
        // LEA r32, m - 8D /r
        instf!("lea", Legacy, opcode!(0x8D), modrm(), reg(Gpr, u(32), W), rm(Gpr, u(32), R)),
        // LEA r64, m - 8D /r (REX.W)
        instf!("lea", Legacy, opcode!(0x8D), modrm(), reg(Gpr, u(64), W), rm(Gpr, u(64), R)),
    ]
}
