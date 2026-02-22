package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record TypeRef(
        String name,
        SourceRange range
) {
    public TypeRef {
        Objects.requireNonNull(name);
        Objects.requireNonNull(range);
    }
}
