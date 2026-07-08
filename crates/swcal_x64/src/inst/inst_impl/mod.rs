use std::collections::HashMap;

use crate::inst::{inst::Inst, format::*};

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

pub fn codgen_emit(inst: &Inst, codgen_table: &InstCodeGenTable) -> Vec<Result<BinInst, String>> {
    let mut ret = vec![];
    if let Some(inst_formats) = codgen_table.get(&inst.mnemonic) {
        for gen_format in inst_formats {
            ret.push(gen_format.encode(inst));
        }
    }
    ret
}
