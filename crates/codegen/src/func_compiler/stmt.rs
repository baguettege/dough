use ast::typed::{Assign, ExprStmt, If, Let, Return, Stmt, While};
use bytecode::{Instr, JumpKind};
use crate::func_compiler::FuncCompiler;

impl FuncCompiler<'_> {
    pub(super) fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::ExprStmt(node) => self.compile_expr_stmt(node),
            Stmt::Let(node) => self.compile_let(node),
            Stmt::Assign(node) => self.compile_assign(node),
            Stmt::If(node) => self.compile_if(node),
            Stmt::While(node) => self.compile_while(node),
            Stmt::Return(node) => self.compile_return(node),
        }
    }

    fn compile_expr_stmt(&mut self, node: &ExprStmt) {
        self.compile_expr(node.expr());
        self.chunk.emit(Instr::Pop);
    }

    fn compile_let(&mut self, node: &Let) {
        let idx = self.locals.alloc(node);
        self.compile_expr(node.init());
        self.chunk.emit(Instr::Stl(idx));
    }

    fn compile_assign(&mut self, node: &Assign) {
        let idx = self.locals.get(*node.binding());
        self.compile_expr(node.value());
        self.chunk.emit(Instr::Stl(idx));
    }

    fn compile_if(&mut self, node: &If) {
        self.compile_expr(node.condition());
        let jf_site = self.chunk.emit_patch(JumpKind::Jf);

        self.compile_block(node.then_body());

        if let Some(else_body) = node.else_body() {
            let jmp_site = self.chunk.emit_patch(JumpKind::Jmp);
            self.chunk.patch(jf_site);

            self.compile_block(else_body);
            self.chunk.patch(jmp_site);
        } else {
            self.chunk.patch(jf_site);
        }
    }

    fn compile_while(&mut self, node: &While) {
        let start = self.chunk.offset();
        self.compile_expr(node.condition());

        let site = self.chunk.emit_patch(JumpKind::Jf);
        self.compile_block(node.body());

        self.chunk.emit_jump(JumpKind::Jmp, start);
        self.chunk.patch(site);
    }

    fn compile_return(&mut self, node: &Return) {
        if let Some(value) = node.value() {
            self.compile_expr(value);
        } else {
            self.chunk.emit(Instr::PushU);
        }

        self.chunk.emit(Instr::Ret);
    }
}
