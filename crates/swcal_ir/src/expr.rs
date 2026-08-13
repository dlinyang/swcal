use super::atom_op::*;
use super::types::IRType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IRVar (u16);

pub struct VarBuilder {
    id_count: usize,
    reg_max: usize,
}

impl std::fmt::Display for IRVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

impl VarBuilder {
    pub fn new() -> Self {
        Self {
            id_count: 0,
            reg_max: u16::MAX as usize,
        }
    }

    pub fn fresh_reg(&mut self)-> Result<IRVar, ()> {
        self.id_count += 1;
        if self.id_count > self.reg_max  as usize {
            Err(())
        }
        else {
            Ok(IRVar(self.id_count as u16 - 1 ))
        }
    }

}

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
    /// `dst = src`
    Copy { dst: IRVar, src: IRVar },
    /// `dst = lhs op rhs`
    Binary { dst: IRVar, op: BinaryAtomOp, lhs: IRVar, rhs: IRVar },
    /// `dst = op operand`
    Unary { dst: IRVar, op: UnaryAtomOp, operand: IRVar },
    /// `dest = load ptr`
    Load { dst: IRVar, ptr: IRVar },
    /// `store val, ptr`
    Store { ptr: IRVar, val: IRVar },
    /// `dst = alloca type`
    Alloca { dst: IRVar, ty: IRType },
    /// `dest = call func(args)`
    Call {
        dst: Option<IRVar>,
        func: String,
        args: Vec<Value>,
    },
    /// `return val` or `return`
    Return(Option<IRVar>),
    /// `br label(args...)`
    Jump {
        target: String,
        args: Vec<Value>,
    },
    /// `br cond, then_label(args...), else_label(args...)`
    Branch {
        cond: IRVar,
        then_target: String,
        then_args: Vec<Value>,
        else_target: String,
        else_args: Vec<Value>,
    },
    // /// `dest = phi [(val1, label1), (val2, label2), ...]`
    // Phi {
    //     dest: IRVar,
    //     incoming: Vec<(Value, String)>,
    // },
}

use std::fmt;

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instr::Copy { dst, src } => write!(f, "{} = {}", dest, src),
            Instr::Binary { dst, op, lhs, rhs } => write!(f, "{} = {} {} {}", dest, lhs, op, rhs),
            Instr::Unary { dst, op, operand } => write!(f, "{} = {} {}", dest, op, operand),
            Instr::Load { dst, ptr } => write!(f, "{} = load {}", dest, ptr),
            Instr::Store { ptr, val } => write!(f, "store {}, {}", val, ptr),
            Instr::Alloca { dst, ty } => write!(f, "{} = alloca {}", dest, ty),
            Instr::Call { dst, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                match dst {
                    Some(d) => write!(f, "{} = call {}({})", d, func, args_str.join(", ")),
                    None => write!(f, "call {}({})", func, args_str.join(", ")),
                }
            }
            Instr::Return(var) => match var {
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
            // Instr::Phi { dest, incoming } => {
            //     let pairs: Vec<String> = incoming
            //         .iter()
            //         .map(|(val, label)| format!("({}, {})", val, label))
            //         .collect();
            //     write!(f, "{} = phi [{}]", dest, pairs.join(", "))
            // }
        }
    }
}
