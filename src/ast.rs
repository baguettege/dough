pub(crate) mod types;

use crate::ast::types::{BinaryOp, Param, UnaryOp};
use crate::span::{Span, Spanned};

macro_rules! node_enum {
    ($name:ident {
        $( $variant:ident($ty:ty) ),* $(,)?
    }) => {
        #[derive(Debug)]
        pub(crate) enum $name {
            $( $variant($ty), )*
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                match self {
                    $( Self::$variant(v) => v.span(), )*
                }
            }
        }

        $(
            impl From<$ty> for $name {
                fn from(value: $ty) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

macro_rules! node {
    ($name: ident {
        $( $field:ident: $ty:ty ),* $(,)?
    }) => {
        #[derive(Debug)]
        pub(crate) struct $name {
            $( $field: $ty, )*
            span: Span
        }

        impl $name {
            pub(crate) fn new($( $field: $ty, )* span: Span) -> Self {
                Self { $( $field, )* span }
            }

            $(
                pub(crate) fn $field(&self) -> &$ty {
                    &self.$field
                }
            )*
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.span
            }
        }
    };
}

macro_rules! ast {
    ($(
        $name:ident {
            $(
                $variant:ident {
                    $( $field:ident: $ty:ty ),* $(,)?
                }
            )*
        }
    )+) => {
        node_enum! {
            Node { $( $name($name), )* }
        }

        $(
            node_enum! {
                $name { $( $variant($variant), )* }
            }

            $(
                node! {
                    $variant { $( $field: $ty, )* }
                }
            )*
        )*
    };
}

ast! {
    Decl {
        VarDecl { name: String, type_name: String, init: Expr }
        FuncDef { name: String, params: Vec<Param>, return_type: String, body: Block }
    }

    Stmt {
        Block { stmts: Vec<Stmt> }
        ExprStmt { expr: Expr }

        IfStmt { cond: Expr, then_body: Block, else_body: Option<Block> }
        WhileStmt { cond: Expr, body: Block }

        ReturnStmt { value: Option<Expr> }

        AssignStmt { target: Expr, value: Expr }
    }

    Expr {
        BinaryExpr { left: Box<Expr>, op: BinaryOp, right: Box<Expr> }
        UnaryExpr { op: UnaryOp, operand: Box<Expr> }

        CallExpr { name: String, args: Vec<Expr> }

        IdentExpr { ident: String }
        IndexExpr { array: Box<Expr>, index: Box<Expr> }

        IntLiteral { value: i64 }
        FloatLiteral { value: f64 }
        BoolLiteral { value: bool }
        StrLiteral { value: String }
        ArrayLiteral { values: Vec<Expr> }
    }
}