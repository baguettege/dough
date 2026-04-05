fn main() {}

#[cfg(test)]
mod tests {
    use ast::types::Ident;
    use ast::untyped;
    use dough_core::Type;

    fn parse(src: &str) -> untyped::Program {
        let tokens = lexer::lex(src).unwrap();
        parser::parse(&tokens).unwrap()
    }

    fn analyze(src: &str) -> semantic::Result<semantic::TypedProgram> {
        semantic::analyze(&parse(src))
    }

    #[test]
    fn test_fn() {
        let program = analyze(r#"


            fn add(lhs: int, rhs: int): int {
                return lhs + rhs;
            }
        "#);

        println!("{:?}", program)
    }
}
