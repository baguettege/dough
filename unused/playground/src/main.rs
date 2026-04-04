use ast_macros::ast;

fn main() {
    
}

#[ast]
pub enum Expr {
    Binary { lhs: Box<Expr>, rhs: Box<Expr> },
    Unary { expr: Box<Expr> },
    Test { #[copy] x: i32 },
}
