package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record BoolLiteral(
        boolean value,
        SourceRange range
) implements Expr {
    public BoolLiteral {
        Objects.requireNonNull(range);
    }

    @Override
    public <T> T accept(ExprVisitor<T> visitor) {
        return visitor.visitBoolLiteral(this);
    }
}
