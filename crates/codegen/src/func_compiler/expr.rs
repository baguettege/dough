use ast::typed::{Binary, Call, Expr, Ident, LiteralExpr, Unary};
use ast::types::{BinOp, Literal, UnOp};
use bytecode::{Argc, Constant, Instr};
use dough_core::Type;
use crate::func_compiler::FuncCompiler;

impl FuncCompiler<'_> {
    pub(super) fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::LiteralExpr(node) => self.compile_literal_expr(node),
            Expr::Ident(node) => self.compile_ident(node),
            Expr::Binary(node) => self.compile_binary(node),
            Expr::Unary(node) => self.compile_unary(node),
            Expr::Call(node) => self.compile_call(node),
        }
    }

    fn compile_literal_expr(&mut self, node: &LiteralExpr) {
        let constant = match node.literal() {
            Literal::Int(v) => Constant::Int(*v),
            Literal::Float(v) => Constant::Float(*v),
            Literal::Bool(v) => Constant::Bool(*v),
            Literal::Str(v) => Constant::Str(v.clone()),
        };

        let idx = self.chunk.constant(constant);
        self.chunk.emit(Instr::Push(idx));
    }

    fn compile_ident(&mut self, node: &Ident) {
        let idx = self.locals.get(*node.binding());
        self.chunk.emit(Instr::Ldl(idx));
    }

    fn compile_binary(&mut self, node: &Binary) {
        self.compile_expr(node.lhs());
        self.compile_expr(node.rhs());

        let ty = node.lhs().ty();
        assert_eq!(ty, node.rhs().ty());

        let instr = match (ty, node.op()) {
            (Type::Int, BinOp::Add) => Instr::IAdd,
            (Type::Int, BinOp::Sub) => Instr::ISub,
            (Type::Int, BinOp::Mul) => Instr::IMul,
            (Type::Int, BinOp::Div) => Instr::IDiv,
            (Type::Int, BinOp::Lt)  => Instr::ILt,
            (Type::Int, BinOp::Le)  => Instr::ILe,
            (Type::Int, BinOp::Gt)  => Instr::IGt,
            (Type::Int, BinOp::Ge)  => Instr::IGe,
            (Type::Int, BinOp::Eq)  => Instr::IEq,
            (Type::Int, BinOp::Ne)  => Instr::INe,

            (Type::Float, BinOp::Add) => Instr::FAdd,
            (Type::Float, BinOp::Sub) => Instr::FSub,
            (Type::Float, BinOp::Mul) => Instr::FMul,
            (Type::Float, BinOp::Div) => Instr::FDiv,
            (Type::Float, BinOp::Lt)  => Instr::FLt,
            (Type::Float, BinOp::Le)  => Instr::FLe,
            (Type::Float, BinOp::Gt)  => Instr::FGt,
            (Type::Float, BinOp::Ge)  => Instr::FGe,
            (Type::Float, BinOp::Eq)  => Instr::FEq,
            (Type::Float, BinOp::Ne)  => Instr::FNe,

            (Type::Bool, BinOp::And) => Instr::BAnd,
            (Type::Bool, BinOp::Or)  => Instr::BOr,
            (Type::Bool, BinOp::Eq)  => Instr::BEq,
            (Type::Bool, BinOp::Ne)  => Instr::BNe,

            (Type::Str, BinOp::Add) => Instr::SAdd,
            (Type::Str, BinOp::Eq)  => Instr::SEq,
            (Type::Str, BinOp::Ne)  => Instr::SNe,

            _ => unreachable!(),
        };

        self.chunk.emit(instr);
    }

    fn compile_unary(&mut self, node: &Unary) {
        self.compile_expr(node.expr());

        let instr = match (node.ty(), node.op()) {
            (Type::Bool, UnOp::Not) => Instr::BNot,

            (Type::Int, UnOp::Neg) => Instr::INeg,
            (Type::Float, UnOp::Neg) => Instr::FNeg,

            _ => unreachable!(),
        };

        self.chunk.emit(instr);
    }

    fn compile_call(&mut self, node: &Call) {
        let idx = self.funcs.get(*node.binding());
        let argc: Argc = node.args()
            .len()
            .try_into()
            .expect("compiler bug: call argc out of range");

        for arg in node.args() {
            self.compile_expr(arg);
        }

        self.chunk.emit(Instr::Call(idx, argc));
    }
}
