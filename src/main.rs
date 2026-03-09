use instr::Instr;
use crate::instr::disassembler::disassemble;
use crate::lexer::{Lexer};

mod heap;
mod value;
mod error;
mod bytecode;
mod source;
mod ast;
mod compiler;
mod instr;
mod lexer;

fn main() {
    let code = assemble! {
        Instr::Nop {},
        Instr::IAdd { dst: 1, a: 2, b: 3 }
        Instr::Call { idx: 128, arg_start: 56, ret: 255 }
    };

    println!("{:?}", code);

    let disassembled = disassemble(&code);
    disassembled.iter().for_each(|instr| println!("{}", instr));

    println!("-----------------------------");

    let s = "
var x: int = 10;
var y: float = 3.14;
var flag: bool = true;

func add(a: int, b: int): int {
    var result: int = a + b;
    return result;
}

if x >= 10 and flag {
    y = y + 1.5;
} else {
    y = y - 2.0;
}

while x < 20 {
    x = x + 1;
}
    ";

    match Lexer::lex(s) {
        Ok(tokens) => {
            let vec: Vec<String> =
            tokens.iter().map(|t| t.to_string()).collect();
            vec.iter().for_each(|s| println!("{}", s))
        },
        Err(e) => eprintln!("{:?}", e)
    }

    println!("-----------------------------");
}