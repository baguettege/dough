package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record ReturnStmt(
        Expr value,
        SourceRange range
) implements Stmt {
    public ReturnStmt {
        // value is nullable
        Objects.requireNonNull(range);
    }

    public boolean hasValue() {
        return value != null;
    }

    @Override
    public void accept(StmtVisitor visitor) {
        visitor.visitReturnStmt(this);
    }
}
