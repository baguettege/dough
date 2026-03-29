use crate::ast::types::{BinaryOp, Literal, Name, Param, TypeRef, UnaryOp};

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
        Unary { op: UnaryOp, operand: Box<Expr> },

        Ident { name: Name },

        Call { name: Name, args: Vec<Expr> },
        Index { array: Box<Expr>, index: Box<Expr> },
        MemberAccess { object: Box<Expr>, member: Name },

        Lit { value: Literal },
    },

    Decl {
        Func { name: Name, params: Vec<Param>, return_type: TypeRef, body: Block },
        Var { name: Name, type_ref: TypeRef, init: Expr },
    }
}
