use std::{io::Write, path::Path};

use crate::{format::*, generate::SrcGen};

pub mod format;
pub mod generate;
pub mod inst;

pub fn generate_inst_list(p: &Path) {
    if !p.is_dir() {
        return;
    }
    let mut src  = crate::generate::RustBuilder::new();
    src.function(stringify!(pub fn list_inst()), |c|{
        for (_mnemonic, inst_codgen) in inst::inst_codegen_table() {
            for instf in inst_codgen {
                c.line(format!("println!(\"{}\");", instf).as_str());
            }
        }
    });

    let inst_info_path = p.join("inst_info.rs");
    let mut f = std::fs::File::create(inst_info_path).unwrap();
    f.write(src.build().as_bytes()).unwrap();
}

pub fn generate_inst_emit(p: &Path) {
    if !p.is_dir() {
        return;
    }

    let mut src = crate::generate::RustBuilder::new();

    for (_mnemonic, inst_codgen) in inst::inst_codegen_table() {
        for instf in inst_codgen {
            // build inst
            build_inst(&mut src, &instf);
            src.blank();
        }
    }

    // build asmebler
    generate_asmebler(&mut src);

    let asm_mod_file_path = p.join("codegen.rs");
    let mut f = std::fs::File::create(asm_mod_file_path).unwrap();
    f.write(src.build().as_bytes()).unwrap();
}

// build asmebler
fn generate_asmebler(src: &mut crate::generate::RustBuilder) {
    // match mnemonic
    src.function(stringify!(pub fn x86_64_asembler(inst: &Inst) -> Result<InstBin,String>), |src|{
        src.blank()
            .stmt_match("inst.mnemonic.as_str()", |src|{
                for (mnemonic, _inst_codgen) in inst::inst_codegen_table() {
                    src.line(format!("\"{}\"=> asm_{}(inst),", mnemonic, mnemonic));
                }
                src.line(stringify!(_ => panic!("unsupport instruction {}", inst.mnemonic),));
            });
    });

    // instruction try
    for (mnemonic, inst_table) in inst::inst_codegen_table() {
        src.blank()
            .function(format!("pub fn asm_{}(inst: &Inst) -> Result<InstBin,String>", mnemonic), |src| {
                src.line("let mut ret: Option<InstBin> = None;");
                src.line("let mut temp = InstBin::new();");
                for instf in inst_table {
                    src.if_codition(format!("let Ok(inst) = {}::from_inst(inst)", instf.type_name()), |src| {
                        src.line("inst.encode(&mut temp);");
                        src.stmt_match("&ret", |src| {
                            src.line("Some(bin) if bin.len() > temp.len() => ret = Some(temp.clone()),");
                            src.line("None => ret = Some(temp.clone()),");
                            src.line("_=>{},");
                        });
                        src.line("temp.reset()");
                    });
                }
                src.line("ret.ok_or(\"unmatched instruction\".into())");
            });
    }
}
