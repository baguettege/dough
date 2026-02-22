package io.github.bagu.dough.ast;

public interface DeclVisitor {
    void visitFuncDef(FuncDef decl);
    void visitVarDecl(VarDecl decl);
}
