package io.github.bagu.dough.parser;

import io.github.bagu.dough.exception.DoughException;
import io.github.bagu.dough.source.SourceRange;

public final class ParseException extends DoughException {
    public ParseException(String message, SourceRange range) {
        super(message, range);
    }
}
