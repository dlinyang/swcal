use swcal_x64::{asm::parser::*, inst::encode::InstBin};

fn main() {
    swcal_x64::codegen::list_inst();
    // let src = include_str!("../tests/test_asm.asm");


    // match parse(src) {
    //     Ok((el, _)) => {
    //         // println!("{}", el);
    //         for section in el.sections {
    //             println!("load {:?}", section.name);
    //             for data in section.data {
    //                 match data {
    //                     swcal_x64::el::Data::Inst(inst) => {
    //                         if inst.mnemonic == "mov" {
    //                             println!("parse {inst}");
    //                             let ret = swcal_x64::codegen::mov(&inst);
    //                             let ok_ret: Vec<_> = ret.iter().filter(|x| x.is_ok()).flatten().collect();
    //                             if ok_ret.len() > 0 {
    //                                 let mut a: Option<InstBin> = None;
    //                                 for i in ok_ret {
    //                                     a = if let Some(j) = a { Some(j.less(*i)) } else { Some(*i) };
    //                                 }
    //                                 println!("{}", a.unwrap());
    //                             } else {
    //                                 for i in ret {
    //                                     if let Err(err) = i {
    //                                         println!("error: {}", err);
    //                                     }
    //                                 }
    //                             }
    //                             println!("--------------------------");
    //                         }
    //                     }
    //                     swcal_x64::el::Data::RawData(_items) => {}
    //                     swcal_x64::el::Data::Res(_) => {}
    //                     swcal_x64::el::Data::Align(_) => {}
    //                 }
    //             }
    //         }
    //     }
    //     Err(err) => println!("{:?}", err),
    // }
}
