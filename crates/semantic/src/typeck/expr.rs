use crate::typeck::{ty, TypeChecker};
use crate::{Error, Result};
use ast::typed::{Binary, Call, Expr, Ident, LiteralExpr, Unary};
use ast::{untyped, Node};
use ast::types::{BinOp, UnOp};
use dough_core::Type;
use crate::symbol::Symbol;

impl TypeChecker<'_> {
    pub(super) fn check_expr(&self, expr: &untyped::Expr) -> Result<Expr> {
        match expr {
            untyped::Expr::LiteralExpr(node) => self.check_literal_expr(node).map(Into::into),
            untyped::Expr::Ident(node) => self.check_ident(node).map(Into::into),
            untyped::Expr::Binary(node) => self.check_binary(node).map(Into::into),
            untyped::Expr::Unary(node) => self.check_unary(node).map(Into::into),
            untyped::Expr::Call(node) => self.check_call(node).map(Into::into),
        }
    }

    fn check_literal_expr(&self, node: &untyped::LiteralExpr) -> Result<LiteralExpr> {
        let literal = node.literal().clone();
        Ok(LiteralExpr::new(node.id(), literal))
    }

    fn check_ident(&self, node: &untyped::Ident) -> Result<Ident> {
        let ty = match self.bindings.get(node) {
            Symbol::Global(ty) | Symbol::Local(ty) => *ty,
            Symbol::Fn { .. } => return Err(Error::NotAValue(node.ident().clone())),
        };

        Ok(Ident::new(node.id(), node.ident().clone(), ty))
    }

    fn check_binary(&self, node: &untyped::Binary) -> Result<Binary> {
        let (lhs, rhs) = (self.check_expr(node.lhs())?, self.check_expr(node.rhs())?);

        let ty = ty::of(&lhs);
        ty::expect(ty, ty::of(&rhs))?;

        let op = *node.op();
        let ty = match (ty, op) {
            (Type::Int | Type::Float,
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div) => ty,

            (Type::Int | Type::Float,
                BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge) => Type::Bool,

            (Type::Bool,
                BinOp::And | BinOp::Or) => Type::Bool,

            (Type::Int | Type::Float | Type::Bool | Type::Str,
                BinOp::Eq | BinOp::Ne) => Type::Bool,

            (Type::Str,
                BinOp::Add) => Type::Str,

            _ => return Err(Error::InvalidBinOp { op, ty }),
        };

        Ok(Binary::new(node.id(), Box::new(lhs), op, Box::new(rhs), ty))
    }

    fn check_unary(&self, node: &untyped::Unary) -> Result<Unary> {
        let expr = self.check_expr(node.expr())?;
        let ty = ty::of(&expr);

        let op = *node.op();
        match (ty, *node.op()) {
            (Type::Bool, UnOp::Not) |
            (Type::Int | Type::Float, UnOp::Neg) =>
                Ok(Unary::new(node.id(), op, Box::new(expr), ty)),

            _ => Err(Error::InvalidUnOp { op, ty }),
        }
    }

    fn check_call(&self, node: &untyped::Call) -> Result<Call> {
        let Symbol::Fn { params, return_ty } = self.bindings.get(node)
        else { return Err(Error::NotCallable(node.callee().clone())) };

        let args = node.args();
        if args.len() != params.len() {
            return Err(Error::ArityMismatch {
                expected: params.len(),
                found: args.len(),
            });
        }

        let args = args
            .iter()
            .zip(params)
            .map(|(arg, &ty)| {
                let arg = self.check_expr(arg)?;
                ty::expect(ty, ty::of(&arg))?;
                Ok(arg)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Call::new(node.id(), node.callee().clone(), args, *return_ty))
    }
}
