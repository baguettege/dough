package io.github.bagu.dough.lexer;

import io.github.bagu.dough.exception.DoughException;
import io.github.bagu.dough.source.SourceRange;

public final class LexerException extends DoughException {
    public LexerException(String message, SourceRange range) {
        super(message, range);
    }
}
