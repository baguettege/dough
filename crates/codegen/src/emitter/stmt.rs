use ast::typed::{Assign, ExprStmt, If, Let, Return, Stmt, While};
use bytecode::Instr;
use crate::emitter::chunk::{Builder, JumpPatch};
use crate::emitter::Emitter;
use crate::{emitter, Result};
use crate::slot::Slot;

impl Emitter<'_> {
    pub(super) fn emit_stmt(&self, builder: &mut Builder, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::ExprStmt(node) => self.emit_expr_stmt(builder, node),
            Stmt::Let(node) => self.emit_let(builder, node),
            Stmt::Assign(node) => self.emit_assign(builder, node),
            Stmt::If(node) => self.emit_if(builder, node),
            Stmt::While(node) => self.emit_while(builder, node),
            Stmt::Return(node) => self.emit_return(builder, node),
        }
    }

    fn emit_expr_stmt(&self, builder: &mut Builder, node: &ExprStmt) -> Result<()> {
        let _src = self.emit_expr(builder, node.expr())?;
        Ok(())
    }

    fn emit_let(&self, builder: &mut Builder, node: &Let) -> Result<()> {
        let src = self.emit_expr(builder, node.init())?;
        let dst = self.local(node);

        builder.emit(Instr::Mov { dst, src });
        Ok(())
    }

    fn emit_assign(&self, builder: &mut Builder, node: &Assign) -> Result<()> {
        let src = self.emit_expr(builder, node.value())?;
        let instr = match self.binding_slot(*node.binding()) {
            Slot::Global(idx) => Instr::Stg { idx, src },
            Slot::Local(dst) => Instr::Mov { dst, src },
            _ => unreachable!(),
        };

        builder.emit(instr);
        Ok(())
    }

    fn emit_if(&self, builder: &mut Builder, node: &If) -> Result<()> {
        let src = self.emit_expr(builder, node.condition())?;
        let jf_patch = builder.emit_patch(JumpPatch::Jf(src));

        self.emit_block(builder, node.then_body())?;

        if let Some(else_body) = node.else_body() {
            let jmp_patch = builder.emit_patch(JumpPatch::Jmp);
            builder.patch(jf_patch)?;

            self.emit_block(builder, else_body)?;

            builder.patch(jmp_patch)?;
        } else {
            builder.patch(jf_patch)?;
        }

        Ok(())
    }

    fn emit_while(&self, builder: &mut Builder, node: &While) -> Result<()> {
        let start = builder.offset();

        let src = self.emit_expr(builder, node.condition())?;
        let site = builder.emit_patch(JumpPatch::Jf(src));

        self.emit_block(builder, node.body())?;

        builder.emit_jump_to(JumpPatch::Jmp, start)?;
        builder.patch(site)?;

        Ok(())
    }

    fn emit_return(&self, builder: &mut Builder, node: &Return) -> Result<()> {
        let ret = emitter::RET_REG;

        if let Some(value) = node.value() {
            let src = self.emit_expr(builder, value)?;
            builder.emit(Instr::Mov { dst: ret, src });
        } else {
            builder.emit(Instr::Ldu { dst: ret });
        }

        builder.emit(Instr::Ret { src: ret });
        Ok(())
    }
}
