use swcal_x64::asm::parser::*;

fn main() {
    // swcal_x64::codegen::list_inst();
    let src = include_str!("../tests/test_asm.asm");

    match parse(src) {
        Ok((mut program, _)) => {
            program.scan_reloc_and_modify_inst();
            for section in program.sections {
                let mut base = 0x1000;
                for data in section.data {
                    match data {
                        swcal_x64::el::Data::Inst(inst) => {
                            let bin = swcal_x64::codegen::x86_64_asembler(&inst);
                            match bin {
                                Ok(bin) => {
                                    base += bin.len();
                                    println!("{base:<8x} {bin:50} {inst}");
                                },
                                Err(err) => {
                                    println!("error: {err} {inst}");
                                },
                            }
                        }
                        swcal_x64::el::Data::RawData{..} => {}
                        swcal_x64::el::Data::Res(_) => {}
                        swcal_x64::el::Data::Align(_) => {}
                    }
                }
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
