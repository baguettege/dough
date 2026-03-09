use crate::source::{SourceRange, Spanned};

macro_rules! node {
    (
        $(
            $name:ident {
                $( $field:ident: $ty:ty ),* $(,)?
            }
        )*
    ) => {
        $(
            #[derive(Debug)]
            pub(crate) struct $name {
                $( pub(crate)  $field: $ty, )*
                pub(crate) source_range: SourceRange
            }

            impl $name {
                pub(crate) fn new( $( $field: $ty, )* source_range: SourceRange ) -> Self {
                    Self { $( $field, )* source_range }
                }
            }

            impl Spanned for $name {
                fn source_range(&self) -> SourceRange {
                    self.source_range
                }
            }
        )*
    };
}

macro_rules! ast_enum {
    (
        $name:ident {
            $( $variant:ident ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub(crate) enum $name {
            $( $variant($variant), )*
        }

        impl Spanned for $name {
            fn source_range(&self) -> SourceRange {
                match self {
                    $( $name::$variant(v) => v.source_range(), )*
                }
            }
        }
    };
}

macro_rules! ast {
    (
        decls {
            $(
                $dname:ident {
                    $( $dfield:ident: $dty:ty ),* $(,)?
                }
            )+
        }
        stmts {
            $(
                $sname:ident {
                    $( $sfield:ident: $sty:ty ),* $(,)?
                }
            )+
        }
        exprs {
            $(
                $ename:ident {
                    $( $efield:ident: $ety:ty ),* $(,)?
                }
            )+
        }
    ) => {
        #[derive(Debug)]
        pub(crate) enum Node {
            Decl(Decl),
            Stmt(Stmt),
            Expr(Expr)
        }

        impl Spanned for Node {
            fn source_range(&self) -> SourceRange {
                match self {
                    Node::Decl(decl) => decl.source_range(),
                    Node::Stmt(stmt) => stmt.source_range(),
                    Node::Expr(expr) => expr.source_range()
                }
            }
        }

        ast_enum!(Decl { $( $dname, )* });
        ast_enum!(Stmt { $( $sname, )* });
        ast_enum!(Expr { $( $ename, )* });

        node! {
            $(
                $dname { $( $dfield: $dty, )* }
            )*
            $(
                $sname { $( $sfield: $sty, )* }
            )*
            $(
                $ename { $( $efield: $ety, )* }
            )*
        }
    };
}

#[derive(Debug)]
pub(crate) struct Param {
    name: String,
    type_name: String
}

impl Param {
    pub(crate) fn new(name: &str, type_name: &str) -> Self {
        Self {
            name: name.to_string(),
            type_name: type_name.to_string()
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(crate) fn type_name(&self) -> &str {
        self.type_name.as_str()
    }
}

#[derive(Debug)]
pub(crate) enum BinaryOp {
    Mul, Div,

    Add, Sub,

    Lt, Le, Gt, Ge,

    Eq, Ne,

    And,

    Or
}

impl BinaryOp {
    pub(crate) fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Mul => 5,
            BinaryOp::Div => 5,

            BinaryOp::Add => 4,
            BinaryOp::Sub => 4,

            BinaryOp::Lt => 3,
            BinaryOp::Le => 3,
            BinaryOp::Gt => 3,
            BinaryOp::Ge => 3,

            BinaryOp::Eq => 2,
            BinaryOp::Ne => 2,

            BinaryOp::And => 1,

            BinaryOp::Or => 0
        }
    }
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Not, Neg
}

ast! {
    decls {
        VarDecl { name: String, type_name: String, init: Expr }
        FuncDef { name: String, params: Vec<Param>, return_type: String, body: Block }
    }

    stmts {
        Block { stmts: Vec<Stmt> }
        ExprStmt { expr: Expr }

        IfStmt { cond: Expr, then_body: Block, else_body: Option<Block> }
        WhileStmt { cond: Expr, body: Block }

        ReturnStmt { value: Option<Expr> }

        AssignStmt { target: Expr, value: Expr }
    }

    exprs {
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