package io.github.bagu.dough.ast;

public sealed interface Expr extends Node
        permits BinaryExpr, BoolLiteral, CallExpr,
        FloatLiteral, IntLiteral, StrLiteral,
        UnaryExpr, VarExpr {
    <T> T accept(ExprVisitor<T> visitor);
}
