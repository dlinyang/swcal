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
        parse_inst_rm2reg,
        parse_inst_imm2reg,
        parse_inst_reg2reg,
        parse_inst_rm,
        parse_inst_imm,
        parse_inst_reg,
        parse_inst_oprand_zero
    ).parse(src)
}

pub fn parse_inst_oprand_zero<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = lexeme(ws, newline_or_end).parse(rest)?;
    Ok((
            Inst {
                mnemonic: mnemonic_name.inner.to_string(),
                operand: Operand::Zero,
            },
        rest,
    ))
}

pub fn parse_inst_reg<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (reg, rest) = lexeme(ws, parse_reg).parse(rest)?;
    Ok((
            Inst {
                mnemonic: mnemonic_name.inner.to_string(),
                operand: Operand::Reg(reg),
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
                operand: Operand::ModRM(rm),
            },
        rest,
    ))
}

pub fn parse_inst_imm2reg<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (reg, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (imm, rest) = lexeme(ws, parse_imm).parse(rest)?;
    Ok((
            Inst {
                mnemonic: mnemonic_name.inner.to_string(),
                operand: Operand::Imm2Reg { reg: reg, imm: imm }
            },
        rest,
    ))
}

pub fn parse_inst_reg2reg<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (dst_reg, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let (_, rest) = lexeme(ws, char_pc(',')).parse(rest)?;
    let (src_reg, rest) = lexeme(ws, parse_reg).parse(rest)?;

    Ok((
            Inst {
                mnemonic: mnemonic_name.inner.to_string(),
                operand: Operand::Reg2Reg { dst_reg, src_reg }
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

fn parse_rm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, RM> {
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

    Ok((
        reg,
        rest,
    ))
}

fn parse_reg_mem(src: Text) -> ParseResult<Text, RM> {
    parse_reg(src).map(|(r, rest)| (RM::AddrReg(r), rest))
}

fn str2imm<'a>(
    f: impl ParsecT<Text<'a>, Token<&'a str>>,
    radix: u32,
) -> impl ParsecT<Text<'a>, Imm> {
    move |input| {
        let (num_str, rest) = f.parse(input)?;
        let value = u64::from_str_radix(num_str.inner, radix).map_err(|_| Default::default())?;
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
        str2imm(parse_hex, 16),
        str2imm(parse_oct, 8),
        str2imm(parse_bin, 2),
        str2imm(parse_dex, 10)
    )
    .parse(src)
}

fn parse_disp<'a>(src: Text<'a>) -> ParseResult<Text<'a>, RM> {
    let (reg, rest) = lexeme(ws, parse_reg).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (imm, rest) = lexeme(ws, parse_imm).parse(rest)?;
    Ok((
        RM::AddrRegDisp(reg, imm),
        rest,
    ))
}

fn parse_sib<'a>(src: Text<'a>) -> ParseResult<Text<'a>, RM> {
    let (index, rest) = lexeme(ws, parse_reg).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (base, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let (scale, rest) = lexeme(
        ws,
        char_fn_pc(|ch| ch == '1' || ch == '2' || ch == '4' || ch == '8'),
    )
    .parse(rest)?;
    Ok((
            RM::AddrSIB(
                index,
                base,
                scale.inner.to_digit(10).unwrap() as u8,
            ),
        rest,
    ))
}

pub enum FakeAddr {
    Val(String),
    Addr(String),
    Disp(String, Imm),
}

pub fn parse_inst_stmt_with_fake_addr(src: Text) -> ParseResult<Text, (FakeAddr, Inst)> {
    todo!()
}

pub fn parse_inst_stmt_label(src: Text) -> ParseResult<Text, (FakeAddr, Inst)> {
    todo!()
}

pub fn parse_fake_addr(src: Text) -> ParseResult<Text, FakeAddr> {
    choice!(
        parse_rm_label,
        parse_label_val
    ).parse(src)
}

fn parse_label_val(src: Text) -> ParseResult<Text, FakeAddr> {
    lexeme(ws, label_name).parse(src).map(|(x, rest)| (FakeAddr::Val(x), rest))
}

fn parse_rm_label<'a>(src: Text<'a>) -> ParseResult<Text<'a>, FakeAddr> {
    choice!(parse_label_addr, parse_label_disp)
        .preceded(char_pc('['))
        .terminated(char_pc(']'))
        .parse(src)
}

fn parse_label_addr(src: Text) -> ParseResult<Text, FakeAddr> {
    lexeme(ws, label_name).parse(src).map(|(x, rest)| (FakeAddr::Addr(x), rest))
}

fn parse_label_disp(src: Text) -> ParseResult<Text, FakeAddr> {
    let (name, rest) = lexeme(ws, label_name).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (num, rest) = lexeme(ws, parse_imm).parse(rest)?;

    Ok((FakeAddr::Disp(name, num), rest))
}
