package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

public sealed interface Node
        permits Decl, Expr, Program, Stmt {
    SourceRange range();
}
