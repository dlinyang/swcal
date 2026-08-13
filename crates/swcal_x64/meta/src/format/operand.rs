use crate::{generate::*, type_name};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    /// unsigned integer n-bit
    U(u16),
    /// integer n-bit
    I(u16),
    /// float n-bit
    F(u16),
}

impl ValType {
    pub fn width(&self) -> u16 {
        match self {
            ValType::U(b) | ValType::I(b) |ValType::F(b) => *b,
        }
    }

    pub fn is_width<const U: u16>(&self) -> bool {
        match self {
            ValType::U(b) | ValType::I(b) |ValType::F(b) => (*b) == U,
        }
    }
}

impl std::fmt::Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValType::U(bit) => write!(f, "u{}", bit),
            ValType::I(bit) => write!(f, "i{}", bit),
            ValType::F(bit) => write!(f, "f{}", bit),
        }
    }
}

pub fn u(bit: u16) -> ValType {
    ValType::U(bit)
}

pub fn i(bit: u16) -> ValType {
    ValType::I(bit)
}

pub fn f(bit: u16) -> ValType {
    ValType::F(bit)
}

impl SrcGen for ValType {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        match self {
            ValType::U(b) => format!("u{}", b),
            ValType::I(b) => format!("i{}", b),
            ValType::F(b) => format!("f{}", b),
        }
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RWAttr {
    /// Read Reg R/M
    R,
    /// Write Reg R/M
    W,
    /// Read and write
    RW,
}

impl std::fmt::Display for RWAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RWAttr::R => write!(f, "Read"),
            RWAttr::W => write!(f, "Write"),
            RWAttr::RW => write!(f, "ReadWrite"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegKind {
    Gpr,
    XMM,
    YMM,
    Fixed{ reg: FixedReg, implicitly: bool},
}

impl RegKind {
    pub fn is_reg(&self) -> bool {
        *self == Self::Gpr
    }
}

impl SrcGen for RegKind {
    fn var_name(&self) -> String {
        match self {
            RegKind::Gpr => "gpr".into(),
            RegKind::XMM => "xmm".into(),
            RegKind::YMM => "ymm".into(),
            RegKind::Fixed{reg,..} => format!("fixed_{}", reg.to_string().to_ascii_lowercase()),
        }
    }

    fn type_name(&self) -> String {
        match self {
            RegKind::Gpr => "Gpr".into(),
            RegKind::XMM => "XMM".into(),
            RegKind::YMM => "YMM".into(),
            RegKind::Fixed{reg,..} => format!("Fixed<{}>", reg),
        }
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandKind {
    Reg(RegKind,RWAttr),
    RM(RegKind,RWAttr),
    IMM,
    Rel,
    MOFFSET,
}

impl SrcGen for OperandKind {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        match self {
            OperandKind::Reg(..) => "R".into(),
            OperandKind::RM(..) => "M".into(),
            OperandKind::IMM => "I".into(),
            OperandKind::Rel => "D".into(),
            OperandKind::MOFFSET => "O".into(),
        }
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}

impl std::fmt::Display for OperandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandKind::Reg(_, _) => write!(f, "reg"),
            OperandKind::RM(_, _) => write!(f, "reg/mem"),
            OperandKind::IMM => write!(f, "imm"),
            OperandKind::MOFFSET => write!(f, "moffset"),
            OperandKind::Rel => write!(f, "rel"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct OperandFormat {
    pub ty: OperandKind,
    pub val_ty: ValType,
}

impl OperandFormat {
    pub fn is_width<const U: u16>(&self) -> bool {
        self.val_ty.is_width::<U>()
    }

    pub fn is_reg(&self) -> bool {
        if let OperandKind::Reg(reg,_) = self.ty { reg.is_reg() } else {false}
    }

    pub fn is_rm(&self) -> bool {
        if let OperandKind::RM(_,_) = self.ty {true} else {false}
    }
}

impl std::fmt::Display for OperandFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ty, self.val_ty)
    }
}

impl SrcGen for OperandFormat {
    fn var_name(&self) -> String {
        match self.ty {
            OperandKind::Reg(reg_kind, _rwattr) => format!("{}{}", reg_kind.var_name(), self.val_ty.width()),
            OperandKind::RM(_reg_kind, _rwattr) => format!("rm{}", self.val_ty.width()),
            OperandKind::IMM => format!("imm{}", self.val_ty.width()),
            OperandKind::Rel => format!("rel{}", self.val_ty.width()),
            OperandKind::MOFFSET => todo!(),
        }
    }

    fn type_name(&self) -> String {
        match self.ty {
            OperandKind::Reg(reg_kind, _rwattr) => format!("{}", reg_kind.type_name()),
            OperandKind::RM(reg_kind, _rwattr) => format!("RM<{}>", reg_kind.type_name()),
            OperandKind::IMM => format!("Imm{}", self.val_ty.width()),
            OperandKind::Rel => format!("Rel{}", self.val_ty.width()),
            OperandKind::MOFFSET => format!("Moffset{}", self.val_ty.width()),
        }
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}

pub fn reg(reg_ty: RegKind, val_ty: ValType, rw: RWAttr) -> OperandFormat {
    OperandFormat { ty: OperandKind::Reg(reg_ty,rw), val_ty }
}

pub fn rm(reg_ty: RegKind, val_ty: ValType, rw: RWAttr) -> OperandFormat {
    OperandFormat { ty: OperandKind::RM(reg_ty, rw), val_ty }
}

pub fn imm_u(bit: u16) -> OperandFormat {
    OperandFormat { ty: OperandKind::IMM, val_ty: u(bit)}
}

pub fn imm_i(bit: u16) -> OperandFormat {
    OperandFormat { ty: OperandKind::IMM, val_ty: i(bit)}
}

pub fn imm_f(bit: u16) -> OperandFormat {
    OperandFormat { ty: OperandKind::IMM, val_ty: f(bit)}
}

pub fn rel(bit: u16) -> OperandFormat {
    OperandFormat { ty: OperandKind::Rel, val_ty: i(bit)}
}
pub fn fixed(reg: FixedReg, val_ty: ValType, rw: RWAttr) -> OperandFormat {
    OperandFormat { ty:  OperandKind::Reg( RegKind::Fixed { reg, implicitly: false }, rw), val_ty}
}

pub fn implicit(reg: FixedReg, val_ty: ValType, rw: RWAttr) -> OperandFormat {
    OperandFormat { ty: OperandKind::Reg(RegKind::Fixed { reg, implicitly: true }, rw), val_ty}
}

#[derive(Clone, Copy)]
pub enum OperandEncode {
    NoOperand,
    One(OperandFormat),
    Two(OperandFormat, OperandFormat),
    Three(OperandFormat, OperandFormat, OperandFormat),
}

impl OperandEncode {
    pub fn is_all_width<const U: u16>(&self) -> bool {
        //need fix exist 32bit to 64bit
        match self {
            OperandEncode::NoOperand => false,
            OperandEncode::One(o1) => o1.is_width::<U>(),
            OperandEncode::Two(o1, o2) => o1.is_width::<U>() && o2.is_width::<U>() ,
            OperandEncode::Three(o1, o2, o3) => o1.is_width::<U>() && o2.is_width::<U>() && o3.is_width::<U>(),
        }
    }

    pub fn is_exist_width<const U: u16>(&self) -> bool {
        match self {
            OperandEncode::NoOperand => false,
            OperandEncode::One(o1) => o1.is_width::<U>(),
            OperandEncode::Two(o1, o2) => o1.is_width::<U>() || o2.is_width::<U>(),
            OperandEncode::Three(o1, o2, o3) => o1.is_width::<U>() || o2.is_width::<U>() || o3.is_width::<U>(),
        }
    }

    pub fn to_vec(&self) -> Vec<OperandFormat> {
        match self.clone() {
            OperandEncode::NoOperand => vec![],
            OperandEncode::One(o) => vec![o],
            OperandEncode::Two(o1, o2) => vec![o1, o2],
            OperandEncode::Three(o1, o2, o3) => vec![o1, o2, o3],
        }
    }
}

impl std::fmt::Display for OperandEncode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandEncode::NoOperand => Ok(()),
            OperandEncode::One(ope) => write!(f, "{}", ope),
            OperandEncode::Two(dst, src) => write!(f, "{:<12}, {:<12}", dst, src),
            OperandEncode::Three(dst, src, src_other) => write!(f, "{:<12}, {:<12}, {:<12}", dst, src, src_other),
        }
    }
}

