use std::time::Instant;
use error::Result;

mod error;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .expect("usage: dough <path>");
    let src = std::fs::read_to_string(path)?;

    let tokens = lexer::lex(&src)?;
    let untyped = parser::parse(&tokens)?;
    let typed = semantic::analyze(&untyped)?;
    let program = codegen::compile(&typed);

    let start = Instant::now();
    let result = vm::run(&program);
    let elapsed = start.elapsed();

    println!("executed in {:?}", elapsed);

    result.map_err(|e| e.into())
}
