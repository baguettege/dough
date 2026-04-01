pub(crate) use crate::ast::types::*;

mod types;

macro_rules! ast {
    ($(
        $node:ident {
            $(
                $variant:ident {
                    $(
                        $field: ident: $type:ty
                    ),* $(,)?
                }
            ),+ $(,)?
        }
    ),* $(,)?) => {
        $(
            #[derive(Debug)]
            pub(crate) enum $node {
                $( $variant($variant) ),*
            }

            impl $crate::span::Spanned for $node {
                fn span(&self) -> $crate::span::Span {
                    match self {
                        $( Self::$variant(v) => v.span, )*
                    }
                }
            }

            $(
                #[derive(Debug)]
                pub(crate) struct $variant {
                    $( $field: $type, )*
                    span: $crate::span::Span,
                }

                impl $variant {
                    pub(crate) fn new( $( $field: $type, )* span: $crate::span::Span ) -> Self {
                        Self { $( $field, )* span }
                    }

                    $(
                        pub(crate) fn $field(&self) -> &$type {
                            &self.$field
                        }
                    )*
                }

                impl $crate::span::Spanned for $variant {
                    fn span(&self) -> $crate::span::Span {
                        self.span
                    }
                }

                impl From<$variant> for $node {
                    fn from(value: $variant) -> Self {
                        $node::$variant(value)
                    }
                }
            )*
        )*
    };
}

ast! {
    Stmt {
        ExprStmt { expr: Expr },
        DeclStmt { decl: Decl },

        Block { stmts: Vec<Stmt> },

        Assign { target: Expr, value: Expr },

        If { condition: Expr, then_body: Block, else_body: Option<Block> },
        While { condition: Expr, body: Block },

        Return { value: Option<Expr> },
    },

    Expr {
        Binary { lhs: Box<Expr>, op: BinaryOp, rhs: Box<Expr> },
        Unary { op: UnaryOp, expr: Box<Expr> },

        Ident { name: Name },

        Call { callee: Box<Expr>, args: Vec<Expr> },
        Index { array: Box<Expr>, index: Box<Expr> },
        Member { object: Box<Expr>, member: Name },

        Lit { value: Literal },
    },

    Decl {
        Func { name: Name, params: Vec<Param>, return_type: Option<TypeRef>, body: Block },
        Var { name: Name, type_ref: TypeRef, init: Expr },
    }
}

#[derive(Debug)]
pub(crate) struct Program {
    decls: Vec<Decl>,
}

impl Program {
    pub(crate) fn new(decls: Vec<Decl>) -> Self {
        Self { decls }
    }
    
    pub(crate) fn decls(&self) -> &[Decl] {
        &self.decls
    }
}
