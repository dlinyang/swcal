/// Represents a type in the IR.
#[derive(Debug, Clone, PartialEq)]
pub enum IRType {
    Primitive(Primitive),
    Pointer(Box<IRType>),
    Array {
        ty: Box<IRType>,
        len: usize
    },
    Vector{
        ty: Primitive,
        len: usize
    },
    Struct{
        name: Option<String>,
        record: Record,
    },
    Function{
        params: Vec<IRType>,
        ret: Box<IRType>
    },
    Void,
    Never,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    fields: Vec<Field>,
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", field)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    name:  String,
    ty: Box<IRType>,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}:{}", self.name, self.ty)
    }
}

/// Primitive/built-in scalar types.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
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
            IRType::Pointer(inner) => write!(f, "ptr<{}>", inner),
            IRType::Array{ ty, len } => write!(f, "[{} * {}]", ty, len),
            IRType::Vector { ty, len } => write!(f, "vec<{} * {}>", ty, len),
            IRType::Struct{ name, record } => {
                if let Some(name) = name {
                    write!(f, "{} : struct {{ {} }}", name, record)
                }
                else {
                    write!(f, "struct {{ {} }}", record)
                }
            },
            IRType::Never => write!(f, "!"),
            IRType::Void => write!(f, "void"),
            IRType::Function{ params, ret} => {
                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
        }
    }
}
