package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record BinaryExpr(
        Expr left,
        BinaryOp op,
        Expr right,
        SourceRange range
) implements Expr {
    public BinaryExpr {
        Objects.requireNonNull(left);
        Objects.requireNonNull(op);
        Objects.requireNonNull(right);
        Objects.requireNonNull(range);
    }

    @Override
    public <T> T accept(ExprVisitor<T> visitor) {
        return visitor.visitBinaryExpr(this);
    }
}
