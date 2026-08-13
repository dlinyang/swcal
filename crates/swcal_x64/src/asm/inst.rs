use crate::asm::data::*;
use crate::asm::lexer::*;
use crate::inst::{*, base::*, operand::*, reg::*, mem::*, disp::*, imm::*};
use std::str::FromStr;
use tinyparsec::choice;
use tinyparsec::parsec::*;
use tinyparsec::text::*;

/// Note: this is not fast way to impl parse inst
pub fn parse_inst_label_opt(src: Text) -> ParseResult<Text, (Option<Label>, AsmInst)> {
    let (mnemonic_tok, rest) = lexeme(ws, mnemonic_name).terminated(ws).parse(src)?;

    let (label_dst_opt, rest) = parse_operand_label_opt
        .optional(rest);

    let (label_src_opt, rest) = parse_operand_label_opt
        .preceded(lexeme(ws, char_pc(',')))
        .optional(rest);

    let (label_src_ext_opt, rest) = parse_operand_label_opt
        .preceded(lexeme(ws, char_pc(',')))
        .optional(rest);

    let (dlabel, dst) = if let Some((label, dst)) = label_dst_opt {
        (label, Some(dst))
    } else {
        (None, None)
    };

    let (slabel, src) = if let Some((label, src)) = label_src_opt {
        (label, Some(src))
    } else {
        (None, None)
    };

    let (selabel, src_ext) = if let Some((label, src_ext)) = label_src_ext_opt {
        (label, Some(src_ext))
    } else {
        (None, None)
    };

    // TODO: fix label's mem access's width
    let (dst, src, src_ext) = match (&dlabel, &slabel, &selabel) {
        (None, None, Some(_)) => {
            todo!()
        },
        (None, Some(_), None) => {
            let src = if let Some(Operand::Mem { width:_, mem }) = src {
                Some(Operand::Mem { width: dst.unwrap().width(), mem })
            }
            else {
                src
            };
            (dst, src, src_ext)
        },
        (Some(_), None, None) => {
            let dst = if let Some(Operand::Mem { width:_, mem }) = dst {
                Some(Operand::Mem { width: src.unwrap().width(), mem })
            }
            else {
                dst
            };
            (dst, src, src_ext)
        },
        _ => (dst, src, src_ext),
    };

    let label = match (dlabel, slabel, selabel) {
        (None, None, None) => None,
        (None, None, Some(label)) => Some(label),
        (None, Some(label), None) => Some(label),
        (Some(label), None, None) => Some(label),
        _ => panic!("multi label in instruction"),
    };

    Ok((
        (
            label,
            AsmInst {
                mnemonic: mnemonic_tok.inner.to_string(),
                dst,
                src,
                src_ext,
            },
        ),
        rest,
    ))
}

fn parse_operand(src: Text) -> ParseResult<Text, Operand> {
    choice!(parse_operand_mem, parse_operand_reg, parse_imm).parse(src)
}

fn parse_operand_mem(src: Text) -> ParseResult<Text, Operand> {
    let (width_opt, rest) = lexeme(ws, parse_size_define).optional(src);
    let (mem, rest) = parse_mem(rest)?;
    Ok((Operand::Mem{width: width_opt.unwrap_or(0) ,mem}, rest))
}

fn parse_mem(src: Text) -> ParseResult<Text, Mem> {
    choice!(
        parse_mem_index,
        parse_mem_reg,
        parse_mem_rip_disp
    ).parse(src)
}

fn parse_mem_reg(src: Text) -> ParseResult<Text, Mem> {
    let (_, rest) = lexeme(ws, char_pc('[')).parse(src)?;
    let (reg, rest) = parse_reg(rest)?;
    let (disp, rest) = lexeme(ws, char_pc('+')).then(parse_disp).optional(rest);
    let (_, rest) = lexeme(ws, char_pc(']')).parse(rest)?;
    // check reg
    Ok((Mem::Mem { reg, disp }, rest))
}

