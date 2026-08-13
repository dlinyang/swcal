/// Atomic operation kinds in the IR.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryAtomOp {
    // ── Arithmetic ──
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    // ── Bitwise ──
    And,
    Or,
    Xor,
    Shl,
    Shr,

    // ── Comparison ──
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryAtomOp {
    // ── Unary ──
    /// Arithmetic negation (`-x`).
    Neg,
    /// Bitwise/logical not (`~x` / `!x`).
    Not,

    // ── Cast ──
    /// Zero-extension (unsigned widening).
    ZExt,
    /// Sign-extension (signed widening).
    SExt,
    /// Truncation (narrowing).
    Trunc,
    /// Float to signed integer.
    FpToSi,
    /// Float to unsigned integer.
    FpToUi,
    /// Signed integer to float.
    SiToFp,
    /// Unsigned integer to float.
    UiToFp,
    /// Bit-preserving reinterpret cast.
    BitCast,
    /// Integer to pointer.
    IntToPtr,
    /// Pointer to integer.
    PtrToInt,
}
use std::fmt;

impl fmt::Display for BinaryAtomOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Mul => write!(f, "mul"),
            Self::Div => write!(f, "div"),
            Self::Rem => write!(f, "rem"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::Shl => write!(f, "shl"),
            Self::Shr => write!(f, "shr"),
            Self::Eq => write!(f, "eq"),
            Self::Ne => write!(f, "ne"),
            Self::Lt => write!(f, "lt"),
            Self::Le => write!(f, "le"),
            Self::Gt => write!(f, "gt"),
            Self::Ge => write!(f, "ge"),
        }
    }
}

impl fmt::Display for UnaryAtomOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neg => write!(f, "neg"),
            Self::Not => write!(f, "not"),
            Self::ZExt => write!(f, "zext"),
            Self::SExt => write!(f, "sext"),
            Self::Trunc => write!(f, "trunc"),
            Self::FpToSi => write!(f, "fptosi"),
            Self::FpToUi => write!(f, "fptoui"),
            Self::SiToFp => write!(f, "sitofp"),
            Self::UiToFp => write!(f, "uitofp"),
            Self::BitCast => write!(f, "bitcast"),
            Self::IntToPtr => write!(f, "inttoptr"),
            Self::PtrToInt => write!(f, "ptrtoint"),
        }
    }
}
