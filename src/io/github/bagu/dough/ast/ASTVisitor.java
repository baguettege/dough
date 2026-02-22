package io.github.bagu.dough.ast;

public interface ASTVisitor<T> {
    void visitFuncDef(FuncDef decl);
    void visitVarDecl(VarDecl decl);

    void visitExprStmt(ExprStmt stmt);
    void visitBlock(Block stmt);
    void visitWhileStmt(WhileStmt stmt);
    void visitIfStmt(IfStmt stmt);
    void visitAssignStmt(AssignStmt stmt);
    void visitReturnStmt(ReturnStmt stmt);

    T visitBinaryExpr(BinaryExpr expr);
    T visitUnaryExpr(UnaryExpr expr);
    T visitVarExpr(VarExpr expr);
    T visitCallExpr(CallExpr expr);
    T visitStrLiteral(StrLiteral expr);
    T visitIntLiteral(IntLiteral expr);
    T visitFloatLiteral(FloatLiteral expr);
    T visitBoolLiteral(BoolLiteral expr);
}
