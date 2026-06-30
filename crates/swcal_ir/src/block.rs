use super::atom_op::AtomOp;
use super::types::IRType;

/// An operand / value in three-address code.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Named SSA variable.
    Var(String),
    /// Integer literal.
    ConstInt(i128),
    /// Float literal.
    ConstFloat(f64),
    /// Boolean literal.
    ConstBool(bool),
    /// Basic-block label used as a value (e.g. for phi nodes, indirect branch).
    Label(String),
}

/// A three-address code instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    /// `dest = src`
    Copy { dest: String, src: Value },
    /// `dest = lhs op rhs`
    Binary { dest: String, op: AtomOp, lhs: Value, rhs: Value },
    /// `dest = op operand`
    Unary { dest: String, op: AtomOp, operand: Value },
    /// `dest = load ptr`
    Load { dest: String, ptr: Value },
    /// `store val, ptr`
    Store { ptr: Value, val: Value },
    /// `dest = alloca type`
    Alloca { dest: String, ty: IRType },
    /// `dest = call func(args)`
    Call {
        dest: Option<String>,
        func: String,
        args: Vec<Value>,
    },
    /// `return val` or `return`
    Return(Option<Value>),
    /// `br label(args...)`
    Jump {
        target: String,
        args: Vec<Value>,
    },
    /// `br cond, then_label(args...), else_label(args...)`
    Branch {
        cond: Value,
        then_target: String,
        then_args: Vec<Value>,
        else_target: String,
        else_args: Vec<Value>,
    },
    /// `dest = phi [(val1, label1), (val2, label2), ...]`
    Phi {
        dest: String,
        incoming: Vec<(Value, String)>,
    },
}

/// A basic block: a sequence of instructions with a labelled entry and block parameters.
///
/// Block parameters (also called block arguments) are SSA values yielded by the terminator
/// of predecessor blocks. They subsume phi nodes and are the canonical way to thread values
/// across control-flow joins.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: String,
    pub params: Vec<String>,
    pub instructions: Vec<Instr>,
}

impl BasicBlock {
    pub fn new(label: impl Into<String>) -> Self {
        BasicBlock {
            label: label.into(),
            params: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn new_with_params(label: impl Into<String>, params: Vec<String>) -> Self {
        BasicBlock {
            label: label.into(),
            params,
            instructions: Vec::new(),
        }
    }

    pub fn push(&mut self, instr: Instr) {
        self.instructions.push(instr);
    }
}
