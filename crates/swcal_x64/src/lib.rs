pub mod el;
pub mod inst;
pub mod asm;

pub mod codegen {
    use crate::inst::*;
    use crate::inst::encode::*;
    use crate::inst::reg::*;
    include!(concat!(env!("OUT_DIR"),"/inst_info.rs"));
    include!(concat!(env!("OUT_DIR"),"/codegen.rs"));
}