impl SrcGen for OperandEncode {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        match self {
            OperandEncode::NoOperand => "".into(),
            OperandEncode::One(ope) => type_name!(ope.ty, ope.val_ty),
            OperandEncode::Two(dst, src) => type_name!(dst.ty, src.ty, dst.val_ty, src.val_ty),
            OperandEncode::Three(o1, o2, o3) => type_name!(o1.ty,o2.ty,o3.ty,o1.val_ty,o2.val_ty,o3.val_ty),
        }
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}

#[macro_export]
macro_rules! operand {
    {} => {
        OperandEncode::NoOperand
    };
    {$ope: expr} => {
        OperandEncode::One($ope)
    };
    {$dst: expr, $src: expr} => {
        OperandEncode::Two($dst,$src)
    };
    {$dst: expr, $src: expr, $src_other: expr } => {
        OperandEncode::Three($dst, $src, $src_other)
    };
}

pub fn genarate_operand_field(builder: &mut RustBuilder, encode: &OperandEncode) {
    match encode {
        OperandEncode::NoOperand => {},
        OperandEncode::One(o) => {builder.line(format!("pub {}: {},", o.var_name(), o.type_name()));},
        OperandEncode::Two(o1, o2) => {
            builder.line(format!("pub {}: {},", o1.var_name(), o1.type_name()));
            builder.line(format!("pub {}: {},", o2.var_name(), o2.type_name()));
        },
        OperandEncode::Three(o1, o2, o3) => {
            builder.line(format!("pub {}: {},", o1.var_name(), o1.type_name()));
            builder.line(format!("pub {}: {},", o2.var_name(), o2.type_name()));
            builder.line(format!("pub {}: {},", o3.var_name(), o3.type_name()));
        },
    }
}

macro_rules! fixed_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident $(= $value:expr)?),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant $(= $value)?),*
        }

        impl $name {
            pub fn all() -> Vec<Self> {
                vec![$(Self::$variant),*]
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, stringify!($variant)),)*
                }
            }
        }
    };
}

fixed_enum!(
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FixedReg {
        // Fixed registers.
        AL,
        AX,
        EAX,
        RAX,
        RBX,
        DX,
        EDX,
        RDX,
        CL,
        RCX,
        // XMM0,
    }
);
