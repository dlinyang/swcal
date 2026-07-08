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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Var(name) => write!(f, "{}", name),
            Value::ConstInt(val) => write!(f, "{}", val),
            Value::ConstFloat(val) => write!(f, "{}", val),
            Value::ConstBool(val) => write!(f, "{}", val),
            Value::Label(name) => write!(f, "label {}", name),
        }
    }
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

use std::fmt;

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instr::Copy { dest, src } => write!(f, "{} = {}", dest, src),
            Instr::Binary { dest, op, lhs, rhs } => write!(f, "{} = {} {} {}", dest, lhs, op, rhs),
            Instr::Unary { dest, op, operand } => write!(f, "{} = {} {}", dest, op, operand),
            Instr::Load { dest, ptr } => write!(f, "{} = load {}", dest, ptr),
            Instr::Store { ptr, val } => write!(f, "store {}, {}", val, ptr),
            Instr::Alloca { dest, ty } => write!(f, "{} = alloca {}", dest, ty),
            Instr::Call { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                match dest {
                    Some(d) => write!(f, "{} = call {}({})", d, func, args_str.join(", ")),
                    None => write!(f, "call {}({})", func, args_str.join(", ")),
                }
            }
            Instr::Return(val) => match val {
                Some(v) => write!(f, "return {}", v),
                None => write!(f, "return"),
            },
            Instr::Jump { target, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "br {}({})", target, args_str.join(", "))
            }
            Instr::Branch {
                cond,
                then_target,
                then_args,
                else_target,
                else_args,
            } => {
                let then_args_str: Vec<String> = then_args.iter().map(|a| a.to_string()).collect();
                let else_args_str: Vec<String> = else_args.iter().map(|a| a.to_string()).collect();
                write!(
                    f,
                    "br {}, {}({}), {}({})",
                    cond,
                    then_target,
                    then_args_str.join(", "),
                    else_target,
                    else_args_str.join(", ")
                )
            }
            Instr::Phi { dest, incoming } => {
                let pairs: Vec<String> = incoming
                    .iter()
                    .map(|(val, label)| format!("({}, {})", val, label))
                    .collect();
                write!(f, "{} = phi [{}]", dest, pairs.join(", "))
            }
        }
    }
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
