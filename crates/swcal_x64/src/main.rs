use swcal_x64::asm::parser::*;
use swcal_x64::inst::inst_impl::{inst_codegen_table, codgen_emit};

fn main() {
    let src = include_str!("../tests/test_asm.asm");
    let codegen_table = inst_codegen_table();

    match parse(src) {
        Ok((el, _)) => {
            // println!("{}", el);
            for section in el.sections {
                println!("load {:?}", section.name);
                for data in section.data {
                    match data {
                        swcal_x64::el::Data::Inst(inst) => {
                            println!("parse {inst}");
                            let inst_gen = codgen_emit(&inst, &codegen_table);
                            for gen_inst in inst_gen {
                                match gen_inst {
                                    Ok(inst) => println!("{inst}"),
                                    Err(err) => println!("{err}"),
                                }
                            }
                            println!("----------------------------------");
                        }
                        swcal_x64::el::Data::RawData(items) => {}
                        swcal_x64::el::Data::Res(_) => {}
                        swcal_x64::el::Data::Align(_) => {}
                    }
                }
            }
        }
        Err(err) => println!("{:?}", err),
    }
    // let l = Lexer::new(src);
    // for t in l {
    //     println!("{:?}", t);
    // }
}
