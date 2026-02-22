package io.github.bagu.dough.ast;

import io.github.bagu.dough.source.SourceRange;

import java.util.List;
import java.util.Objects;

public record Program(
        List<Decl> decls,
        SourceRange range
) implements Node {
    public Program {
        decls = List.copyOf(decls);
        Objects.requireNonNull(range);
    }

    public List<VarDecl> globalVars() {
        return decls.stream()
                .filter(VarDecl.class::isInstance)
                .map(VarDecl.class::cast)
                .toList();
    }

    public List<FuncDef> functions() {
        return decls.stream()
                .filter(FuncDef.class::isInstance)
                .map(FuncDef.class::cast)
                .toList();
    }
}
