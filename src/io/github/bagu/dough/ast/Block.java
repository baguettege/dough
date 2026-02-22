package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.List;
import java.util.Objects;

public record Block(
        List<Stmt> stmts,
        SourceRange range
) implements Stmt {
    public Block {
        stmts = List.copyOf(stmts);
        Objects.requireNonNull(range);
    }

    @Override
    public void accept(StmtVisitor visitor) {
        visitor.visitBlock(this);
    }
}
