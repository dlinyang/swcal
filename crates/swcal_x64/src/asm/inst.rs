use std::str::FromStr;

use crate::asm::data::*;
use crate::asm::lexer::*;
use crate::inst::*;
use swcal_parsec::choice;
use swcal_parsec::parsec::*;
use swcal_parsec::text::*;

/// Note: this is not fast way to impl parse inst
pub fn parse_inst(src: Text) -> ParseResult<Text, Inst> {
    choice!(
        parse_inst_rm_op_imm_to_reg,
        parse_inst_rm2reg,
        parse_inst_imm2rm,
        parse_inst_reg2rm,
        parse_inst_rm,
        parse_inst_imm,
        parse_inst_oprand_zero
    )
    .parse(src)
}

pub fn parse_inst_oprand_zero<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    // let (_, rest) = lexeme(ws, newline_or_end).parse(rest)?;
    Ok((
        Inst {
            mnemonic: mnemonic_name.inner.to_string(),
            operand: Operand::Zero,
        },
        rest,
    ))
}

pub fn parse_inst_imm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (imm, rest) = lexeme(ws, parse_imm).parse(rest)?;
    Ok((
        Inst {
            mnemonic: mnemonic_name.inner.to_string(),
            operand: Operand::Imm(imm),
        },
        rest,
    ))
}

pub fn parse_inst_rm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (rm, rest) = lexeme(ws, parse_rm).parse(rest)?;
    Ok((
        Inst {
            mnemonic: mnemonic_name.inner.to_string(),
            operand: Operand::RM(rm),
        },
        rest,
    ))
}

pub fn parse_inst_imm2rm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;

    let (width_opt, rest) =
        if let Ok((with, rest)) = lexeme(ws, parse_size_define).terminated(ws).parse(rest) {
            (Some(with), rest)
        } else {
            (None, rest)
        };

    let (rm, rest) = lexeme(ws, parse_rm).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (imm, rest) = lexeme(ws, parse_imm).parse(rest)?;

    Ok((
        Inst {
            mnemonic: mnemonic_name.inner.to_string(),
            operand: Operand::Imm2RM { rm: rm, imm: imm },
        },
        rest,
    ))
}

pub fn parse_inst_reg2rm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (rm, rest) = lexeme(ws, parse_rm).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (reg, rest) = lexeme(ws, parse_reg).parse(rest)?;

    Ok((
        Inst {
            mnemonic: mnemonic_name.inner.to_string(),
            operand: Operand::Reg2RM { reg: reg, rm: rm },
        },
        rest,
    ))
}

pub fn parse_inst_rm2reg<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (reg, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (rm, rest) = lexeme(ws, parse_rm).parse(rest)?;

    Ok((
        Inst {
            mnemonic: mnemonic_name.inner.to_string(),
            operand: Operand::RM2Reg { reg, rm },
        },
        rest,
    ))
}

pub fn parse_inst_rm_op_imm_to_reg(src: Text) -> ParseResult<Text, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (reg, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (rm, rest) = lexeme(ws, parse_rm).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (imm, rest) = lexeme(ws, parse_imm).parse(rest)?;

    Ok((
        Inst {
            mnemonic: mnemonic_name.inner.to_string(),
            operand: Operand::RmOpImm2reg { src: reg, rm, imm },
        },
        rest,
    ))
}

fn parse_rm(src: Text) -> ParseResult<Text, RM> {
    let parse_rm_reg = |input| parse_reg(input).map(|(reg, rest)| (RM::Reg(reg), rest));
    choice!(parse_mem, parse_rm_reg).parse(src)
}

fn parse_mem<'a>(src: Text<'a>) -> ParseResult<Text<'a>, RM> {
    choice!(parse_sib, parse_disp, parse_reg_mem)
        .preceded(char_pc('['))
        .terminated(char_pc(']'))
        .parse(src)
}

