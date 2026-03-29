use crate::bytecode::{bytecode, Decoder, Instr};

mod bytecode;
mod heap;
mod value;
mod lexer;
mod span;

fn main() {
    let code = bytecode! {
        IAdd dst:0 lhs:1 rhs:2;
        IAdd dst:3 lhs:1 rhs:2;
        Nop;
    };

    let mut decoder = Decoder::new(&code);
    println!(
        "{}\n{}\n{}",
        decoder.decode::<Instr>().unwrap(),
        decoder.decode::<Instr>().unwrap(),
        decoder.decode::<Instr>().unwrap()
    );
}
