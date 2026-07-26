use std::collections::HashMap;

use crate::format::*;

mod mov;
mod lea;
mod xchg;
mod arith;
mod bit_arith;
mod bool_arith;
mod jmp;
mod stack;
mod call;
mod syscall;
mod ret;

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

    load_instformat(&mut t, mov::mov());
    load_instformat(&mut t, lea::lea());
    load_instformat(&mut t, xchg::xchg());
    load_instformat(&mut t, arith::arith());
    load_instformat(&mut t, bit_arith::bit_arith());
    load_instformat(&mut t, bool_arith::bool_arith());
    load_instformat(&mut t, jmp::jmp());
    load_instformat(&mut t, stack::stack());
    load_instformat(&mut t, call::call());
    load_instformat(&mut t, ret::ret());
    load_instformat(&mut t, syscall::syscall());

    t
}
