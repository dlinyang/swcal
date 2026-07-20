use crate::format::OperandEncode;

#[derive(Debug)]
pub enum Prefix {
    Legacy,
    Vex,
    Evex,
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Legacy => write!(f, "Legacy"),
            Prefix::Vex => write!(f, "Vex"),
            Prefix::Evex => write!(f, "Evex"),
        }
    }
}

/// check 64bit long mode and 32bit compat need operand size override for 16bit
pub fn legacy_prefix_66h(op_ecode: &OperandEncode) -> bool {
    op_ecode.is_width::<16>()
}

/// check 64bit long mode for 64bit operand
pub fn legacy_prefix_rex_w(op_ecode: &OperandEncode) -> bool {
    op_ecode.is_width::<64>()
}
