use std::{any::type_name, fmt::format};

use crate::{generate::*, type_name};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    /// unsigned integer n-bit
    U(u16),
    /// integer n-bit
    I(u16),
    /// unsigned integer n-bit
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
}

impl SrcGen for RegKind {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        match self {
            RegKind::Gpr => "Gpr".into(),
            RegKind::XMM => "XMM".into(),
            RegKind::YMM => "YMM".into(),
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
            OperandKind::MOFFSET => "D".into(),
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
        }
    }
}

pub struct OperandFormat {
    ty: OperandKind,
    val_ty: ValType,
}

impl OperandFormat {
    pub fn is_width<const U: u16>(&self) -> bool {
        self.val_ty.is_width::<U>()
    }
}

impl std::fmt::Display for OperandFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ty, self.val_ty)
    }
}

impl SrcGen for OperandFormat {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        match self.ty {
            OperandKind::Reg(reg_kind, rwattr) => format!("{}{}<R,{}>", reg_kind.type_name(), self.val_ty.width(), rwattr),
            OperandKind::RM(reg_kind, rwattr) => format!("{}{}<M,{}>", reg_kind.type_name(), self.val_ty.width(), rwattr),
            OperandKind::IMM => format!("Imm{}", self.val_ty.width()),
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

pub enum OperandEncode {
    NoOperand,
    One(OperandFormat),
    Two(OperandFormat, OperandFormat),
    Tree(OperandFormat, OperandFormat, OperandFormat),
}

impl OperandEncode {
    pub fn is_width<const U: u16>(&self) -> bool {
        match self {
            OperandEncode::NoOperand => false,
            OperandEncode::One(o1) => o1.is_width::<U>(),
            OperandEncode::Two(o1, o2) => o1.is_width::<U>() && o2.is_width::<U>(),
            OperandEncode::Tree(o1, o2, o3) => o1.is_width::<U>() && o2.is_width::<U>() && o3.is_width::<U>(),
        }
    }
}

impl std::fmt::Display for OperandEncode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandEncode::NoOperand => Ok(()),
            OperandEncode::One(ope) => write!(f, "{}", ope),
            OperandEncode::Two(dst, src) => write!(f, "{:<12}, {:<12}", dst, src),
            OperandEncode::Tree(dst, src, src_other) => write!(f, "{:<12}, {:<12}, {:<12}", dst, src, src_other),
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
            OperandEncode::Tree(_, _, _) => todo!(),
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
        OperandEncode::One(ope) => {builder.line(format!("operand: {},", ope.type_name()));},
        OperandEncode::Two(dst, src) => {
            builder.line(format!("dst: {},", dst.type_name()));
            builder.line(format!("src: {},", src.type_name()));
        },
        OperandEncode::Tree(_dst, _src, _src_ext) => todo!(),
    }
}