fn parse_reg<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Reg> {
    // First get the register name using mnemonic_name parser
    let (reg_name, rest) = mnemonic_name.parse(src)?;

    // Convert the string to a Reg by attempting to parse it
    let reg: Reg = Reg::from_str(reg_name.inner.to_lowercase().as_str())
        .map_err(|_| ParseError::new(format!("Unknown register: {}", reg_name)))?;

    Ok((reg, rest))
}

fn parse_reg_mem(src: Text) -> ParseResult<Text, RM> {
    parse_reg(src).map(|(r, rest)| (RM::AddrReg(r.bit_width(), r), rest))
}

fn str2imm<'a>(
    f: impl ParsecT<Text<'a>, Token<&'a str>>,
    radix: u32,
) -> impl ParsecT<Text<'a>, Imm> {
    move |input| {
        let (num_str, rest) = f.parse(input)?;
        let value = u64::from_str_radix(num_str.inner, radix).map_err(|err| {
            println!("{err}");
            ParseError::new(format!("{} err num", num_str))
        })?;
        let imm = if value <= u8::MAX as u64 {
            Imm::Imm8(value as u8)
        } else if value <= u16::MAX as u64 {
            Imm::Imm16(value as u16)
        } else if value <= u32::MAX as u64 {
            Imm::Imm32(value as u32)
        } else {
            Imm::Imm64(value)
        };
        Ok((imm, rest))
    }
}

fn parse_imm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Imm> {
    choice!(
        parse_neg_num_to_imm,
        str2imm(parse_hex, 16),
        str2imm(parse_oct, 8),
        str2imm(parse_bin, 2),
        str2imm(parse_dex, 10)
    )
    .parse(src)
}

fn parse_neg_num_to_imm(src: Text) -> ParseResult<Text, Imm> {
    let (num, rest) = parse_neg(src)?;
    let n = str::parse::<i64>(num.inner)
        .map_err(|_| ParseError::new(format!("wrong neg num {num}")))?;
    let imm = if n >= i8::MIN as i64 {
        Imm::Imm8(n as i8 as u8)
    } else if n >= i16::MIN as i64 {
        Imm::Imm16(n as i16 as u16)
    } else if n >= i32::MIN as i64 {
        Imm::Imm32(n as i32 as u32)
    } else {
        Imm::Imm64(n as u64)
    };
    Ok((imm, rest))
}

fn parse_disp<'a>(src: Text<'a>) -> ParseResult<Text<'a>, RM> {
    let (reg, rest) = lexeme(ws, parse_reg).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (imm, rest) = lexeme(ws, parse_imm).parse(rest)?;
    Ok((RM::AddrRegDisp(reg.bit_width(), reg, imm), rest))
}

fn parse_sib<'a>(src: Text<'a>) -> ParseResult<Text<'a>, RM> {
    let (index, rest) = lexeme(ws, parse_reg).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (base, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let scale_opt = lexeme(ws, char_pc('*'))
        .then(lexeme(
            ws,
            char_fn_pc(|ch| ch == '1' || ch == '2' || ch == '4' || ch == '8'),
        ))
        .parse(rest);
    let (scale, rest) = if let Ok((scale_str, rest)) = scale_opt {
        (scale_str.inner.to_digit(10).unwrap() as u8, rest)
    } else {
        (1, rest)
    };

    Ok((RM::AddrSIB(index.bit_width(), index, base, scale), rest))
}

#[derive(Debug)]
pub enum LabelAddr {
    Val(String),
    Addr(String),
    Disp(String, Imm),
}

/// Inst like ```  lea rax, [label] ```
pub fn parse_inst_with_label_addr(src: Text) -> ParseResult<Text, (LabelAddr, Inst)> {
    choice!(
        parse_inst_labeladdr2reg,
        parse_inst_reg2labeladdr,
        parse_inst_label
    )
    .parse(src)
}

