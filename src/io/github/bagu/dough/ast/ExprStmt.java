package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record ExprStmt(
        Expr expr,
        SourceRange range
) implements Stmt {
    public ExprStmt {
        Objects.requireNonNull(expr);
        Objects.requireNonNull(range);
    }

    @Override
    public void accept(StmtVisitor visitor) {
        visitor.visitExprStmt(this);
    }
}
