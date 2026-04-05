use crate::analyzer::{resolve, Analyzer};
use crate::symbol::Symbol;
use crate::{Error, Result};
use ast::typed::{Item, Param};
use ast::types::Ident;
use ast::untyped;

impl Analyzer {
    pub(super) fn analyze_item(
        &mut self,
        item: &untyped::Item
    ) -> Result<Item> {
        match item {
            untyped::Item::Fn { ident, params, body, .. } =>
                self.analyze_fn(ident, params, body),
            untyped::Item::Static { ident, init, .. } =>
                self.analyze_static(ident, init),
        }
    }

    fn analyze_fn(
        &mut self,
        ident: &Ident,
        params: &[untyped::Param],
        body: &untyped::Block,
    ) -> Result<Item> {
        // symbol was resolved in 1st pass so this shouldn't panic
        let (param_tys, return_ty) = match self.table.lookup(ident) {
            Some(Symbol::Fn { params, return_ty }) =>
                (params.clone(), *return_ty),
            _ => unreachable!(),
        };

        self.return_ty = Some(return_ty);

        let (typed_params, body) =
            self.with_scope(|this| {
                let mut typed_params = Vec::new();

                // insert each param into the current local scope and
                // build the typed param list
                for (param, ty) in params.iter().zip(param_tys) {
                    let ident = param.ident();
                    this.stack.insert(ident.clone(), ty);

                    let param = Param::new(ident.clone(), ty);
                    typed_params.push(param);
                }

                let body = this.analyze_block(body)?;
                Ok((typed_params, body))
            })?;

        self.return_ty = None;

        Ok(Item::Fn {
            ident: ident.clone(),
            params: typed_params,
            return_ty,
            body,
        })
    }

    fn analyze_static(
        &mut self,
        ident: &Ident,
        init: &untyped::Expr,
    ) -> Result<Item> {
        // symbol was resolved in 1st pass so this shouldn't panic
        let ty = match self.table.lookup(ident) {
            Some(Symbol::Global(ty)) => *ty,
            _ => unreachable!(),
        };

        let init = self.analyze_expr(init)?;
        let init_ty = resolve::expr(&init);

        if ty == init_ty {
            Ok(Item::Static {
                ident: ident.clone(),
                ty,
                init
            })
        } else {
            Err(Error::TypeMismatch {
                expected: ty,
                found: init_ty,
            })
        }
    }
}
