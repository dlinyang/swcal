use std::collections::HashMap;

use crate::format::*;

pub mod mov;
pub mod xchg;
pub mod add;
pub mod sub;
pub mod inc;

pub type InstCodeGenTable = HashMap<String, Vec<InstFormat>>;

pub fn inst_codegen_table() -> InstCodeGenTable {
    let mut t = HashMap::new();
    t.insert("mov".to_string(), mov::mov());
    t.insert("xchg".to_string(), xchg::xchg());
    t.insert("add".to_string(), add::add());
    t.insert("sub".to_string(), sub::sub());
    t.insert("inc".to_string(), inc::inc());
    t
}
