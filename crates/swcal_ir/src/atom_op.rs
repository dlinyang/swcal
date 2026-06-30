/// Atomic operation kinds in the IR.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtomOp {
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
