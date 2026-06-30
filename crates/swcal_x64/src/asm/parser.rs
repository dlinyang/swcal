use crate::asm::data::parse_data;
use crate::el::Section;
use crate::inst::Inst;
use swcal_parsec::parsec::*;
use swcal_parsec::text::*;
use crate::asm::lexer::*;

pub fn parse(src: &str) -> Vec<Inst> {
    let text = Text::new(src, Default::default());
    while let Ok((_, text)) = many0(empty_line.or(|src| parse_comment.parse(src).map(|(_, rest)| ((), rest)))).parse(text) {
        let _s = parse_section(text);
        panic!()
        // if let Ok((inst, rest)) = parse_stmt(text) {

        // }
    }
    vec![]
}

pub fn parse_comment<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Token<&'a str>> {
    lexeme(ws, char_pc(';'))
        .then(
            str_fn_pc(|x| x.find('\n').unwrap_or_default())
        )
        .terminated(newline_or_end)
        .parse(src)
}

fn parse_section<'a>(src: Text<'a>) -> ParseResult<Text<'a>, Section> {
    eprintln!("parse section");
    let mut section = Section::new();

    let mut rest = if let Ok((_, rest)) = keyworld("section").parse(src) {
        let (_, rest) = many(ws).parse(rest)?;
        let (name, rest) = section_name(rest)?;
        let (_, rest) = lexeme(ws, newline).parse(rest)?;
        section.name = name;
        rest
    } else {
        src
    };

    eprintln!("parse data");
    while let Ok((_, el_cm_rest)) = many0(empty_line.or(|src| parse_comment.parse(src).map(|(_, el_cm_rest)| ((), el_cm_rest)))).parse(rest) {
        let (data, stmt_rest) = parse_data(el_cm_rest)?;
        rest = stmt_rest;
        eprintln!("data name: {}", data.inner.0);
        // eprintln!("data: {:#?}", data.inner.1);
        // eprintln!("data length: {}", data.inner.1.len());
    }
    //parse

    Ok((section, src))
}
/// 解析一行 x86_64 汇编指令文本，返回对应的 Inst
/// 支持的格式示例：
/// - `mov rax, rbx` → Reg2Reg
/// - `mov rax, 42` → Imm2Reg
/// - `mov rax, [rbx]` → RM2Reg
/// - `mov rax, [rbx+4]` → RM2Reg with disp
/// - `mov rax, [rbx+rcx*4]` → RM2Reg with SIB
/// - `mov [rbx], rax` → Reg2RM
/// - `mov [rbx+4], rax` → Reg2RM with disp
/// - `mov [rbx+rcx*4], rax` → Reg2RM with SIB
/// - `ret` → Zero
/// - `nop` → Zero
/// - `add rax, rbx` → Reg2Reg
/// - `sub rax, 10` → Imm2Reg
/// - `xor eax, eax` → Reg2Reg
pub fn test() { todo!() }
// pub fn parse_line(line: &str) -> Result<Inst, String> {
//     let line = line.trim();
//     if line.is_empty() {
//         return Err("empty line".to_string());
//     }

//     //comment

//     // 移除注释 (; 或 #)
//     let line = if let Some(pos) = line.find(|c: char| c == ';' || c == '#') {
//         &line[..pos]
//     } else {
//         line
//     };
//     let line = line.trim();
//     if line.is_empty() {
//         return Err("empty line after removing comments".to_string());
//     }

//     // 按空格或制表符分割，但保留括号内的内容
//     let tokens = tokenize(line)?;
//     if tokens.is_empty() {
//         return Err("no tokens".to_string());
//     }

//     let mnemonic = tokens[0].to_lowercase();
//     let mnemonic = mnemonic.as_str();

//     if tokens.len() == 1 {
//         // 无操作数指令: ret, nop, etc.
//         return Ok(Inst {
//             mnemonic: mnemonic.to_string(),
//             operand: Operand::Zero,
//         });
//     }

//     // 第二个 token 应为逗号分隔的操作数列表
//     // 合并剩余 tokens 并分割逗号
//     let operands_str = tokens[1..].join(" ");
//     let parts: Vec<&str> = operands_str.split(',')
//         .map(|s| s.trim())
//         .filter(|s| !s.is_empty())
//         .collect();

//     if parts.len() != 2 {
//         return Err(format!("expected 2 operands, got {}: {}", parts.len(), operands_str));
//     }

//     let dst_raw = parts[0].trim();
//     let src_raw = parts[1].trim();

//     let (is_dst_mem, dst_reg, dst_rm) = parse_operand(dst_raw)?;
//     let (is_src_mem, src_reg, src_rm) = parse_operand(src_raw)?;

//     match (is_dst_mem, is_src_mem, dst_reg, src_reg, dst_rm, src_rm) {
//         // 两个都是寄存器: Reg2Reg
//         (false, false, Some(dst), Some(src), None, None) => {
//             Ok(Inst {
//                 mnemonic: mnemonic.to_string(),
//                 operand: Operand::Reg2Reg {
//                     src_reg: src,
//                     dst_reg: dst,
//                 },
//             })
//         }
//         // dst 是寄存器, src 是立即数: Imm2Reg
//         (false, false, Some(reg), None, None, Some(imm)) => {
//             Ok(Inst {
//                 mnemonic: mnemonic.to_string(),
//                 operand: Operand::Imm2Reg { reg, imm },
//             })
//         }
//         // dst 是寄存器, src 是内存: RM2Reg
//         (false, true, Some(reg), None, None, Some(rm)) => {
//             Ok(Inst {
//                 mnemonic: mnemonic.to_string(),
//                 operand: Operand::RM2Reg { reg, rm },
//             })
//         }
//         // dst 是内存, src 是寄存器: Reg2RM
//         (true, false, None, Some(reg), Some(rm), None) => {
//             Ok(Inst {
//                 mnemonic: mnemonic.to_string(),
//                 operand: Operand::Reg2RM { reg, rm },
//             })
//         }
//         _ => {
//             Err(format!("unsupported operand combination: {} {}", dst_raw, src_raw))
//         }
//     }
// }
