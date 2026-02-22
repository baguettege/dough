package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record VarExpr(
        String ident,
        SourceRange range
) implements Expr {
    public VarExpr {
        Objects.requireNonNull(ident);
        Objects.requireNonNull(range);
    }

    @Override
    public <T> T accept(ExprVisitor<T> visitor) {
        return visitor.visitVarExpr(this);
    }
}