fn parse_mem_index(src: Text) -> ParseResult<Text, Mem> {
    let (_, rest) = lexeme(ws, char_pc('[')).parse(src)?;
    let (base, rest) = parse_reg(rest)?;
    let ((index, scale), rest) = lexeme(ws, char_pc('+'))
        .then(parse_index_mul_scale.or(parse_index_only))
        .parse(rest)?;
    let (disp, rest) = lexeme(ws, char_pc('+')).then(parse_disp).optional(rest);
    let (_, rest) = lexeme(ws, char_pc(']')).parse(rest)?;
    // check reg
    Ok((Mem::Index { base, index, scale, disp }, rest))
}

fn parse_mem_rip_disp(src: Text) -> ParseResult<Text, Mem> {
    let (_, rest) = lexeme(ws, char_pc('[')).parse(src)?;
    let (disp32, rest) = parse_disp32(rest)?;
    let (_, rest) = lexeme(ws, char_pc(']')).parse(rest)?;
    // check reg
    Ok((Mem::RIPDisp { disp32}, rest))
}

fn parse_operand_reg(src: Text) -> ParseResult<Text, Operand> {
    let (reg, rest) = parse_reg(src)?;
    Ok((Operand::Reg(reg), rest))
}

fn parse_reg<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Reg> {
    let (reg_name, rest) = lexeme(ws, reg_name).parse(src)?;

    let reg: Reg = Reg::from_str(reg_name.inner.to_lowercase().as_str())
        .map_err(|_| ParseError::new(format!("Unknown register: {}", reg_name)))?;

    Ok((reg, rest))
}

fn str2imm<'a>(
    f: impl ParsecT<Text<'a>, Token<&'a str>>,
    radix: u32,
) -> impl ParsecT<Text<'a>, Operand> {
    move |input| {
        let (num_str, rest) = f.parse(input)?;
        let value = u64::from_str_radix(num_str.inner, radix).map_err(|err| {
            println!("{err}");
            ParseError::new(format!("{} err num", num_str))
        })?;
        let imm = if value <= u8::MAX as u64 {
            Operand::Imm(Imm::Imm8(value as u8))
        } else if value <= u16::MAX as u64 {
            Operand::Imm(Imm::Imm16(value as u16))
        } else if value <= u32::MAX as u64 {
            Operand::Imm(Imm::Imm32(value as u32))
        } else {
            Operand::Imm(Imm::Imm64(value))
        };
        Ok((imm, rest))
    }
}

fn parse_imm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Operand> {
    lexeme(ws, choice!(
        str2imm(parse_hex, 16),
        str2imm(parse_oct, 8),
        str2imm(parse_bin, 2),
        str2imm(parse_dex, 10),
        parse_signed_num_to_imm
    ))
    .parse(src)
}

fn parse_signed_num_to_imm(src: Text) -> ParseResult<Text, Operand> {
    let (num, rest) = parse_signed(src)?;
    let n = str::parse::<i64>(num.inner)
        .map_err(|_| ParseError::new(format!("wrong neg num {num}")))?;
    let imm = if n >= i8::MIN as i64 && n <= i8::MAX as i64{
        Imm::Imm8(n as i8 as u8)
    } else if n >= i16::MIN as i64 && n <= i16::MAX as i64{
        Imm::Imm16(n as i16 as u16)
    } else if n >= i32::MIN as i64 && n <= i32::MAX as i64{
        Imm::Imm32(n as i32 as u32)
    } else {
        Imm::Imm64(n as u64)
    };
    Ok((Operand::Imm(imm), rest))
}

fn parse_disp<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Disp> {
    let (num, rest) = lexeme(ws, parse_signed).parse(src)?;

    let n = str::parse::<i32>(num.inner)
        .map_err(|_| ParseError::new(format!("wrong neg num {num}")))?;

    if n >= i8::MIN as i32 && n < i8::MAX as i32{
        Ok((Disp::Disp8(n as i8), rest))
    } else if n >= i32::MIN && n <= i32::MAX {
        Ok((Disp::Disp32(n), rest))
    } else {
        Err(ParseError::new("wrong displace number"))
    }
}

fn parse_scale(src: Text) -> ParseResult<Text, u8> {
    lexeme(
        ws,
        char_fn_pc(|ch| ch == '1' || ch == '2' || ch == '4' || ch == '8'),
    )
    .parse(src)
    .map(|(ch, rest)| ((ch.inner as u8) - ('0' as u8), rest))
}

