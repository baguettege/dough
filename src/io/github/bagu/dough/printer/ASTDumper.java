package io.github.bagu.dough.printer;

import io.github.bagu.dough.ast.*;
import vm.ast.*;

import java.util.Objects;

/*
│
├
└
─
 */

public final class ASTDumper {
    private final Program ast;
    private final StringBuilder sb = new StringBuilder();

    private int indent = 0;

    public ASTDumper(Program ast) {
        this.ast = Objects.requireNonNull(ast);
    }

    public String dump() {
        indent = 0;
        sb.setLength(0);
        dumpProgram(ast);
        return sb.toString().trim();
    }

    private void sb(Object... objects) {
        sb.append("    ".repeat(indent));
        for (Object o : objects) {
            sb.append(o);
        }
        sb.append("\n");
    }

    private void dumpHeader(Node node) {
        sb(node.getClass().getSimpleName(),
                "(", node.range().start(),
                "-", node.range().end(), ")");
    }

    private String type(TypeRef typeRef) {
        return typeRef.name();
    }

    private void dumpDecl(Decl decl) {
        switch (decl) {
            case VarDecl d -> dumpVarDecl(d);
            case FuncDef d -> dumpFuncDef(d);
        }
    }

    private void dumpStmt(Stmt stmt) {
        switch (stmt) {
            case ExprStmt s   -> dumpExprStmt(s);
            case Block s      -> dumpBlock(s);
            case VarDecl s    -> dumpVarDecl(s);
            case IfStmt s     -> dumpIfStmt(s);
            case WhileStmt s  -> dumpWhileStmt(s);
            case AssignStmt s -> dumpAssignStmt(s);
            case ReturnStmt s -> dumpReturnStmt(s);
        }
    }

    private void dumpExpr(Expr expr) {
        switch (expr) {
            case BinaryExpr e   -> dumpBinaryExpr(e);
            case UnaryExpr e    -> dumpUnaryExpr(e);
            case VarExpr e      -> dumpVarExpr(e);
            case CallExpr e     -> dumpCallExpr(e);
            case StrLiteral e   -> dumpStrLiteral(e);
            case IntLiteral e   -> dumpIntLiteral(e);
            case FloatLiteral e -> dumpFloatLiteral(e);
            case BoolLiteral e  -> dumpBoolLiteral(e);
        }
    }

    private void dumpProgram(Program node) {
        dumpHeader(node);

        indent++;
        for (Decl decl : node.decls()) {
            dumpDecl(decl);
            sb.append("\n");
        }
        indent--;
    }

    private void dumpVarDecl(VarDecl node) {
        dumpHeader(node);

        indent++;
        sb("ident=", node.ident());
        sb("type=", type(node.type()));
        sb("init=");
        indent++; dumpExpr(node.init()); indent--;
        indent--;
    }

    private void dumpFuncDef(FuncDef node) {
        dumpHeader(node);

        indent++;
        sb("ident=", node.ident());
        if (!node.params().isEmpty()) sb("paramTypes=");
        indent++;
        for (Param param : node.params()) {
            sb("param=");
            indent++;
            sb("ident=", param.ident());
            sb("type=", type(param.type()));
            indent--;
        }
        indent--;
        sb("returnType=", type(node.returnType()));
        sb("body=");
        indent++; dumpBlock(node.body()); indent--;
        indent--;
    }

    private void dumpBlock(Block node) {
        dumpHeader(node);

        indent++;
        for (Stmt stmt : node.stmts()) {
            dumpStmt(stmt);
            sb.append("\n");
        }
        indent--;
    }

    private void dumpExprStmt(ExprStmt node) {
        dumpHeader(node);

        indent++;
        dumpExpr(node.expr());
        indent--;
    }

    private void dumpIfStmt(IfStmt node) {
        dumpHeader(node);

        indent++;
        sb("condition=");
        indent++;
        dumpExpr(node.condition());
        indent--;
        sb("thenBranch=");
        indent++;
        dumpBlock(node.thenBranch());
        indent--;
        if (node.hasElse()) {
            sb("elseBranch=");
            indent++;
            dumpBlock(node.elseBranch());
            indent--;
        }
        indent--;
    }

    private void dumpWhileStmt(WhileStmt node) {
        dumpHeader(node);

        indent++;
        sb("condition=");
        indent++;
        dumpExpr(node.condition());
        indent--;
        sb("body=");
        indent++;
        dumpBlock(node.body());
        indent--;
        indent--;
    }

    private void dumpAssignStmt(AssignStmt node) {
        dumpHeader(node);

        indent++;
        sb("ident=", node.ident());
        sb("value=");
        indent++;
        dumpExpr(node.value());
        indent--;
        indent--;
    }

    private void dumpReturnStmt(ReturnStmt node) {
        dumpHeader(node);

        indent++;
        if (node.hasValue()) {
            sb("value=");
            indent++;
            dumpExpr(node.value());
            indent--;
        }
        indent--;
    }

    private void dumpBinaryExpr(BinaryExpr node) {
        dumpHeader(node);

        indent++;
        sb("left=");
        indent++;
        dumpExpr(node.left());
        indent--;
        sb("op=", node.op());
        sb("right=");
        indent++;
        dumpExpr(node.right());
        indent--;
        indent--;
    }

    private void dumpUnaryExpr(UnaryExpr node) {
        dumpHeader(node);

        indent++;
        sb("op=", node.op());
        sb("expr=");
        indent++;
        dumpExpr(node.expr());
        indent--;
        indent--;
    }

    private void dumpVarExpr(VarExpr node) {
        dumpHeader(node);

        indent++;
        sb("ident=", node.ident());
        indent--;
    }

    private void dumpCallExpr(CallExpr node) {
        dumpHeader(node);

        indent++;
        sb("ident=", node.ident());
        if (!node.args().isEmpty()) sb("args=");
        indent++;
        for (Expr arg : node.args()) {
            sb("arg=");
            indent++;
            dumpExpr(arg);
            indent--;
        }
        indent--;
        indent--;
    }

    private void dumpStrLiteral(StrLiteral node) {
        dumpHeader(node);

        indent++;
        sb("value=", node.value());
        indent--;
    }

    private void dumpIntLiteral(IntLiteral node) {
        dumpHeader(node);

        indent++;
        sb("value=", node.value());
        indent--;
    }

    private void dumpFloatLiteral(FloatLiteral node) {
        dumpHeader(node);

        indent++;
        sb("value=", node.value());
        indent--;
    }

    private void dumpBoolLiteral(BoolLiteral node) {
        dumpHeader(node);

        indent++;
        sb("value=", node.value());
        indent--;
    }
}
