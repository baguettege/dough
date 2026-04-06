use ast::untyped::{Binary, Call, Expr, Ident, LiteralExpr, Unary};
use crate::resolver::Resolver;
use crate::Result;

impl Resolver {
    pub(super) fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::LiteralExpr(node) => self.resolve_literal_expr(node),
            Expr::Ident(node) => self.resolve_ident(node),
            Expr::Binary(node) => self.resolve_binary(node),
            Expr::Unary(node) => self.resolve_unary(node),
            Expr::Call(node) => self.resolve_call(node),
        }
    }

    fn resolve_literal_expr(&mut self, _node: &LiteralExpr) -> Result<()> {
        Ok(())
    }

    fn resolve_ident(&mut self, node: &Ident) -> Result<()> {
        let symbol = self.scope.lookup(node.ident())?.clone();
        self.bindings.insert(node, symbol);
        Ok(())
    }

    fn resolve_binary(&mut self, node: &Binary) -> Result<()> {
        self.resolve_expr(node.lhs())?;
        self.resolve_expr(node.rhs())?;
        Ok(())
    }

    fn resolve_unary(&mut self, node: &Unary) -> Result<()> {
        self.resolve_expr(node.expr())
    }

    fn resolve_call(&mut self, node: &Call) -> Result<()> {
        let symbol = self.scope.lookup(node.callee())?.clone();
        self.bindings.insert(node, symbol);
        node.args()
            .iter()
            .try_for_each(|a| self.resolve_expr(a))?;
        Ok(())
    }
}
