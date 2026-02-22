package io.github.bagu.dough.semantic;

import io.github.bagu.dough.ast.Expr;
import io.github.bagu.dough.ast.Node;
import io.github.bagu.dough.ast.Program;
import io.github.bagu.dough.type.DoughType;

import java.util.Map;
import java.util.Objects;

public record AnalysisResult(
        Program ast,
        Map<Node, Symbol> symbols,
        Map<Expr, DoughType> types,
        int mainIndex
) {
    public AnalysisResult {
        Objects.requireNonNull(ast);
        symbols = Map.copyOf(symbols);
        types = Map.copyOf(types);
    }
}
