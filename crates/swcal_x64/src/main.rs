use swcal_x64::asm::parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut iter = args.iter();
    iter.next();
    let src_path = iter.next().expect("no input file");
    let src = std::fs::read_to_string(src_path).expect("wrong input path");

    match parse(src.as_str()) {
        Ok((mut program, _)) => {
            program.scan_reloc_and_modify_inst();
            let mut out = Vec::<u8>::new();
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
                                    for i in 0..bin.len() {
                                        out.push(bin[i]);
                                    }
                                },
                                Err(err) => {
                                    println!("error: {err} {inst}");
                                },
                            }
                        }
                        swcal_x64::el::Data::RawData{ width: _, data } => {
                            out.append(&mut data.clone());
                        }
                        swcal_x64::el::Data::Res(len) => {
                            for _ in 0..len {
                                out.push(0);
                            }
                        }
                        swcal_x64::el::Data::Align(align) => {
                            let addr = base + out.len();
                            let align = align as usize;
                            let fill_len = if addr % align != 0 {
                                align - (addr % align)
                            } else {
                                0
                            };
                            for _ in 0..fill_len {
                                out.push(0);
                            }
                        }
                    }
                }
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
