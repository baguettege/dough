package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.List;
import java.util.Objects;

public record CallExpr(
        String ident,
        List<Expr> args,
        SourceRange range
) implements Expr {
    public CallExpr {
        Objects.requireNonNull(ident);
        args = List.copyOf(args);
        Objects.requireNonNull(range);
    }

    @Override
    public <T> T accept(ExprVisitor<T> visitor) {
        return visitor.visitCallExpr(this);
    }
}
