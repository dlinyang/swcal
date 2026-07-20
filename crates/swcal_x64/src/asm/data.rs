use crate::el::Data;
use crate::asm::lexer::*;
use tinyparsec::parsec::*;
use tinyparsec::text::*;
use tinyparsec::*;

pub fn parse_data<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<(String, Data)>> {
    let (name, rest) = lexeme(ws, db_name).parse(src)?;
    let (_, rest) = char_pc(' ').parse(rest)?;

    let (data, rest) = choice!(
        parse_data_byte,
        parse_data_word,
        parse_data_double_word,
        parse_data_quadra_word,
        parse_data_res_pc("resb", 1),
        parse_data_res_pc("resw", 2),
        parse_data_res_pc("resd", 4),
        parse_data_res_pc("resq", 8)
    )
    .parse(rest)?;

    Ok((name.zip(data), rest))
}

fn parse_data_byte<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Data>> {
    let (_, rest) = keyworld("db").parse(src)?;
    let (_, rest) = ws(rest)?;
    let (mut data, mut rest) = lexeme(ws, parse_one_db_val).parse(rest)?;
    while let Ok((_, rest_of_comma)) = lexeme(ws, char_pc(',')).parse(rest) {
        let (mut a_data, a_rest) = lexeme(ws, parse_one_db_val).parse(rest_of_comma)?;
        data.inner.append(&mut a_data.inner);
        rest = a_rest;
    }
    Ok((Token::new(Data::RawData(data.inner), data.text_pos), rest))
}

fn parse_one_db_val<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Vec<u8>>> {
    lexeme(ws,
    choice!(
        parse_db_str,
        str2val2byte::<u8>(parse_hex, 16),
        str2val2byte::<u8>(parse_oct, 8),
        str2val2byte::<u8>(parse_bin, 2),
        str2val2byte::<u8>(parse_dex, 10)
    ))
    .parse(src)
}

fn parse_data_word<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Data>> {
    let (_, rest) = lexeme(ws, keyworld("dw")).parse(src)?;
    let (_, rest) = ws(rest)?;
    lexeme(ws,
    choice!(
        str2val2byte::<u16>(parse_hex, 16),
        str2val2byte::<u16>(parse_oct, 8),
        str2val2byte::<u16>(parse_bin, 2),
        str2val2byte::<u16>(parse_dex, 10)
    ))
    .parse(rest).map(|(x, rest)| (Token::new(Data::RawData(x.inner), x.text_pos), rest))
}

fn parse_data_double_word<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Data>> {
    let (_, rest) = lexeme(ws, keyworld("dd")).parse(src)?;
    let (_, rest) = ws(rest)?;
    lexeme(ws,
    choice!(
        str2val2byte::<u32>(parse_hex, 16),
        str2val2byte::<u32>(parse_oct, 8),
        str2val2byte::<u32>(parse_bin, 2),
        str2val2byte::<u32>(parse_dex, 10)
    ))
    .parse(rest).map(|(x, rest)| (Token::new(Data::RawData(x.inner), x.text_pos), rest))
}

fn parse_data_quadra_word<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Data>> {
    let (_, rest) = lexeme(ws, keyworld("dq")).parse(src)?;
    let (_, rest) = ws(rest)?;
    lexeme(ws,
    choice!(
        str2val2byte::<u64>(parse_hex, 16),
        str2val2byte::<u64>(parse_oct, 8),
        str2val2byte::<u64>(parse_bin, 2),
        str2val2byte::<u64>(parse_dex, 10)
    ))
    .parse(rest).map(|(x, rest)| (Token::new(Data::RawData(x.inner), x.text_pos), rest))
}

fn parse_data_res<'a>(src: Text<'a>, res_type: &'a str, type_size: u64) -> ParseResult<Text<'a>, Token<u64>> {
    let (_, rest) = lexeme(ws, str_pc(res_type)).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (num, rest) = lexeme(ws, parse_dex).parse(rest)?;
    let res_len = u64::from_str_radix(num.inner, 10).map_err(|_| Default::default())?;
    Ok((Token::new(res_len * type_size, src.text_pos), rest))
}

