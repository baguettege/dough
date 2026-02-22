package io.github.bagu.dough.ast;

public sealed interface Stmt extends Node
        permits AssignStmt, Block, ExprStmt,
        IfStmt, ReturnStmt, VarDecl, WhileStmt {
    void accept(StmtVisitor visitor);
}
