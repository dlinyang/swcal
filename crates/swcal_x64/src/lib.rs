pub mod el;
pub mod reloc;
pub mod inst;
pub mod asm;

pub mod codegen {
    use crate::inst::*;
    use crate::inst::base::*;
    use crate::inst::encode::*;
    use crate::inst::rex::*;
    use crate::inst::mem::RM;
    use crate::inst::gpr::*;
    use crate::inst::imm::*;
    use crate::inst::modrm::*;
    use crate::inst::rel::*;
    include!(concat!(env!("OUT_DIR"),"/inst_info.rs"));
    include!(concat!(env!("OUT_DIR"),"/codegen.rs"));
}
