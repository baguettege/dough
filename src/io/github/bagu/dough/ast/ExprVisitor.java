package io.github.bagu.dough.ast;

public interface ExprVisitor<T> {
    T visitBinaryExpr(BinaryExpr expr);
    T visitUnaryExpr(UnaryExpr expr);
    T visitVarExpr(VarExpr expr);
    T visitCallExpr(CallExpr expr);
    T visitStrLiteral(StrLiteral expr);
    T visitIntLiteral(IntLiteral expr);
    T visitFloatLiteral(FloatLiteral expr);
    T visitBoolLiteral(BoolLiteral expr);
}
