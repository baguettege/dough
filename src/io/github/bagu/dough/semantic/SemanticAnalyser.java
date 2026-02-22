package io.github.bagu.dough.semantic;

import io.github.bagu.dough.ast.*;
import io.github.bagu.dough.source.SourceRange;
import io.github.bagu.dough.type.DoughType;
import io.github.bagu.dough.type.PrimitiveType;

import java.util.*;

public final class SemanticAnalyser
        implements DeclVisitor, StmtVisitor, ExprVisitor<DoughType> {
    private static final class Scope {
        private final Scope parent;
        private final Map<String, Symbol> symbols = new HashMap<>();

        private Scope(Scope parent) {
            // nullable
            this.parent = parent;
        }

        private void define(String name, Symbol symbol) {
            symbols.put(name, symbol);
        }

        private Symbol lookup(String name) {
            if (symbols.containsKey(name)) return symbols.get(name);
            if (parent != null) return parent.lookup(name);
            return null;
        }

        private boolean definedLocally(String name) {
            return symbols.containsKey(name);
        }
    }

    private final Program ast;
    private final Deque<Scope> scopes = new ArrayDeque<>();
    private final Map<Node, Symbol> symbols = new HashMap<>();
    private final Map<Expr, DoughType> types = new HashMap<>();

    private int globalIndex = 0;
    private int functionIndex = 0;
    private int localSlot = 0;

    private DoughType currentReturnType = null;

    public SemanticAnalyser(Program ast) {
        this.ast = Objects.requireNonNull(ast);
    }

    private void enterScope() {
        scopes.push(new Scope(scopes.peek()));
    }

    private void exitScope() {
        scopes.pop();
    }

    private Scope currentScope() {
        return scopes.peek();
    }

    private DoughType resolveType(TypeRef ref) {
        return switch (ref.name()) {
            case "str" -> PrimitiveType.STR;
            case "int" -> PrimitiveType.INT;
            case "float" -> PrimitiveType.FLOAT;
            case "bool" -> PrimitiveType.BOOL;
            case "void" -> PrimitiveType.VOID;

            default -> throw new SemanticException("Unknown type \"" + ref.name() + "\"", ref.range());
        };
    }

    private void collectTopLevel() {
        for (Decl decl : ast.decls()) {
            switch (decl) {
                case VarDecl d -> registerGlobalVar(d);
                case FuncDef d -> registerFunction(d);
            }
        }
    }

    private void registerGlobalVar(VarDecl decl) {
        if (currentScope().definedLocally(decl.ident()))
            throw new SemanticException("Global \"" + decl.ident() + "\" already defined", decl.range());

        DoughType type = resolveType(decl.type());
        if (type == PrimitiveType.VOID)
            throw new SemanticException("Cannot declare variable of type void", decl.range());

        int index = globalIndex++;
        Symbol.Global symbol = new Symbol.Global(type, index);

        currentScope().define(decl.ident(), symbol);
        symbols.put(decl, symbol);
    }

    private void registerFunction(FuncDef decl) {
        if (currentScope().definedLocally(decl.ident()))
            throw new SemanticException("Function \"" + decl.ident() + "\" already defined", decl.range());

        List<DoughType> paramTypes = new ArrayList<>();
        for (Param param : decl.params()) {
            DoughType paramType = resolveType(param.type());
            paramTypes.add(paramType);
        }

        DoughType returnType = resolveType(decl.returnType());
        int index = functionIndex++;
        Symbol.Function symbol = new Symbol.Function(paramTypes, returnType, index);

        currentScope().define(decl.ident(),symbol);
        symbols.put(decl, symbol);
    }

    private boolean alwaysReturns(Block stmt) {
        for (Stmt s : stmt.stmts()) {
            if (alwaysReturns(s)) return true;
        }
        return false;
    }

    private boolean alwaysReturns(Stmt stmt) {
        return switch (stmt) {
            case ReturnStmt ignored -> true;
            case Block s -> alwaysReturns(s);
            case IfStmt s -> s.hasElse() &&
                    alwaysReturns(s.thenBranch()) &&
                    alwaysReturns(s.elseBranch());

            default -> false;
        };
    }

    private boolean isGlobal() {
        return currentReturnType == null;
    }

    public AnalysisResult analyse() {
        enterScope();
        collectTopLevel();

        Symbol mainSymbol = currentScope().lookup("main");
        if (mainSymbol == null)
            throw new SemanticException("No main function defined", new SourceRange(0, 0));

        if (!(mainSymbol instanceof Symbol.Function main))
            throw new SemanticException("\"main\" must be a function", new SourceRange(0, 0));

        if (!main.paramTypes().isEmpty())
            throw new SemanticException("Main function must be zero-arg", new SourceRange(0, 0));

        if (main.returnType() != PrimitiveType.VOID)
            throw new SemanticException("Main function must return void", new SourceRange(0, 0));

        for (Decl decl : ast.decls()) {
            decl.accept(this);
        }

        exitScope();
        return new AnalysisResult(ast, symbols, types, main.index());
    }

    @Override
    public void visitFuncDef(FuncDef decl) {
        currentReturnType = resolveType(decl.returnType());
        localSlot = 0;
        enterScope();

        for (Param param : decl.params()) {
            if (currentScope().definedLocally(param.ident()))
                throw new SemanticException("Parameter \"" + param.ident() + "\" already defined", param.range());

            DoughType paramType = resolveType(param.type());
            if (paramType == PrimitiveType.VOID)
                throw new SemanticException("Cannot declare parameter of type void", param.range());

            int slot = localSlot++;
            Symbol.Local symbol = new Symbol.Local(paramType, slot);

            currentScope().define(param.ident(), symbol);
        }

        decl.body().accept(this);

        if (currentReturnType != PrimitiveType.VOID && !alwaysReturns(decl.body()))
            throw new SemanticException("Function does not always return on all branches", decl.range());

        exitScope();
        currentReturnType = null;
    }

    @Override
    public void visitVarDecl(VarDecl decl) {
        if (!isGlobal() && currentScope().definedLocally(decl.ident()))
            throw new SemanticException("Variable \"" + decl.ident() + "\" already defined", decl.range());

        DoughType type = resolveType(decl.type());
        if (type == PrimitiveType.VOID)
            throw new SemanticException("Cannot declare variable of type void", decl.range());

        DoughType initType = decl.init().accept(this);
        if (initType != type)
            throw new SemanticException("Expected " + type + ", got " + initType, decl.range());

        if (!isGlobal()) {
            // inside a function
            int slot = localSlot++;
            Symbol.Local symbol = new Symbol.Local(type, slot);

            currentScope().define(decl.ident(), symbol);
            symbols.put(decl, symbol);
        }
        // global vars already registered in pass 1
    }

    @Override
    public void visitExprStmt(ExprStmt stmt) {
        stmt.expr().accept(this);
    }

    @Override
    public void visitBlock(Block stmt) {
        for (Stmt s : stmt.stmts()) {
            s.accept(this);
        }
    }

    @Override
    public void visitWhileStmt(WhileStmt stmt) {
        DoughType condType = stmt.condition().accept(this);
        if (condType != PrimitiveType.BOOL)
            throw new SemanticException(
                    "Expected bool, got " + condType.toString(), stmt.condition().range());

        enterScope();
        stmt.body().accept(this);
        exitScope();
    }

    @Override
    public void visitIfStmt(IfStmt stmt) {
        DoughType condType = stmt.condition().accept(this);
        if (condType != PrimitiveType.BOOL)
            throw new SemanticException(
                    "Expected bool, got " + condType, stmt.condition().range());

        enterScope();
        stmt.thenBranch().accept(this);
        exitScope();

        if (stmt.hasElse()) {
            enterScope();
            stmt.elseBranch().accept(this);
            exitScope();
        }
    }

    @Override
    public void visitAssignStmt(AssignStmt stmt) {
        Symbol symbol = currentScope().lookup(stmt.ident());
        if (symbol == null)
            throw new SemanticException("Undefined variable \"" + stmt.ident() + "\"", stmt.range());

        DoughType varType = switch (symbol) {
            case Symbol.Global global -> global.type();
            case Symbol.Local local -> local.type();
            case Symbol.Function ignored -> throw new SemanticException(
                    "\"" + stmt.ident() + "\" is not a variable", stmt.range());
        };

        DoughType valueType = stmt.value().accept(this);
        if (varType != valueType)
            throw new SemanticException(
                    "Expected " + varType + ", got " + valueType, stmt.range());

        symbols.put(stmt, symbol);
    }

    @Override
    public void visitReturnStmt(ReturnStmt stmt) {
        if (isGlobal()) throw new AssertionError("Return outside of function");

        if (stmt.hasValue()) {
            DoughType returnType = stmt.value().accept(this);
            if (returnType != currentReturnType) {
                throw new SemanticException(
                        "Expected " + currentReturnType + ", got " + returnType, stmt.range());
            }
        } else {
            if (currentReturnType != PrimitiveType.VOID) {
                throw new SemanticException(
                        "Expected " + currentReturnType + ", got void", stmt.range());
            }
        }
    }

    @Override
    public DoughType visitBinaryExpr(BinaryExpr expr) {
        DoughType leftType = expr.left().accept(this);
        DoughType rightType = expr.right().accept(this);
        BinaryOp op = expr.op();

        if (leftType == PrimitiveType.VOID || rightType == PrimitiveType.VOID)
            throw new SemanticException("Cannot apply operator to void", expr.range());

        if (leftType != rightType)
            throw new SemanticException("Type mismatch: " + leftType + " and " + rightType, expr.range());

        if (leftType == PrimitiveType.STR) {
            if (op != BinaryOp.ADD && op != BinaryOp.EQ
                    && op != BinaryOp.NEQ) {
                throw new SemanticException("Cannot apply \"" + op + "\" to str", expr.range());
            }
        }

        else if (leftType == PrimitiveType.INT || leftType == PrimitiveType.FLOAT) {
            if (op != BinaryOp.ADD && op != BinaryOp.SUB
                    && op != BinaryOp.MUL && op != BinaryOp.DIV
                    && op != BinaryOp.EQ && op != BinaryOp.NEQ
                    && op != BinaryOp.GT && op != BinaryOp.LT
                    && op != BinaryOp.GE && op != BinaryOp.LE) {
                throw new SemanticException("Cannot apply \"" + op + "\" to " + leftType, expr.range());
            }
        }

        else if (leftType == PrimitiveType.BOOL) {
            if (op != BinaryOp.AND && op != BinaryOp.OR
                    && op != BinaryOp.EQ && op != BinaryOp.NEQ) {
                throw new SemanticException("Cannot apply \"" + op + "\" to bool", expr.range());
            }
        }

        DoughType resultType = switch (op) {
            case EQ, NEQ, GT, LT, GE, LE -> PrimitiveType.BOOL;
            default -> leftType;
        };

        types.put(expr, resultType);
        return resultType;
    }

    @Override
    public DoughType visitUnaryExpr(UnaryExpr expr) {
        DoughType exprType = expr.expr().accept(this);
        UnaryOp op = expr.op();

        if (op == UnaryOp.NEG) {
            if (exprType != PrimitiveType.INT && exprType != PrimitiveType.FLOAT) {
                throw new SemanticException("Cannot apply \"" + op + "\" to " + exprType, expr.range());
            }
        }

        else if (op == UnaryOp.NOT) {
            if (exprType != PrimitiveType.BOOL) {
                throw new SemanticException("Cannot apply \"" + op + "\" to " + exprType, expr.range());
            }
        }

        types.put(expr, exprType);
        return exprType;
    }

    @Override
    public DoughType visitVarExpr(VarExpr expr) {
        Symbol symbol = currentScope().lookup(expr.ident());
        if (symbol == null)
            throw new SemanticException("Undefined variable \"" + expr.ident() + "\"", expr.range());

        DoughType type = switch (symbol) {
            case Symbol.Global global -> global.type();
            case Symbol.Local local -> local.type();
            case Symbol.Function ignored -> throw new SemanticException(
                    "\"" + expr.ident() + "\" is not a variable", expr.range());
        };

        symbols.put(expr, symbol);
        types.put(expr, type);
        return type;
    }

    @Override
    public DoughType visitCallExpr(CallExpr expr) {
        Symbol symbol = currentScope().lookup(expr.ident());
        if (symbol == null)
            throw new SemanticException("Undefined function \"" + expr.ident() + "\"", expr.range());

        if (!(symbol instanceof Symbol.Function funcSymbol))
            throw new SemanticException("\"" + expr.ident() + "\" is not a function", expr.range());

        List<Expr> args = expr.args();
        List<DoughType> paramTypes = funcSymbol.paramTypes();

        if (args.size() != paramTypes.size())
            throw new SemanticException(
                    "Expected " + paramTypes.size() + " args, got " + args.size(), expr.range());

        for (int i = 0; i < args.size(); i++) {
            DoughType argType = args.get(i).accept(this);
            DoughType paramType = paramTypes.get(i);

            if (argType != paramType)
                throw new SemanticException("Expected " + paramType + ", got " + argType, expr.range());
        }

        DoughType returnType = funcSymbol.returnType();
        symbols.put(expr, funcSymbol);
        types.put(expr, returnType);
        return returnType;
    }

    @Override
    public DoughType visitStrLiteral(StrLiteral expr) {
        types.put(expr, PrimitiveType.STR);
        return PrimitiveType.STR;
    }

    @Override
    public DoughType visitIntLiteral(IntLiteral expr) {
        try {
            Long.parseLong(expr.value());
        } catch (NumberFormatException e) {
            throw new SemanticException("Integer literal out of range", expr.range());
        }

        types.put(expr, PrimitiveType.INT);
        return PrimitiveType.INT;
    }

    @Override
    public DoughType visitFloatLiteral(FloatLiteral expr) {
        try {
            double value = Double.parseDouble(expr.value());
            if (Double.isInfinite(value) || Double.isNaN(value))
                throw new SemanticException("Float literal out of range", expr.range());
        } catch (NumberFormatException e) {
            throw new SemanticException("Float literal out of range", expr.range());
        }

        types.put(expr, PrimitiveType.FLOAT);
        return PrimitiveType.FLOAT;
    }

    @Override
    public DoughType visitBoolLiteral(BoolLiteral expr) {
        types.put(expr, PrimitiveType.BOOL);
        return PrimitiveType.BOOL;
    }
}
