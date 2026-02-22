package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record WhileStmt(
        Expr condition,
        Block body,
        SourceRange range
) implements Stmt {
    public WhileStmt {
        Objects.requireNonNull(condition);
        Objects.requireNonNull(body);
        Objects.requireNonNull(range);
    }

    @Override
    public void accept(StmtVisitor visitor) {
        visitor.visitWhileStmt(this);
    }
}
