package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record UnaryExpr(
        UnaryOp op,
        Expr expr,
        SourceRange range
) implements Expr {
    public UnaryExpr {
        Objects.requireNonNull(op);
        Objects.requireNonNull(expr);
        Objects.requireNonNull(range);
    }

    @Override
    public <T> T accept(ExprVisitor<T> visitor) {
        return visitor.visitUnaryExpr(this);
    }
}
