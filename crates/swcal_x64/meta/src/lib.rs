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
            // gernate structure of Instruction
            src.record(instf.type_name(), |src| {
                genarate_operand_field(src, &instf.encode.operand);
            });

            src.blank();
            // method
            src.implement(instf.type_name(), |src| {
                // generate Inst to Specific Instrution struct
                src.function("pub fn build(inst: &Inst) -> Result<Self, String>", |src| {
                    src.line("todo!()");
                });
                src.blank();
            });
            src.blank();
        }
    }

    generate_asmbler(&mut src);

    let asm_mod_file_path = p.join("codegen.rs");
    let mut f = std::fs::File::create(asm_mod_file_path).unwrap();
    f.write(src.build().as_bytes()).unwrap();
}

fn generate_asmbler(src: &mut crate::generate::RustBuilder) {
    src.function(stringify!(pub fn x86_64_asmbler(inst: &Inst) -> Result<InstBin,String>), |c|{
        c.blank()
            .smatch("inst.mnemonic.as_str()", |c|{
                for (mnemonic, inst_codgen) in inst::inst_codegen_table() {
                    c.line(format!("\"{}\"=> asm_{}(inst),", mnemonic, mnemonic));
                }
                c.line(stringify!(_ => panic!(),));

            });

    });

    for (mnemonic, inst_codgen) in inst::inst_codegen_table() {
        src.blank()
            .function(format!("pub fn asm_{}(inst: &Inst) -> Result<InstBin,String>", mnemonic), |c| {
                c.line("let mut ret = InstBin::new();")
                    .line("todo!()");
            });
    }
}
