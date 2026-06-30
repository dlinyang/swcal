use crate::asm::lexer::*;
use crate::asm::data::*;
use crate::inst::*;
use swcal_parsec::choice;
use swcal_parsec::parsec::*;
use swcal_parsec::text::*;

pub fn parse_inst_oprand_zero<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = lexeme(ws, newline_or_end).parse(rest)?;
    Ok((
        Inst {
            mnemonic: mnemonic_name.to_string(),
            operand: Operand::Zero,
        },
        rest,
    ))
}

pub fn parse_inst_oprand_one<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = lexeme(ws, newline_or_end).parse(rest)?;
    Ok((
        Inst {
            mnemonic: mnemonic_name.to_string(),
            operand: Operand::Zero,
        },
        rest,
    ))
}

pub fn parse_inst_oprand_two<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Inst> {
    let (mnemonic_name, rest) = lexeme(ws, mnemonic_name).parse(src)?;
    let (_, rest) = lexeme(ws, newline_or_end).parse(rest)?;
    Ok((
        Inst {
            mnemonic: mnemonic_name.to_string(),
            operand: Operand::Zero,
        },
        rest,
    ))
}

fn parse_mem<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    mnemonic_name
        .preceded(char_pc('['))
        .terminated(char_pc(']'))
        .parse(src)
}

fn parse_reg<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Reg>> {
    // First get the register name using mnemonic_name parser
    let (reg_name, rest) = mnemonic_name.parse(src)?;

    // Convert the string to a Reg by attempting to parse it
    let reg = match reg_name.inner.to_lowercase().as_str() {
        "al" => Reg::AL,
        "cl" => Reg::CL,
        "dl" => Reg::DL,
        "bl" => Reg::BL,
        "ah" => Reg::AH,
        "ch" => Reg::CH,
        "dh" => Reg::DH,
        "bh" => Reg::BH,
        "ax" => Reg::AX,
        "cx" => Reg::CX,
        "dx" => Reg::DX,
        "bx" => Reg::BX,
        "sp" => Reg::SP,
        "bp" => Reg::BP,
        "si" => Reg::SI,
        "di" => Reg::DI,
        "eax" => Reg::EAX,
        "ecx" => Reg::ECX,
        "edx" => Reg::EDX,
        "ebx" => Reg::EBX,
        "esp" => Reg::ESP,
        "ebp" => Reg::EBP,
        "esi" => Reg::ESI,
        "edi" => Reg::EDI,
        "rax" => Reg::RAX,
        "rcx" => Reg::RCX,
        "rdx" => Reg::RDX,
        "rbx" => Reg::RBX,
        "rsp" => Reg::RSP,
        "rbp" => Reg::RBP,
        "rsi" => Reg::RSI,
        "rdi" => Reg::RDI,
        "r8" => Reg::R8,
        "r9" => Reg::R9,
        "r10" => Reg::R10,
        "r11" => Reg::R11,
        "r12" => Reg::R12,
        "r13" => Reg::R13,
        "r14" => Reg::R14,
        "r15" => Reg::R15,
        _ => {
            return Err(ParseError::new(
                format!("Unknown register: {}", reg_name),
            ));
        }
    };

    Ok((
        Token {
            inner: reg,
            text_pos: reg_name.text_pos,
        },
        rest,
    ))
}

fn str2imm<'a>(f: impl ParsecT<Text<'a>, Token<&'a str>>, radix: u32) -> impl ParsecT<Text<'a>, Token<Imm>> {
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
        Ok((Token::new(imm, num_str.text_pos), rest))
    }
}

fn parse_imm<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Imm>> {
    choice!(
        str2imm(parse_hex, 16),
        str2imm(parse_oct, 8),
        str2imm(parse_bin, 2),
        str2imm(parse_dex, 10)
    ).parse(src)
}

fn parse_disp<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<RM>> {
    let (reg, rest) = lexeme(ws, parse_reg).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (imm, rest) = lexeme(ws, parse_imm).parse(rest)?;
    Ok((
        Token::new(RM::AddrRegDisp(reg.inner, imm.inner), reg.text_pos),
        rest,
    ))
}

fn parse_sib<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<RM>> {
    let (index, rest) = lexeme(ws, parse_reg).parse(src)?;
    let (_, rest) = lexeme(ws, char_pc('+')).parse(rest)?;
    let (base, rest) = lexeme(ws, parse_reg).parse(rest)?;
    let (scale, rest) = lexeme(ws, char_fn_pc(|ch| ch == '1' || ch == '2' || ch == '4' || ch == '8')).parse(rest)?;
    Ok((
        Token::new(RM::AddrSIB(index.inner, base.inner, scale.inner.to_digit(10).unwrap() as u8), index.text_pos),
        rest,
    ))
}
