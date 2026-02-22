package io.github.bagu.dough.exception;

import io.github.bagu.dough.lexer.LexerException;
import io.github.bagu.dough.parser.ParseException;
import io.github.bagu.dough.semantic.SemanticException;
import io.github.bagu.dough.source.SourceRange;

import java.util.Objects;

public sealed abstract class DoughException extends RuntimeException
        permits LexerException, ParseException, SemanticException {
    private final SourceRange range;

    public DoughException(String message, SourceRange range) {
        super(message);
        this.range = Objects.requireNonNull(range);
    }

    public SourceRange getRange() {
        return range;
    }
}