fn parse_inst_label(src: Text) -> ParseResult<Text, (LabelAddr, Inst)> {
    let (mnemonic, rest) = mnemonic_name(src)?;
    let (_, rest) = ws(rest)?;
    let (label, rest) = parse_fake_addr(rest)?;
    Ok((
        (
            label,
            Inst {
                mnemonic: mnemonic.inner.to_string(),
                operand: Operand::Imm(Imm::Imm64(0)),
            },
        ),
        rest,
    ))
}

fn parse_inst_reg2labeladdr(src: Text) -> ParseResult<Text, (LabelAddr, Inst)> {
    let (mnemonic, rest) = mnemonic_name.terminated(ws).parse(src)?;

    let (width_opt, rest) = if let Ok((width, rest)) = lexeme(ws, parse_size_define).terminated(ws).parse(rest)
    {
        (Some(width), rest)
    } else {
        (None, rest)
    };

    let (label, rest) = lexeme(ws, parse_fake_addr).terminated(lexeme(ws, char_pc(','))).parse(rest)?;

    let (reg, rest) = lexeme(ws, parse_reg).parse(rest)?;

    let width = if let Some(width) = width_opt {
        width
    } else {
        reg.bit_width()
    };

    Ok((
        (
            label,
            Inst {
                mnemonic: mnemonic.inner.to_string(),
                operand: Operand::Reg2RM {
                    reg,
                    rm: RM::AddrRegDisp(width, Reg::RBP, Imm::Imm8(0)),
                },
            },
        ),
        rest,
    ))
}

fn parse_inst_labeladdr2reg(src: Text) -> ParseResult<Text, (LabelAddr, Inst)> {
    let (mnemonic, rest) = mnemonic_name(src)?;
    let (_, rest) = ws(rest)?;
    let (reg, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (width, rest) = if let Ok((width, rest)) = parse_size_define.terminated(ws).parse(rest) {
        (width, rest)
    } else {
        (reg.bit_width(), rest)
    };
    let (label, rest) = lexeme(ws, parse_fake_addr).parse(rest)?;
    Ok((
        (
            label,
            Inst {
                mnemonic: mnemonic.inner.to_string(),
                operand: Operand::RM2Reg {
                    reg,
                    rm: RM::AddrRegDisp(width, Reg::RSI, Imm::Imm8(0)),
                },
            },
        ),
        rest,
    ))
}

pub fn parse_fake_addr(src: Text) -> ParseResult<Text, LabelAddr> {
    choice!(parse_rm_label, parse_label_val).parse(src)
}

fn parse_label_val(src: Text) -> ParseResult<Text, LabelAddr> {
    lexeme(ws, label_name)
        .parse(src)
        .map(|(x, rest)| (LabelAddr::Val(x), rest))
}

fn parse_rm_label<'a>(src: Text<'a>) -> ParseResult<Text<'a>, LabelAddr> {
    choice!(parse_label_addr, parse_label_disp)
        .preceded(char_pc('['))
        .terminated(char_pc(']'))
        .parse(src)
}

fn parse_label_addr(src: Text) -> ParseResult<Text, LabelAddr> {
    lexeme(ws, label_name)
        .parse(src)
        .map(|(x, rest)| (LabelAddr::Addr(x), rest))
}

fn parse_label_disp(src: Text) -> ParseResult<Text, LabelAddr> {
    let (name, rest) = lexeme(ws, label_name).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (num, rest) = lexeme(ws, parse_imm).parse(rest)?;

    Ok((LabelAddr::Disp(name, num), rest))
}

fn parse_size_define(src: Text) -> ParseResult<Text, AddrWidth> {
    choice!(
        parse_size_define_pc("byte", AddrWidth::B8),
        parse_size_define_pc("word", AddrWidth::B16),
        parse_size_define_pc("dword", AddrWidth::B32),
        parse_size_define_pc("qword", AddrWidth::B64)
    )
    .parse(src)
}

#[inline]
fn parse_size_define_pc<'a>(key: &'a str, width: AddrWidth) -> impl ParsecT<Text<'a>, AddrWidth> {
    move |input| keyworld(key).parse(input).map(|(_, rest)| (width, rest))
}
