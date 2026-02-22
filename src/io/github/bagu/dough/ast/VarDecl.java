package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record VarDecl(
        String ident,
        TypeRef type,
        Expr init,
        SourceRange range
) implements Decl, Stmt {
    public VarDecl {
        Objects.requireNonNull(ident);
        Objects.requireNonNull(type);
        Objects.requireNonNull(init);
        Objects.requireNonNull(range);
    }

    @Override
    public void accept(DeclVisitor visitor) {
        visitor.visitVarDecl(this);
    }

    @Override
    public void accept(StmtVisitor visitor) {
        visitor.visitVarDecl(this);
    }
}
