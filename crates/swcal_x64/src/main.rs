// use swcal_x64::asm::lexer::*;
use swcal_x64::asm::parser::*;

fn main () {
    let src = include_str!("../tests/test_asm.asm");
    match parse(src) {
        Ok((el,_)) => println!("{:?}", el),
        Err(err) => println!("{:?}", err),
    }
    // let l = Lexer::new(src);
    // for t in l {
    //     println!("{:?}", t);
    // }
}
