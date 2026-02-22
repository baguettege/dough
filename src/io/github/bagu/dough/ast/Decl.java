package io.github.bagu.dough.ast;

public sealed interface Decl extends Node
        permits FuncDef, VarDecl {
    void accept(DeclVisitor visitor);
}
