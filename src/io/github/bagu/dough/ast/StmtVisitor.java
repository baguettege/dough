package io.github.bagu.dough.ast;

public interface StmtVisitor {
    void visitVarDecl(VarDecl stmt);
    void visitExprStmt(ExprStmt stmt);
    void visitBlock(Block stmt);
    void visitWhileStmt(WhileStmt stmt);
    void visitIfStmt(IfStmt stmt);
    void visitAssignStmt(AssignStmt stmt);
    void visitReturnStmt(ReturnStmt stmt);
}
