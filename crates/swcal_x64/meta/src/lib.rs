use std::{io::Write, path::Path};

use crate::format::*;

pub mod format;
pub mod generate;
pub mod inst;

pub fn generate_inst_list(p: &Path) {
    if !p.is_dir() {
        return;
    }
    let mut src  = crate::generate::RustBuilder::new();
    src.function(stringify!(pub fn list_inst()), |c|{
        for instf in inst::mov::mov() {
            c.line(format!("println!(\"{}\");", instf).as_str());
        }
    });


    let asm_mod_file_path = p.join("inst_info.rs");
    let mut f = std::fs::File::create(asm_mod_file_path).unwrap();
    f.write(src.build().as_bytes()).unwrap();
}

pub fn generate_inst_emit(p: &Path) {
    if !p.is_dir() {
        return;
    }

    let mut src = crate::generate::RustBuilder::new();

    for instf in inst::mov::mov() {
        src.function(format!("pub fn {}(inst: &Inst, buf: &mut impl CodeSink) -> Result<(), String>",instf.name()), |c|{
            // check width
            c.line(format!("inst.width_validate::<{}>()?;", instf.operand_size));
            // prefix
            match &instf.prefix {
                Prefix::Legacy => {c.line("inst.encode_prefix_lagecy(buf)?;");}
                Prefix::EvexVex => todo!(),
            };
            // opcode
            c.line(format!("buf.putb(0x{:x});", instf.opcode.fst));
            if let Some(code) = instf.opcode.snd {
                c.line(format!("buf.putb(0x{:x});", code));
            }
            if let Some(code) = instf.opcode.trd {
                c.line(format!("buf.putb(0x{:x});", code));
            }
            // opcode? and operand
            c.if_codition(format!("!({})",check_operand(&instf)), |c| {
              c.line(format!("return Err(\"operand kind unmatched\".to_string())" ));
            });

            match &instf.encode_kind {
                EncodeKind::ModRM => {
                    match &instf.operand_kind {
                        OperandKind::RM2Reg | OperandKind::Reg2RM | OperandKind::Reg2Reg => {
                            c.line("inst.encode_modrm(buf)?;");
                        },
                        _ => {}
                    }
                },
                EncodeKind::RegExtOp(n) => {
                    match &instf.operand_kind {
                        OperandKind::Imm2RM => {
                            c.line(format!("inst.encode_modrm_reg_ext_op::<{}>(buf)?;", n));
                        }
                        _ => {
                            panic!("EncodeKind unmatch Operand {}", instf);
                        }
                    }
                },
                EncodeKind::RegEncOp => {
                    match &instf.operand_kind {
                        OperandKind::Imm2RM => {
                            c.line("inst.encode_reg_enc_op(buf)?;");
                        },
                        _ => {
                            panic!("EncodeKind unmatched Operand {}", instf);
                        }
                    }
                },
                EncodeKind::OpFixedReg(reg_id) => {
                    todo!()
                },
            }
            c.line("Ok(())");
        });
        src.blank();
    }

    src.function("pub fn mov(inst: &Inst) -> Vec<Result<Vec<u8>, String>>", |c| {
        c.line("let mut ret = Vec::new();");

        for instf in inst::mov::mov() {
            c.blank()
                .line("let mut buf = Vec::new();")
                .line(format!("ret.push({}(inst, &mut buf).map(|_| buf));",instf.name()));
        }

        c.blank()
            .line("ret");
    });

    let asm_mod_file_path = p.join("codegen.rs");
    let mut f = std::fs::File::create(asm_mod_file_path).unwrap();
    f.write(src.build().as_bytes()).unwrap();
}

fn check_operand(instf: &InstFormat) -> &'static str {
    match instf.operand_kind {
        OperandKind::NoOperand => "inst.dst.is_none() && inst.src.is_none()",
        OperandKind::RM => "operand_is_rm(&inst.dst) && inst.src.is_none()",
        OperandKind::Imm2RM => "operand_is_rm(&inst.dst) && operand_is_imm(&inst.src)",
        OperandKind::Reg2RM => "operand_is_rm(&inst.dst) && operand_is_reg(&inst.src)",
        OperandKind::RM2Reg => "operand_is_reg(&inst.dst) && operand_is_rm(&inst.src)",
        OperandKind::Reg2Reg => "operand_is_reg(&inst.dst) && operand_is_reg(&inst.src)",
    }
}