fn parse_index_mul_scale(src: Text) -> ParseResult<Text, (Reg, u8)> {
    parse_reg
        .terminated(lexeme(ws, char_pc('*')))
        .and(parse_scale)
        .parse(src)
}

fn parse_index_only(src: Text) -> ParseResult<Text, (Reg, u8)> {
    parse_reg(src).map(|(reg, rest)| ((reg, 1), rest))
}

fn parse_size_define(src: Text) -> ParseResult<Text, u16> {
    choice!(
        parse_size_define_pc("byte", 8),
        parse_size_define_pc("word", 16),
        parse_size_define_pc("dword", 32),
        parse_size_define_pc("qword", 64)
    )
    .parse(src)
}

#[inline]
fn parse_size_define_pc<'a>(key: &'a str, width: u16) -> impl ParsecT<Text<'a>, u16> {
    move |input| keyworld(key).parse(input).map(|(_, rest)| (width, rest))
}

// /// Inst like ```  lea rax, [label] ```
// pub fn parse_inst_with_label(src: Text) -> ParseResult<Text, (Label, Inst)> {
//     let (mnemonic_tok, rest) = lexeme(ws, mnemonic_name).terminated(ws).parse(src)?;
//     let (dst, rest) = parse_operand
//         .terminated(lexeme(ws, char_pc(',')))
//         .optional(rest);
//     let (src, rest) = parse_operand.optional(rest);
//     let (src_ext, rest) = parse_operand
//         .preceded(lexeme(ws, char_pc(',')))
//         .optional(rest);
//     Ok((
//         (
//             Label::Addr{ name: "f".to_string(), disp: todo!()},
//             Inst {
//                 mnemonic: mnemonic_tok.inner.to_string(),
//                 dst,
//                 src,
//                 src_ext,
//             },
//         ),
//         rest,
//     ))
// }

fn parse_operand_label_opt(src: Text) -> ParseResult<Text, (Option<Label>, Operand)> {
    choice!(parse_operand_without_label, parse_operand_with_label).parse(src)
}

fn parse_operand_without_label(src: Text) -> ParseResult<Text, (Option<Label>, Operand)> {
    let (operand, rest) = parse_operand(src)?;
    Ok(((None, operand), rest))
}

fn parse_operand_with_label(src: Text) -> ParseResult<Text, (Option<Label>, Operand)> {
    let (label_op, rest) = choice!(parse_label_mem, parse_label_address).parse(src)?;

    let (label, operand) = label_op;
    Ok(((Some(label), operand), rest))
}

fn parse_label_address(src: Text) -> ParseResult<Text, (Label, Operand)> {
    let (name, rest) = lexeme(ws, label_name).parse(src)?;
    let (disp, rest) = parse_disp32.optional(rest);
    let disp = disp.unwrap_or(0);
    let label = Label::Addr{name, disp};
    let operand = Operand::Label;
    Ok(((label, operand), rest))
}

fn parse_label_mem(src: Text) -> ParseResult<Text, (Label, Operand)> {
    let (width_opt, rest) = lexeme(ws, parse_size_define).optional(src);
    let (_, rest) = lexeme(ws, char_pc('[')).parse(rest)?;
    let (label, rest) = lexeme(ws, label_name).parse(rest)?;
    let (disp, rest) = parse_disp32.optional(rest);
    let disp = disp.unwrap_or(0);
    let (_, rest) = lexeme(ws, char_pc(']')).parse(rest)?;
    Ok((
        (Label::Mem { name: label, disp }, Operand::Mem { width: width_opt.unwrap_or(0), mem: Mem::RIPDisp { disp32:0 } }),
        rest
    ))
}

fn parse_disp32(src: Text) -> ParseResult<Text, i32> {
    let (s, rest) = choice!(lexeme(ws, char_pc('+')), lexeme(ws, char_pc('-'))).parse(src)?;
    let (num, rest) = lexeme(ws, parse_dex).parse(rest)?;
    let val = str::parse::<i32>(num.inner)
        .map_err(|_| ParseError::new(format!("unkown label displacement: {num}")))?;
    let val = if s.inner == '-' { -val } else { val };
    Ok((val, rest))
}
