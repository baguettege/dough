package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record AssignStmt(
        String ident,
        Expr value,
        SourceRange range
) implements Stmt {
    public AssignStmt {
        Objects.requireNonNull(ident);
        Objects.requireNonNull(value);
        Objects.requireNonNull(range);
    }

    @Override
    public void accept(StmtVisitor visitor) {
        visitor.visitAssignStmt(this);
    }
}
