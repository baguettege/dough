use crate::typeck::{ty, TypeChecker};
use crate::{Error, Result};
use ast::typed::{Assign, ExprStmt, If, Let, Return, Stmt, While};
use ast::{untyped, Node};
use dough_core::Type;
use crate::symbol::Symbol;

impl TypeChecker<'_> {
    pub(super) fn check_stmt(&self, stmt: &untyped::Stmt) -> Result<Stmt> {
        match stmt {
            untyped::Stmt::ExprStmt(node) => self.check_expr_stmt(node).map(Into::into),
            untyped::Stmt::Let(node) => self.check_let(node).map(Into::into),
            untyped::Stmt::Assign(node) => self.check_assign(node).map(Into::into),
            untyped::Stmt::If(node) => self.check_if(node).map(Into::into),
            untyped::Stmt::While(node) => self.check_while(node).map(Into::into),
            untyped::Stmt::Return(node) => self.check_return(node).map(Into::into),
        }
    }

    fn check_expr_stmt(&self, node: &untyped::ExprStmt) -> Result<ExprStmt> {
        let expr = self.check_expr(node.expr())?;
        Ok(ExprStmt::new(node.id(), expr))
    }

    fn check_let(&self, node: &untyped::Let) -> Result<Let> {
        // resolver guarantees this is a `Symbol::Local`
        let Symbol::Local(ty) = self.bindings.get(node)
        else { unreachable!() };

        let init = self.check_expr(node.init())?;
        ty::expect(*ty, ty::of(&init))?;

        Ok(Let::new(node.id(), node.ident().clone(), *ty, init))
    }

    fn check_assign(&self, node: &untyped::Assign) -> Result<Assign> {
        let ty = match self.bindings.get(node) {
            Symbol::Global(ty) | Symbol::Local(ty) => *ty,
            Symbol::Fn { .. } => return Err(Error::NotAssignable(node.target().clone())),
        };

        let value = self.check_expr(node.value())?;
        ty::expect(ty, ty::of(&value))?;

        Ok(Assign::new(node.id(), node.target().clone(), value))
    }

    fn check_if(&self, node: &untyped::If) -> Result<If> {
        let condition = self.check_expr(node.condition())?;
        ty::expect(Type::Bool, ty::of(&condition))?;

        let then_body = self.check_block(node.then_body())?;
        let else_body = node
            .else_body()
            .as_ref()
            .map(|b| self.check_block(b))
            .transpose()?;

        Ok(If::new(node.id(), condition, then_body, else_body))
    }

    fn check_while(&self, node: &untyped::While) -> Result<While> {
        let condition = self.check_expr(node.condition())?;
        ty::expect(Type::Bool, ty::of(&condition))?;

        let body = self.check_block(node.body())?;

        Ok(While::new(node.id(), condition, body))
    }

    fn check_return(&self, node: &untyped::Return) -> Result<Return> {
        let value = node
            .value()
            .as_ref()
            .map(|expr| self.check_expr(expr))
            .transpose()?;
        let ty = value
            .as_ref()
            .map(ty::of)
            .unwrap_or(Type::Unit);

        // `return_ty` should never be `None` as it is set to `Some` before
        // the type checker walks the function body
        let return_ty = self.return_ty.unwrap();
        ty::expect(ty, return_ty)?;

        Ok(Return::new(node.id(), value))
    }
}