fn parse_data_res_pc<'a>(res_type: &'a str, type_size: u64) -> impl ParsecT<Text<'a>, Token<Data>> {
    move |src| {
        parse_data_res(src, res_type, type_size).map(|(x, rest)| (Token::new(Data::Res(x.inner), x.text_pos),rest))
    }
}

#[inline]
fn parse_db_str<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<Vec<u8>>> {
    parse_string_literal
        .parse(src)
        .map(|(s, rest)| (s.covert(|s| s.as_bytes().to_vec()), rest))
}

pub fn parse_dex<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    str_fn_pc(|str| {
        str.find(|ch: char| !ch.is_ascii_digit())
            .unwrap_or_default()
    })
    .parse(src)
}

pub fn parse_signed<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    str_fn_pc(|str|{
        if let Some(ch) = str.chars().next() && (ch.is_numeric() || ch == '-') {
            str[1..].find(|ch: char| !ch.is_ascii_digit()).unwrap_or_default() + 1
        }
        else { 0 }
    }).parse(src)
}

pub fn parse_hex<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    let (_, rest) = str_pc("0x").parse(src)?;
    str_fn_pc(|str| {
        str.find(|ch: char| !ch.is_ascii_hexdigit())
            .unwrap_or_default()
    })
    .parse(rest)
}

pub fn parse_oct<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    let (_, rest) = str_pc("0b").parse(src)?;
    str_fn_pc(|str| {
        str.find(|ch: char| ch < '0' || ch > '8')
            .unwrap_or_default()
    })
    .parse(rest)
}

pub fn parse_bin<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    let (_, rest) = str_pc("0b").parse(src)?;
    str_fn_pc(|str| {
        str.find(|ch: char| ch != '1' || ch != '0')
            .unwrap_or_default()
    })
    .parse(rest)
}

trait FromStrRadixBytes: Sized {
    fn from_str_radix_bytes(s: &str, radix: u32) -> Result<Vec<u8>, ()>;
}

impl FromStrRadixBytes for u8 {
    fn from_str_radix_bytes(s: &str, radix: u32) -> Result<Vec<u8>, ()> {
        u8::from_str_radix(s, radix)
            .map(|v| v.to_be_bytes().to_vec())
            .map_err(|_| ())
    }
}

impl FromStrRadixBytes for u16 {
    fn from_str_radix_bytes(s: &str, radix: u32) -> Result<Vec<u8>, ()> {
        u16::from_str_radix(s, radix)
            .map(|v| v.to_be_bytes().to_vec())
            .map_err(|_| ())
    }
}

impl FromStrRadixBytes for u32 {
    fn from_str_radix_bytes(s: &str, radix: u32) -> Result<Vec<u8>, ()> {
        u32::from_str_radix(s, radix)
            .map(|v| v.to_be_bytes().to_vec())
            .map_err(|_| ())
    }
}

impl FromStrRadixBytes for u64 {
    fn from_str_radix_bytes(s: &str, radix: u32) -> Result<Vec<u8>, ()> {
        u64::from_str_radix(s, radix)
            .map(|v| v.to_be_bytes().to_vec())
            .map_err(|_| ())
    }
}

fn str2val2byte<'a, T: FromStrRadixBytes>(
    f: impl ParsecT<Text<'a>, Token<&'a str>>,
    radix: u32,
) -> impl ParsecT<Text<'a>, Token<Vec<u8>>> {
    move |input| {
        f.parse(input)
            .and_then(|(t, rest)| match T::from_str_radix_bytes(t.inner, radix) {
                Ok(bytes) => Ok((Token::new(bytes, t.text_pos), rest)),
                Err(_) => Err(ParseError::new(format!("failed to parse number {}", t))),
            })
    }
}
