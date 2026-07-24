use swcal_x64::{asm::parser::*, inst::encode::InstBin};

fn main() {
    swcal_x64::codegen::list_inst();
    let src = include_str!("../tests/test_asm.asm");

    match parse(src) {
        Ok((el, _)) => {
            for section in el.sections {
                // println!("load {:?}", section.name);
                for data in section.data {
                    match data {
                        swcal_x64::el::Data::Inst(inst) => {
                            let bin = swcal_x64::codegen::x86_64_asembler(&inst);
                            println!("{}", inst);
                            match bin {
                                Ok(bin) => println!("{}", bin),
                                Err(err) => todo!(),
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
