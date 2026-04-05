use crate::analyzer::{common, resolve, Analyzer};
use crate::Result;
use ast::typed::Stmt;
use ast::types::{Ident, TypeRef};
use ast::untyped;
use dough_core::Type;

impl Analyzer {
    pub(super) fn analyze_stmt(
        &mut self,
        stmt: &untyped::Stmt
    ) -> Result<Stmt> {
        match stmt {
            untyped::Stmt::Expr(expr) =>
                self.analyze_expr_stmt(expr),
            untyped::Stmt::Let { ident, ty, init } =>
                self.analyze_let(ident, ty, init),
            untyped::Stmt::Assign { target, value } =>
                self.analyze_assign(target, value),
            untyped::Stmt::If { condition, then_body, else_body } =>
                self.analyze_if(condition, then_body, else_body),
            untyped::Stmt::While { condition, body } =>
                self.analyze_while(condition, body),
            untyped::Stmt::Return { value } =>
                self.analyze_return(value),
        }
    }

    fn analyze_expr_stmt(
        &mut self,
        expr: &untyped::Expr,
    ) -> Result<Stmt> {
        let expr = self.analyze_expr(expr)?;
        Ok(Stmt::Expr(expr))
    }

    fn analyze_let(
        &mut self,
        ident: &Ident,
        ty: &TypeRef,
        init: &untyped::Expr,
    ) -> Result<Stmt> {
        let ty = resolve::ty(ty)?;
        let init = self.analyze_expr(init)?;

        common::expect_expr_type(&init, ty)?;

        self.stack.insert(ident.clone(), ty);
        Ok(Stmt::Let { ident: ident.clone(), ty, init })
    }

    fn analyze_assign(
        &mut self,
        target: &Ident,
        value: &untyped::Expr,
    ) -> Result<Stmt> {
        let ty = self.lookup_var(target)?;

        let value = self.analyze_expr(value)?;
        common::expect_expr_type(&value, ty)?;

        Ok(Stmt::Assign { target: target.clone(), value })
    }

    fn analyze_if(
        &mut self,
        condition: &untyped::Expr,
        then_body: &untyped::Block,
        else_body: &Option<untyped::Block>,
    ) -> Result<Stmt> {
        let condition = self.analyze_expr(condition)?;
        common::expect_expr_type(&condition, Type::Bool)?;

        let then_body = self.with_scope(
            |this| this.analyze_block(then_body))?;
        let else_body = else_body.as_ref()
            .map(|block| self.with_scope(
                |this| this.analyze_block(block)))
            .transpose()?;

        Ok(Stmt::If { condition, then_body, else_body })
    }

    fn analyze_while(
        &mut self,
        condition: &untyped::Expr,
        body: &untyped::Block,
    ) -> Result<Stmt> {
        let condition = self.analyze_expr(condition)?;
        common::expect_expr_type(&condition, Type::Bool)?;

        let body = self.with_scope(
            |this| this.analyze_block(body))?;

        Ok(Stmt::While { condition, body })
    }

    fn analyze_return(
        &mut self,
        value: &Option<untyped::Expr>,
    ) -> Result<Stmt> {
        let value = value
            .as_ref()
            .map(|expr| self.analyze_expr(expr))
            .transpose()?;

        let ty = value
            .as_ref()
            .map(resolve::expr)
            .unwrap_or(Type::Unit);

        common::expect_type(self.return_ty(), ty)?;
        Ok(Stmt::Return { value })
    }
}
