use swcal_x64::asm::parser::*;

fn main() {
    // swcal_x64::codegen::list_inst();
    let src = include_str!("../tests/test_asm.asm");

    match parse(src) {
        Ok((el, _)) => {
            // println!("{}", el);
            for section in el.sections {
                println!("load {:?}", section.name);
                for data in section.data {
                    match data {
                        swcal_x64::el::Data::Inst(inst) => {
                            if inst.mnemonic == "mov" {
                                println!("parse {inst}");
                                let ret = swcal_x64::codegen::mov(&inst);
                                for i in ret {
                                    match i {
                                        Ok(i) => println!("{:x?}", i),
                                        Err(i) => {},//println!("{}", i),
                                    }
                                }
                                println!("--------------------------");
                            }
                        }
                        swcal_x64::el::Data::RawData(_items) => {}
                        swcal_x64::el::Data::Res(_) => {}
                        swcal_x64::el::Data::Align(_) => {}
                    }
                }
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
