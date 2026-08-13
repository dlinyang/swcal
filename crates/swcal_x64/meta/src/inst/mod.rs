use std::collections::HashMap;

use crate::format::*;
use crate::*;
use Prefix::*;
use RegKind::*;
use RWAttr::*;
use FixedReg::*;

include!("mov.rs");
include!("lea.rs");
include!("xchg.rs");
include!("arith.rs");
include!("bit_arith.rs");
include!("bool_arith.rs");
include!("jmp.rs");
include!("stack.rs");
include!("call.rs");
include!("syscall.rs");
include!("ret.rs");

pub type InstCodeGenTable = HashMap<String, Vec<InstFormat>>;

pub fn load_instformat(gen_table: &mut InstCodeGenTable, instfs: Vec<InstFormat>) {
    for instf in instfs {
        let name = &instf.mnemonic;
        if let Some(tinstfs) = gen_table.get_mut(name) {
            tinstfs.push(instf);
        }
        else {
            gen_table.insert(instf.mnemonic.clone(), vec![instf]);
        }
    }
}

pub fn inst_codegen_table() -> InstCodeGenTable {
    let mut t = HashMap::new();

    load_instformat(&mut t, mov());
    load_instformat(&mut t, lea());
    load_instformat(&mut t, xchg());
    load_instformat(&mut t, arith());
    load_instformat(&mut t, bit_arith());
    load_instformat(&mut t, bool_arith());
    load_instformat(&mut t, jmp());
    load_instformat(&mut t, stack());
    load_instformat(&mut t, call());
    load_instformat(&mut t, ret());
    load_instformat(&mut t, syscall());

    t
}
