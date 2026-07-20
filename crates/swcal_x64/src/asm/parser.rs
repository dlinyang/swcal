use crate::asm::data::*;
use crate::asm::inst::*;
use crate::asm::lexer::*;
use crate::el::*;
use crate::inst::Inst;
use tinyparsec::parsec::*;
use tinyparsec::text::*;
use tinyparsec::*;

pub fn parse(src: &str) -> ParseResult<(), EL> {
    let mut text = Text::new(src, Default::default());
    let mut el = EL::new();
    let mut section = Section::new();
    while text.inner.len() > 0 {
        let (_, rest) = many0(empty_line.or(ws_or_comment)).parse(text)?;

        let line_end = |input| {
            choice!(
                lexeme(ws, newline_or_end),
                lexeme(ws, parse_comment_as_empty)
            )
            .parse(input)
        };

        let (asm, rest) = choice!(
            parse_section_stmt.terminated(line_end),
            parse_align_stmt.terminated(line_end),
            parse_label_stmt.terminated(line_end),
            parse_global_stmt.terminated(line_end),
            parse_data_stmt.terminated(line_end),
            parse_inst_stmt.terminated(line_end)
        )
        .parse(rest)?;

        match asm {
            AsmStmt::Section(section_name) => {
                // eprintln!("section {}", section_name);
                if section.name == None {
                    section.name = Some(section_name)
                } else {
                    let mut new_section = Section::new();
                    new_section.name = Some(section_name);
                    el.sections.push(std::mem::replace(&mut section, new_section));
                }
            }
            AsmStmt::Align(align) => {
                section.data.push(Data::Align(align));
            }
            AsmStmt::Label(name) => {
                section.labels.push((name, section.data.len()));
            }
            AsmStmt::Global(name) => {
                el.globals.push(name);
            }
            AsmStmt::Data(name, data) => {
                section.labels.push((name, section.data.len()));
                section.data.push(data);
            }
            AsmStmt::Inst{inst, label} => {
                section.data.push(Data::Inst(inst));
                if let Some(_label) = label {
                    // section.labels.push(label);
                }
            }
        }
        text = rest;
    }

    el.sections.push(section);

    Ok((el, ()))
}

pub enum AsmStmt {
    Section(String),
    Align(u8),
    Label(String),
    Global(String),
    Data(String, Data),
    Inst{inst: Inst, label: Option<Label>},
}

fn parse_section_stmt<'a>(src: Text<'a>) -> ParseResult<Text<'a>, AsmStmt> {
    let (_, rest) = lexeme(ws, keyworld("section")).terminated(ws).parse(src)?;
    let (name, rest) = section_name(rest)?;
    println!("parse section {name}");

    Ok((AsmStmt::Section(name), rest))
}

fn parse_align_stmt(src: Text) -> ParseResult<Text, AsmStmt> {
    let (_, rest) = lexeme(ws, keyworld("align")).parse(src)?;
    let (_, rest) = many(ws).parse(rest)?;
    let (num_str, rest) = parse_dex(rest)?;
    let num = u8::from_str_radix(num_str.inner, 10).map_err(|_| ParseError::new(format!("wrong align set {num_str}")))?;

    Ok((AsmStmt::Align(num), rest))
}

fn parse_label_stmt(src: Text) -> ParseResult<Text, AsmStmt> {
    let (name, rest) = lexeme(ws, label_name).parse(src)?;
    let (_, rest) = char_pc(':').parse(rest)?;
    Ok((AsmStmt::Label(name), rest))
}

fn parse_global_stmt(src: Text) -> ParseResult<Text, AsmStmt> {
    let (_, rest) = lexeme(ws, str_pc("global")).parse(src)?;
    let (_, rest) = ws(rest)?;
    let (name, rest) = lexeme(ws, label_name).parse(rest)?;
    Ok((AsmStmt::Global(name), rest))
}

fn parse_data_stmt(src: Text) -> ParseResult<Text, AsmStmt> {
    parse_data(src).map(|(x, rest)| (AsmStmt::Data(x.inner.0, x.inner.1), rest))
}

fn parse_inst_stmt(src: Text) -> ParseResult<Text, AsmStmt> {
    parse_inst_label_opt(src).map(|((label, inst),rest)| (AsmStmt::Inst { inst, label }, rest))
}
