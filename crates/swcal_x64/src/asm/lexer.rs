use swcal_parsec::*;
use swcal_parsec::parsec::*;
use swcal_parsec::text::*;

#[inline]
pub fn newline_or_end<'a>(src: Text<'a>) -> ParseResult<Text<'a>, ()> {
    choice!(newline, end).parse(src)
}

#[inline]
pub fn ws<'a>(src: Text<'a>)  -> ParseResult<Text<'a>, ()> {
    consume_pc(|c| c == ' ' || c == '\t' || c == '\r').parse(src)
}

#[inline]
pub fn empty_line<'a>(src: Text<'a>) ->  ParseResult<Text<'a>, ()> {
    many0(ws).terminated(newline_or_end).parse(src).map(|(_, rest)| ((), rest))
}

#[inline]
pub fn keyworld<'a>(key: &'a str) -> impl ParsecT<Text<'a>, Token<&'a str>> {
    lexeme(ws, str_pc(key))
}

#[inline]
pub fn db_name<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<String>> {
    let (head, rest) = alphabetic.or(char_pc('_')).parse(src)?;
    let (tail, rest) = many0(alphanum.or(char_pc('_'))).parse(rest)?;
    let mut ret = String::new();
    ret.push(head.inner);
    for ch in tail {
        ret.push(ch.inner);
    }
    Ok((Token::new(ret, src.text_pos), rest))
}

#[inline]
pub fn section_name<'a>(src: Text<'a>) -> ParseResult<Text<'a>, String> {
    let (head, rest) = char_fn_pc(|x| x.is_alphabetic() || x == '.' || x == '_').parse(src)?;
    let (tail, rest) = many0(char_fn_pc(|x| x.is_alphanumeric() || x == '.' || x == '_')).parse(rest)?;
    let mut ret = String::new();
    ret.push(head.inner);
    for ch in tail {
        ret.push(ch.inner);
    }
    Ok((ret, rest))
}

#[inline]
pub fn label_name<'a>(src: Text<'a>) -> ParseResult<Text<'a>, String> {
    let (head, rest) = char_fn_pc(|x| x.is_alphabetic() || x == '_').parse(src)?;
    let (tail, rest) = many0(char_fn_pc(|x| x.is_alphanumeric() || x == '.' || x == '_' || x == '@')).parse(rest)?;
    let mut ret = String::new();
    ret.push(head.inner);
    for ch in tail {
        ret.push(ch.inner);
    }
    Ok((ret, rest))
}

/// string literal bewteen " or '
pub fn parse_string_literal<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    char_fn_pc(|ch| ch == '\''  || ch == '"')
        .then(str_fn_pc(
            |s| s.find(|ch| ch == '\'' || ch == '"').unwrap_or_default()
        ))
        .terminated(char_fn_pc(|ch| ch == '\'' || ch == '"'))
        .parse(src)
}

pub fn mnemonic_name<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    str_fn_pc(|x| x.find(|ch: char| !ch.is_ascii_alphabetic()).unwrap_or_default()).parse(src)
}

pub fn parse_comment<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    char_pc(';')
        .then(
            str_fn_pc(|x| x.find('\n').unwrap_or_default())
        )
        .terminated(newline_or_end)
        .parse(src)
}

pub fn ws_or_comment<'a>(src: Text<'a>) -> ParseResult<Text<'a>, ()> {
    ws.or(parse_comment_as_empty).parse(src)
}

pub fn parse_comment_as_empty(src: Text) -> ParseResult<Text, ()> {
    parse_comment(src).map(|(_, rest)| ((), rest))
}
