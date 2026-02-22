package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record Param(
        String ident,
        TypeRef type,
        SourceRange range
) {
    public Param {
        Objects.requireNonNull(ident);
        Objects.requireNonNull(type);
        Objects.requireNonNull(range);
    }
}
