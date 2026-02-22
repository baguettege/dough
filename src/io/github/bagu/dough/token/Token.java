package io.github.bagu.dough.token;

import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public record Token(
        TokenKind kind,
        String lexeme,
        SourceRange range
) {
    public Token {
        Objects.requireNonNull(kind);
        // lexeme is nullable
        Objects.requireNonNull(range);
    }
}
