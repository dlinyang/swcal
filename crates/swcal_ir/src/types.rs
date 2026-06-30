/// Represents a type in the IR.
#[derive(Debug, Clone, PartialEq)]
pub enum IRType {
    Primitive(Primitive),
    Pointer(Box<IRType>),
    Array(Box<IRType>, usize),
    Function(Vec<IRType>, Box<IRType>),
    NamedStruct(String),
    Never,
}

/// Primitive/built-in scalar types.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Void => write!(f, "void"),
            Primitive::Bool => write!(f, "bool"),
            Primitive::U8 => write!(f, "u8"),
            Primitive::U16 => write!(f, "u16"),
            Primitive::U32 => write!(f, "u32"),
            Primitive::U64 => write!(f, "u64"),
            Primitive::U128 => write!(f, "u128"),
            Primitive::I8 => write!(f, "i8"),
            Primitive::I16 => write!(f, "i16"),
            Primitive::I32 => write!(f, "i32"),
            Primitive::I64 => write!(f, "i64"),
            Primitive::I128 => write!(f, "i128"),
            Primitive::F32 => write!(f, "f32"),
            Primitive::F64 => write!(f, "f64"),
        }
    }
}

impl std::fmt::Display for IRType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRType::Primitive(p) => write!(f, "{}", p),
            IRType::Pointer(inner) => write!(f, "*{}", inner),
            IRType::Array(elem, len) => write!(f, "[{}; {}]", elem, len),
            IRType::Function(params, ret) => {
                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
            IRType::NamedStruct(name) => write!(f, "{}", name),
            IRType::Never => write!(f, "!"),
        }
    }
}
