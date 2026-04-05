use crate::analyzer::{common, resolve, Analyzer};
use crate::{Error, Result};
use crate::symbol::Symbol;
use ast::typed::Expr;
use ast::types::{BinOp, Ident, UnOp};
use ast::untyped;
use dough_core::Type;

impl Analyzer {
    pub(super) fn analyze_expr(
        &mut self,
        expr: &untyped::Expr
    ) -> Result<Expr> {
        match expr {
            untyped::Expr::Literal(literal) =>
                Ok(Expr::Literal(literal.clone())),
            untyped::Expr::Ident(ident) =>
                self.analyze_ident(ident),
            untyped::Expr::Binary { lhs, op, rhs } =>
                self.analyze_binary(lhs, *op, rhs),
            untyped::Expr::Unary { op, expr } =>
                self.analyze_unary(*op, expr),
            untyped::Expr::Call { callee, args } =>
                self.analyze_call(callee, args),
        }
    }

    fn analyze_ident(
        &mut self,
        ident: &Ident
    ) -> Result<Expr> {
        let ty = self.lookup_var(ident)?;
        Ok(Expr::Ident { ident: ident.clone(), ty })
    }

    fn analyze_binary(
        &mut self,
        lhs: &untyped::Expr,
        op: BinOp,
        rhs: &untyped::Expr,
    ) -> Result<Expr> {
        let lhs = self.analyze_expr(lhs)?;
        let rhs = self.analyze_expr(rhs)?;

        let lhs_ty = resolve::expr(&lhs);
        let rhs_ty = resolve::expr(&rhs);

        if lhs_ty != rhs_ty {
            return Err(Error::TypeMismatch {
                expected: lhs_ty,
                found: rhs_ty,
            });
        }

        let ty = match (lhs_ty, op) {
            (Type::Int | Type::Float,
                BinOp::Add | BinOp::Sub |
                BinOp::Mul | BinOp::Div) => lhs_ty,

            (Type::Int | Type::Float,
                BinOp::Lt | BinOp::Le |
                BinOp::Gt | BinOp::Ge) => Type::Bool,

            (Type::Bool,
                BinOp::And | BinOp::Or) => Type::Bool,

            (Type::Int | Type::Float | Type::Bool | Type::Str,
                BinOp::Eq | BinOp::Ne) => Type::Bool,

            (Type::Str,
                BinOp::Add) => Type::Str,

            _ => return Err(Error::InvalidBinOp { op, ty: lhs_ty }),
        };

        Ok(Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            ty,
        })
    }

    fn analyze_unary(
        &mut self,
        op: UnOp,
        expr: &untyped::Expr,
    ) -> Result<Expr> {
        let expr = self.analyze_expr(expr)?;
        let ty = resolve::expr(&expr);

        match (ty, op) {
            (Type::Bool, UnOp::Not) |
            (Type::Int | Type::Float, UnOp::Neg) =>
                Ok(Expr::Unary { op, expr: Box::new(expr), ty }),
            _ => Err(Error::InvalidUnOp { op, ty }),
        }
    }

    fn analyze_call(
        &mut self,
        callee: &Ident,
        args: &[untyped::Expr],
    ) -> Result<Expr> {
        let (params, ty) = match self.table.lookup(callee) {
            Some(Symbol::Fn { params, return_ty }) =>
                (params.clone(), *return_ty),
            Some(_) => return Err(Error::NotCallable(callee.clone())),
            None => return Err(Error::UndefinedIdent(callee.clone())),
        };

        if params.len() != args.len() {
            return Err(Error::ArityMismatch {
                expected: params.len(),
                found: args.len(),
            });
        }

        let args = args
            .iter()
            .zip(params)
            .map(|(arg, ty)| {
                let arg = self.analyze_expr(arg)?;
                common::expect_expr_type(&arg, ty)?;
                Ok(arg)
            })
            .collect::<Result<_>>()?;

        Ok(Expr::Call { callee: callee.clone(), args, ty })
    }
}
