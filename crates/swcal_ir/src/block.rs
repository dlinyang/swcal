use crate::expr::*;
use crate::args::*;

/// A basic block: a sequence of instructions with a labelled entry and block parameters.
///
/// Block parameters (also called block arguments) are SSA values yielded by the terminator
/// of predecessor blocks. They subsume phi nodes and are the canonical way to thread values
/// across control-flow joins.
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub args: Args,
    pub instructions: Vec<Instr>,
}

impl BasicBlock {
    pub fn new(label: impl Into<String>) -> Self {
        BasicBlock {
            label: label.into(),
            args: Args::new(),
            instructions: Vec::new(),
        }
    }

    pub fn new_with_args(label: impl Into<String>, args: Args) -> Self {
        BasicBlock {
            label: label.into(),
            args,
            instructions: Vec::new(),
        }
    }

    pub fn push(&mut self, instr: Instr) {
        self.instructions.push(instr);
    }
}
