use crate::asm::data::parse_data;
use crate::asm::inst::parse_inst;
use crate::el::Data;
use crate::el::EL;
use crate::el::Section;
use crate::inst::Inst;
use swcal_parsec::*;
use swcal_parsec::parsec::*;
use swcal_parsec::text::*;
use crate::asm::lexer::*;

pub fn parse(src: &str) -> ParseResult<(), EL> {
    let mut text = Text::new(src, Default::default());
    let mut el = EL::new();
    let mut section = Section::new();
    while text.inner.len() > 0 {
        let (_, rest) = many0(empty_line.or(ws_or_comment)).parse(text)?;

        eprint!("stmt start {}: ", rest.text_pos);
        let (asm, rest) = choice!(
            parse_section_stmt,
            parse_label_stmt,
            parse_global_stmt,
            parse_data_stmt,
            parse_inst_stmt
        )
        .parse(rest)?;


        let (_, rest) = choice!(
            lexeme(ws, newline_or_end),
            lexeme(ws, parse_comment_as_empty)
        ).parse(rest)?;

        match asm {
            AsmStmt::Section(section_name) => {
                eprintln!("section {}", section_name);
                if section.name == None {
                    section.name = Some(section_name)
                }
                else {
                    el.sections.push(std::mem::replace(&mut section, Section::new()));
                }
            }
            AsmStmt::Label(name) => {
                eprintln!("{name}:");
                section.labels.push((name, section.data.len()));
            }
            AsmStmt::Global(name) => {
                eprintln!("global {name}");
                el.globals.push(name);
            }
            AsmStmt::Data(name, data) => {
                eprintln!("{name} {data:?}");
                section.labels.push((name, section.data.len()));
                section.data.push(data);
            },
            AsmStmt::Inst(inst) => {
                eprintln!("{inst}");
                section.data.push(Data::Inst(inst));
            }
        }
        text = rest;
    }
    Ok((el, ()))
}

pub enum AsmStmt {
    Section(String),
    Label(String),
    Global(String),
    Data(String,Data),
    Inst(Inst),
}

fn parse_section_stmt<'a>(src: Text<'a>) -> ParseResult<Text<'a>, AsmStmt> {
    let (_, rest) = lexeme(ws, keyworld("section")).parse(src)?;
    let (_, rest) = many(ws).parse(rest)?;
    let (name, rest) = section_name(rest)?;

    Ok((AsmStmt::Section(name),rest))
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
    parse_inst(src).map(|(x, rest)| (AsmStmt::Inst(x), rest))
}
