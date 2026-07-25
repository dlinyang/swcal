use std::collections::HashMap;

use crate::format::*;

pub mod mov;
pub mod lea;
pub mod xchg;
pub mod arith;
pub mod bit_arith;
pub mod bool;
pub mod jmp;
pub mod stack;
pub mod call;
pub mod syscall;
pub mod ret;

pub type InstCodeGenTable = HashMap<String, Vec<InstFormat>>;


pub fn inst_codegen_table() -> InstCodeGenTable {
    let mut t = HashMap::new();

    t.insert("mov".to_string(), mov::mov());
    t.insert("movsx".to_string(), mov::movsx());
    t.insert("movzx".to_string(), mov::movzx());

    t.insert("lea".to_string(), lea::lea());

    t.insert("xchg".to_string(), xchg::xchg());
    t.insert("nop".to_string(), xchg::nop());

    t.insert("add".to_string(), arith::add());
    t.insert("sub".to_string(), arith::sub());
    t.insert("inc".to_string(), arith::inc());
    t.insert("dec".to_string(), arith::dec());
    t.insert("mul".to_string(), arith::mul());
    t.insert("imul".to_string(), arith::imul());
    t.insert("div".to_string(), arith::div());

    t.insert("xor".to_string(), bit_arith::xor());
    t.insert("shl".to_string(), bit_arith::shl());
    t.insert("shr".to_string(), bit_arith::shr());
    t.insert("sar".to_string(), bit_arith::sar());

    t.insert("and".to_string(), bool::and());
    t.insert("or".to_string(), bool::or());
    t.insert("not".to_string(), bool::not());

    t.insert("jmp".to_string(), jmp::jmp());
    t.insert("je".to_string(), jmp::je());
    t.insert("jnz".to_string(), jmp::jnz());
    t.insert("jl".to_string(), jmp::jl());
    t.insert("jge".to_string(), jmp::jge());
    t.insert("cmp".to_string(), jmp::cmp());
    t.insert("loop".to_string(), jmp::loop_inst());

    t.insert("push".to_string(), stack::push());
    t.insert("pushfq".to_string(), stack::pushfq());
    t.insert("pop".to_string(), stack::pop());
    t.insert("popfq".to_string(), stack::popfq());

    t.insert("call".to_string(), call::call());
    t.insert("ret".to_string(), ret::ret());

    t.insert("syscall".to_string(), syscall::syscall());

    t
}
