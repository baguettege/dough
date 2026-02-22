package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record FloatLiteral(
        String value,
        SourceRange range
) implements Expr {
    public FloatLiteral {
        Objects.requireNonNull(value);
        Objects.requireNonNull(range);
    }

    @Override
    public <T> T accept(ExprVisitor<T> visitor) {
        return visitor.visitFloatLiteral(this);
    }
}
