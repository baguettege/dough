use ast::typed::{Binary, Call, Expr, LiteralExpr, Unary, Ident};
use ast::types::{BinOp, Literal, UnOp};
use bytecode::{Argc, Constant, Instr, Reg};
use dough_core::Type;
use crate::emitter::chunk::Builder;
use crate::emitter::Emitter;
use crate::Result;
use crate::slot::Slot;

impl Emitter<'_> {
    pub(super) fn emit_expr(&self, builder: &mut Builder, expr: &Expr) -> Result<Reg> {
        match expr {
            Expr::LiteralExpr(node) => self.emit_literal_expr(builder, node),
            Expr::Ident(node) => Ok(self.emit_ident(builder, node)),
            Expr::Binary(node) => self.emit_binary(builder, node),
            Expr::Unary(node) => self.emit_unary(builder, node),
            Expr::Call(node) => self.emit_call(builder, node),
        }
    }

    fn emit_literal_expr(&self, builder: &mut Builder, node: &LiteralExpr) -> Result<Reg> {
        let dst = self.local(node);

        let constant = match node.literal() {
            Literal::Int(v) => Constant::Int(*v),
            Literal::Float(v) => Constant::Float(*v),
            Literal::Bool(v) => Constant::Bool(*v),
            Literal::Str(v) => Constant::Str(v.clone()),
        };

        let idx = builder.constant(constant)?;
        builder.emit(Instr::Ldc { dst, idx });

        Ok(dst)
    }

    fn emit_ident(&self, builder: &mut Builder, node: &Ident) -> Reg {
        let dst = self.local(node);

        let instr = match self.binding_slot(*node.binding()) {
            Slot::Global(idx) => Instr::Ldg { dst, idx },
            // if `dst == src`, then `Mov` doesn't need to be emitted;
            // however, this is an optimization
            Slot::Local(src) => Instr::Mov { dst, src },
            _ => unreachable!(),
        };

        builder.emit(instr);

        dst
    }

    fn emit_binary(&self, builder: &mut Builder, node: &Binary) -> Result<Reg> {
        let dst = self.local(node);
        let lhs = self.emit_expr(builder, node.lhs())?;
        let rhs = self.emit_expr(builder, node.rhs())?;

        let instr = match (node.operand_ty(), node.op()) {
            (Type::Int, BinOp::Add) => Instr::IAdd { dst, lhs, rhs },
            (Type::Int, BinOp::Sub) => Instr::ISub { dst, lhs, rhs },
            (Type::Int, BinOp::Mul) => Instr::IMul { dst, lhs, rhs },
            (Type::Int, BinOp::Div) => Instr::IDiv { dst, lhs, rhs },
            (Type::Int, BinOp::Lt) => Instr::ILt { dst, lhs, rhs },
            (Type::Int, BinOp::Le) => Instr::ILe { dst, lhs, rhs },
            (Type::Int, BinOp::Gt) => Instr::IGt { dst, lhs, rhs },
            (Type::Int, BinOp::Ge) => Instr::IGe { dst, lhs, rhs },
            (Type::Int, BinOp::Eq) => Instr::IEq { dst, lhs, rhs },
            (Type::Int, BinOp::Ne) => Instr::INe { dst, lhs, rhs },

            (Type::Float, BinOp::Add) => Instr::FAdd { dst, lhs, rhs },
            (Type::Float, BinOp::Sub) => Instr::FSub { dst, lhs, rhs },
            (Type::Float, BinOp::Mul) => Instr::FMul { dst, lhs, rhs },
            (Type::Float, BinOp::Div) => Instr::FDiv { dst, lhs, rhs },
            (Type::Float, BinOp::Lt)  => Instr::FLt  { dst, lhs, rhs },
            (Type::Float, BinOp::Le)  => Instr::FLe  { dst, lhs, rhs },
            (Type::Float, BinOp::Gt)  => Instr::FGt  { dst, lhs, rhs },
            (Type::Float, BinOp::Ge)  => Instr::FGe  { dst, lhs, rhs },
            (Type::Float, BinOp::Eq)  => Instr::FEq  { dst, lhs, rhs },
            (Type::Float, BinOp::Ne)  => Instr::FNe  { dst, lhs, rhs },

            (Type::Bool, BinOp::And) => Instr::BAnd { dst, lhs, rhs },
            (Type::Bool, BinOp::Or) => Instr::BOr { dst, lhs, rhs },
            (Type::Bool, BinOp::Eq) => Instr::BEq { dst, lhs, rhs },
            (Type::Bool, BinOp::Ne) => Instr::BNe { dst, lhs, rhs },

            (Type::Str, BinOp::Add) => Instr::SAdd { dst, lhs, rhs },
            (Type::Str, BinOp::Eq) => Instr::SEq { dst, lhs, rhs },
            (Type::Str, BinOp::Ne) => Instr::SNe { dst, lhs, rhs },

            _ => unreachable!(),
        };

        builder.emit(instr);
        Ok(dst)
    }

    fn emit_unary(&self, builder: &mut Builder, node: &Unary) -> Result<Reg> {
        let dst = self.local(node);
        let src = self.emit_expr(builder, node.expr())?;

        let instr = match (node.ty(), node.op()) {
            (Type::Bool, UnOp::Not) => Instr::BNot { dst, src },
            (Type::Int, UnOp::Neg) => Instr::INeg { dst, src },
            (Type::Float, UnOp::Neg) => Instr::FNeg { dst, src },
            _ => unreachable!(),
        };

        builder.emit(instr);
        Ok(dst)
    }

    fn emit_call(&self, builder: &mut Builder, node: &Call) -> Result<Reg> {
        let dst = self.local(node);
        let Slot::Fn(idx) = self.binding_slot(*node.binding()) else { unreachable!() };
        let argc: Argc = node.args().len()
            .try_into()
            .expect("compiler bug: call arg count out of range");

        for (arg, dst) in node.args().iter().zip(dst + 1..) {
            let src = self.emit_expr(builder, arg)?;
            builder.emit(Instr::Mov { dst, src });
        }

        builder.emit(Instr::Call { dst, idx, argc });
        Ok(dst)
    }
}
