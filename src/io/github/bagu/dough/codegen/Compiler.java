package io.github.bagu.dough.codegen;

import io.github.bagu.dough.ast.*;
import io.github.bagu.dough.bytecode.Chunk;
import io.github.bagu.dough.bytecode.Opcodes;
import io.github.bagu.dough.bytecode.TypeTag;
import io.github.bagu.dough.semantic.AnalysisResult;
import io.github.bagu.dough.semantic.Symbol;
import io.github.bagu.dough.type.PrimitiveType;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public final class Compiler implements DeclVisitor, StmtVisitor, ExprVisitor<Void> {
    private final AnalysisResult analysis;
    private final List<Byte> bytecode = new ArrayList<>();

    public Compiler(AnalysisResult analysis) {
        this.analysis = Objects.requireNonNull(analysis);
    }

    private void emit(byte value) {
        bytecode.add(value);
    }

    private void emitLong(long value) {
        emit((byte) (value >> 56));
        emit((byte) (value >> 48));
        emit((byte) (value >> 40));
        emit((byte) (value >> 32));
        emit((byte) (value >> 24));
        emit((byte) (value >> 16));
        emit((byte) (value >> 8));
        emit((byte) value);
    }

    private void emitShort(short value) {
        emit((byte) (value >> 8));
        emit((byte) value);
    }

    private int currentPos() {
        return bytecode.size();
    }

    private void patchShort(int pos, short value) {
        bytecode.set(pos, (byte) (value >> 8));
        bytecode.set(pos + 1, (byte) value);
    }

    private Chunk getChunk() {
        byte[] code = new byte[bytecode.size()];
        for (int i = 0; i < bytecode.size(); i++) {
            code[i] = bytecode.get(i);
        }

        bytecode.clear();
        return new Chunk(code);
    }

    public Bytecode compile() {
        Program ast = analysis.ast();

        for (VarDecl varDecl : ast.globalVars()) {
            varDecl.accept((DeclVisitor) this);
        }

        emit(Opcodes.CALL);
        emitShort((short) analysis.mainIndex());
        emit((byte) 0);
        emit(Opcodes.HALT);

        Chunk topLevel = getChunk();
        Chunk[] functions = new Chunk[ast.functions().size()];

        for (FuncDef funcDef : ast.functions()) {
            funcDef.accept(this);
            Symbol.Function symbol = (Symbol.Function) analysis.symbols().get(funcDef);
            functions[symbol.index()] = getChunk();
        }

        return new Bytecode(topLevel, functions);
    }

    @Override
    public void visitFuncDef(FuncDef decl) {
        decl.body().accept(this);

        Symbol.Function symbol = (Symbol.Function) analysis.symbols().get(decl);
        if (symbol.returnType() == PrimitiveType.VOID) {
            emit(Opcodes.RET);
            emit((byte) 0);
        }
    }

    @Override
    public void visitVarDecl(VarDecl decl) {
        decl.init().accept(this);

        Symbol symbol = analysis.symbols().get(decl);
        switch (symbol) {
            case Symbol.Local local -> {
                emit(Opcodes.ST);
                emitShort((short) local.slot());
            }
            case Symbol.Global global -> {
                emit(Opcodes.GST);
                emitShort((short) global.index());
            }
            case Symbol.Function ignored -> throw new AssertionError();
        }
    }

    @Override
    public void visitExprStmt(ExprStmt stmt) {
        stmt.expr().accept(this);

        if (analysis.types().get(stmt.expr()) != PrimitiveType.VOID) {
            emit(Opcodes.POP);
        }
    }

    @Override
    public void visitBlock(Block stmt) {
        for (Stmt s : stmt.stmts()) {
            s.accept(this);
        }
    }

    @Override
    public void visitWhileStmt(WhileStmt stmt) {
        /*
        start addr
        JF patch
        body
        JMP start addr
        patch addr
         */

        int startPos = currentPos();
        stmt.condition().accept(this);

        emit(Opcodes.JF);
        int patchPos = currentPos();
        emitShort((short) 0);

        stmt.body().accept(this);

        emit(Opcodes.JMP);
        emitShort((short) (startPos - currentPos() - 2));

        patchShort(patchPos, (short) (currentPos() - patchPos - 2));

        // -2 cancels the emitted short
    }

    @Override
    public void visitIfStmt(IfStmt stmt) {
        /*
        if:
        JF patch
        thenBranch body
        patch addr

        if else:
        JF patch
        thenBranch body
        JMP patch
        jfPatch addr
        elseBranch body
        jmpPatch addr
         */

        stmt.condition().accept(this);
        emit(Opcodes.JF);

        int jfPatchPos = currentPos();
        emitShort((short) 0);

        stmt.thenBranch().accept(this);

        if (stmt.hasElse()) {
            emit(Opcodes.JMP);
            int jmpPatchPos = currentPos();
            emitShort((short) 0);

            //
            int jfEndPos = currentPos();
            patchShort(jfPatchPos, (short) (jfEndPos - jfPatchPos - 2));
            //

            stmt.elseBranch().accept(this);

            int jmpEndPos = currentPos();
            patchShort(jmpPatchPos, (short) (jmpEndPos - jmpPatchPos - 2));
        } else {
            //
            int jfEndPos = currentPos();
            patchShort(jfPatchPos, (short) (jfEndPos - jfPatchPos - 2));
            //
        }
    }

    @Override
    public void visitAssignStmt(AssignStmt stmt) {
        stmt.value().accept(this);

        Symbol symbol = analysis.symbols().get(stmt);
        switch (symbol) {
            case Symbol.Local local -> {
                emit(Opcodes.ST);
                emitShort((short) local.slot());
            }
            case Symbol.Global global -> {
                emit(Opcodes.GST);
                emitShort((short) global.index());
            }
            case Symbol.Function ignored -> throw new AssertionError();
        }
    }

    @Override
    public void visitReturnStmt(ReturnStmt stmt) {
        if (stmt.hasValue()) {
            stmt.value().accept(this);
        }

        emit(Opcodes.RET);
        emit((byte) (stmt.hasValue() ? 1 : 0));
    }

    @Override
    public Void visitBinaryExpr(BinaryExpr expr) {
        Expr a = expr.left();
        Expr b = expr.right();

        switch (expr.op()) {
            case MUL -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.MUL);
            }
            case DIV -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.DIV);
            }
            case ADD -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.ADD);
            }
            case SUB -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.SUB);
            }
            case GT -> {
                b.accept(this);
                a.accept(this);
                emit(Opcodes.LT);
            }
            case LT -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.LT);
            }
            case GE -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.LT);
                emit(Opcodes.NOT);
            }
            case LE -> {
                b.accept(this);
                a.accept(this);
                emit(Opcodes.LT);
                emit(Opcodes.NOT);
            }
            case EQ -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.EQ);
            }
            case NEQ -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.EQ);
                emit(Opcodes.NOT);
            }
            case AND -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.AND);
            }
            case OR -> {
                a.accept(this);
                b.accept(this);
                emit(Opcodes.OR);
            }
        }

        return null;
    }

    @Override
    public Void visitUnaryExpr(UnaryExpr expr) {
        expr.expr().accept(this);

        switch (expr.op()) {
            case NOT -> emit(Opcodes.NOT);
            case NEG -> emit(Opcodes.NEG);
        }

        return null;
    }

    @Override
    public Void visitVarExpr(VarExpr expr) {
        Symbol symbol = analysis.symbols().get(expr);
        switch (symbol) {
            case Symbol.Local local -> {
                emit(Opcodes.LD);
                emitShort((short) local.slot());
            }
            case Symbol.Global global -> {
                emit(Opcodes.GLD);
                emitShort((short) global.index());
            }
            case Symbol.Function ignored -> throw new AssertionError();
        }

        return null;
    }

    @Override
    public Void visitCallExpr(CallExpr expr) {
        for (Expr arg : expr.args()) {
            arg.accept(this);
        }

        emit(Opcodes.CALL);
        Symbol.Function symbol = (Symbol.Function) analysis.symbols().get(expr);
        emitShort((short) symbol.index());
        emit((byte) symbol.paramTypes().size());

        return null;
    }

    @Override
    public Void visitStrLiteral(StrLiteral expr) {
        emit(Opcodes.PUSH);
        emit(TypeTag.STR);

        byte[] bytes = expr.value().getBytes(StandardCharsets.UTF_8);
        emitShort((short) bytes.length);

        for (byte b : bytes) {
            emit(b);
        }

        return null;
    }

    @Override
    public Void visitIntLiteral(IntLiteral expr) {
        emit(Opcodes.PUSH);
        emit(TypeTag.INT);

        long bits = Long.parseLong(expr.value());
        emitLong(bits);

        return null;
    }

    @Override
    public Void visitFloatLiteral(FloatLiteral expr) {
        emit(Opcodes.PUSH);
        emit(TypeTag.FLOAT);

        long bits = Double.doubleToRawLongBits(Double.parseDouble(expr.value()));
        emitLong(bits);

        return null;
    }

    @Override
    public Void visitBoolLiteral(BoolLiteral expr) {
        emit(Opcodes.PUSH);
        emit(TypeTag.BOOL);
        emit((byte) (expr.value() ? 1 : 0));

        return null;
    }
}
