package io.github.bagu.dough.codegen;

import io.github.bagu.dough.bytecode.Chunk;

import java.util.Objects;

public record Bytecode(
        Chunk topLevel,
        Chunk[] functions
) {
    public Bytecode {
        Objects.requireNonNull(topLevel);
        functions = functions.clone();
    }
}
