package io.github.bagu.dough.semantic;

import io.github.bagu.dough.exception.DoughException;
import io.github.bagu.dough.source.SourceRange;

public final class SemanticException extends DoughException {
    public SemanticException(String message, SourceRange range) {
        super(message, range);
    }
}
