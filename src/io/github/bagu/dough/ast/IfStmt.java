package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record IfStmt(
        Expr condition,
        Block thenBranch,
        Block elseBranch,
        SourceRange range
) implements Stmt {
    public IfStmt {
        Objects.requireNonNull(condition);
        Objects.requireNonNull(thenBranch);
        // else branch is nullable
        Objects.requireNonNull(range);
    }

    public boolean hasElse() {
        return elseBranch != null;
    }

    @Override
    public void accept(StmtVisitor visitor) {
        visitor.visitIfStmt(this);
    }
}
