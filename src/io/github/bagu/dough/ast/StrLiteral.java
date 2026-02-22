package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record StrLiteral(
        String value,
        SourceRange range
) implements Expr {
    public StrLiteral {
        Objects.requireNonNull(value);
        Objects.requireNonNull(range);
    }

    @Override
    public <T> T accept(ExprVisitor<T> visitor) {
        return visitor.visitStrLiteral(this);
    }
}
