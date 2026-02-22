package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.List;
import java.util.Objects;

public record FuncDef(
        String ident,
        List<Param> params,
        TypeRef returnType,
        Block body,
        SourceRange range
) implements Decl {
    public FuncDef {
        Objects.requireNonNull(ident);
        params = List.copyOf(params);
        Objects.requireNonNull(returnType);
        Objects.requireNonNull(body);
        Objects.requireNonNull(range);
    }

    @Override
    public void accept(DeclVisitor visitor) {
        visitor.visitFuncDef(this);
    }
}
