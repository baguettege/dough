package io.github.bagu.dough.parser;

import io.github.bagu.dough.ast.*;
import vm.ast.*;
import io.github.bagu.dough.source.SourceRange;
import io.github.bagu.dough.token.Token;
import io.github.bagu.dough.token.TokenKind;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

public final class Parser {
    private static final Map<TokenKind, BinaryOp> BINARY_OPS;
    private static final Map<TokenKind, UnaryOp> UNARY_OPS;
    private static final Set<TokenKind> LITERALS;

    static {
        BINARY_OPS = Map.ofEntries(
                Map.entry(TokenKind.STAR, BinaryOp.MUL),
                Map.entry(TokenKind.SLASH, BinaryOp.DIV),
                Map.entry(TokenKind.PLUS, BinaryOp.ADD),
                Map.entry(TokenKind.MINUS, BinaryOp.SUB),

                Map.entry(TokenKind.GT, BinaryOp.GT),
                Map.entry(TokenKind.LT, BinaryOp.LT),
                Map.entry(TokenKind.GE, BinaryOp.GE),
                Map.entry(TokenKind.LE, BinaryOp.LE),

                Map.entry(TokenKind.EQ, BinaryOp.EQ),
                Map.entry(TokenKind.NEQ, BinaryOp.NEQ),

                Map.entry(TokenKind.AND, BinaryOp.AND),
                Map.entry(TokenKind.OR, BinaryOp.OR)
        );

        UNARY_OPS = Map.ofEntries(
                Map.entry(TokenKind.NOT, UnaryOp.NOT),
                Map.entry(TokenKind.MINUS, UnaryOp.NEG)
        );

        LITERALS = Set.of(
                TokenKind.STR_LIT,
                TokenKind.INT_LIT,
                TokenKind.FLOAT_LIT,
                TokenKind.TRUE,
                TokenKind.FALSE
        );
    }

    private final List<Token> tokens;

    private int current = 0;

    public Parser(List<Token> tokens) {
        this.tokens = List.copyOf(tokens);

        if (tokens.isEmpty())
            throw new IllegalArgumentException("Expected terminating EOF token");

        Token last = tokens.getLast();
        if (last.kind() != TokenKind.EOF)
            throw new IllegalArgumentException("Expected terminating EOF token");

        for (Token token : tokens) {
            if (token == last) break;

            if (token.kind() == TokenKind.EOF)
                throw new IllegalArgumentException("Unexpected EOF token at index " + tokens.indexOf(token));
        }
    }

    public Program parse() {
        current = 0;
        List<Decl> decls = new ArrayList<>();

        while (!atEnd()) {
            decls.add(parseDecl());
        }

        return new Program(decls, range(0));
    }

    private ParseException exception(String message) {
        return new ParseException(message, peek(0).range());
    }

    private SourceRange range(int start) {
        return new SourceRange(start, previous().range().end());
    }

    private int start() {
        return peek(0).range().start();
    }

    private boolean atEnd() {
        return tokens.get(current).kind() == TokenKind.EOF;
    }

    private Token advance() {
        return atEnd()
                ? tokens.getLast()
                : tokens.get(current++);
    }

    private Token peek(int offset) {
        int index = current + offset;
        return (index >= tokens.size())
                ? tokens.getLast()
                : tokens.get(index);
    }

    private Token previous() {
        int index = current - 1;
        if (index < 0) return tokens.getFirst();
        if (index >= tokens.size()) return tokens.getLast();
        return tokens.get(index);
    }

    private boolean check(TokenKind expected) {
        return peek(0).kind() == expected;
    }

    private boolean match(TokenKind expected) {
        boolean matches = check(expected);
        if (matches) advance();
        return matches;
    }

    private void skip(int count) {
        for (int i = 0; i < count; i++) {
            advance();
        }
    }

    private Token expect(TokenKind kind, String message) {
        if (check(kind)) return advance();
        throw exception(message);
    }

    private TypeRef expectTypeRef() {
        Token token = peek(0);
        return switch (token.kind()) {
            case STR, INT, FLOAT, BOOL, VOID, IDENT -> new TypeRef(advance().lexeme(), token.range());
            default -> throw exception("Expected type");
        };
    }

    private Decl parseDecl() {
        return switch (peek(0).kind()) {
            case VAR -> parseVarDecl();
            case FUNC -> parseFuncDef();

            default -> throw exception("Unexpected token \"" + peek(0).lexeme() + "\"");
        };
    }

    private Block parseBody() {
        int start = start();

        List<Stmt> stmts = new ArrayList<>();
        if (match(TokenKind.LBRACE)) {
            while (!match(TokenKind.RBRACE)) {
                if (atEnd()) throw exception("Expected '}'");
                stmts.add(parseStmt());
            }
        } else {
            stmts.add(parseStmt());
        }

        return new Block(stmts, range(start));
    }

    private VarDecl parseVarDecl() {
        int start = start();

        if (!match(TokenKind.VAR)) throw new AssertionError();
        String ident = expect(TokenKind.IDENT, "Expected variable identifier").lexeme();
        expect(TokenKind.COLON, "Expected ':' after variable identifier");
        TypeRef typeRef = expectTypeRef();
        expect(TokenKind.ASSIGN, "Expected '=' after variable type");
        Expr init = parseExpr();
        expect(TokenKind.SEMICOLON, "Expected ';' after variable declaration");

        return new VarDecl(ident, typeRef, init, range(start));
    }

    private FuncDef parseFuncDef() {
        int start = start();

        if (!match(TokenKind.FUNC)) throw new AssertionError();
        String ident = expect(TokenKind.IDENT, "Expected function identifier").lexeme();
        expect(TokenKind.LPAREN, "Expected '(' after function identifier");

        List<Param> params = new ArrayList<>();
        if (!check(TokenKind.RPAREN)) {
            do {
                int paramStart = start();

                String paramIdent = expect(TokenKind.IDENT, "Expected parameter identifier").lexeme();
                expect(TokenKind.COLON, "Expected ':' after parameter identifier");
                TypeRef typeRef = expectTypeRef();

                params.add(new Param(paramIdent, typeRef, range(paramStart)));
            } while (match(TokenKind.COMMA));
        }

        expect(TokenKind.RPAREN, "Expected ')' after function parameters");
        expect(TokenKind.COLON, "Expected ':' after function parameters");

        TypeRef returnType = expectTypeRef();
        Block body = parseBody();

        return new FuncDef(ident, params, returnType, body, range(start));
    }

    private Stmt parseStmt() {
        return switch (peek(0).kind()) {
            case VAR -> parseVarDecl();
            case WHILE -> parseWhileStmt();
            case IF -> parseIfStmt();
            case RETURN -> parseReturnStmt();
            case IDENT -> peek(1).kind() == TokenKind.ASSIGN
                    ? parseAssignStmt()
                    : parseExprStmt();

            default -> parseExprStmt();
        };
    }

    private ExprStmt parseExprStmt() {
        int start = start();

        Expr expr = parseExpr();
        expect(TokenKind.SEMICOLON, "Expected ';' after expression");

        return new ExprStmt(expr, range(start));
    }

    private WhileStmt parseWhileStmt() {
        int start = start();

        if (!match(TokenKind.WHILE)) throw new AssertionError();
        expect(TokenKind.LPAREN, "Expected '(' after \"while\"");
        Expr condition = parseExpr();
        expect(TokenKind.RPAREN, "Expected ')' after while condition");
        Block body = parseBody();

        return new WhileStmt(condition, body, range(start));
    }

    private IfStmt parseIfStmt() {
        int start = start();

        if (!match(TokenKind.IF)) throw new AssertionError();
        expect(TokenKind.LPAREN, "Expected '(' after \"if\"");
        Expr condition = parseExpr();
        expect(TokenKind.RPAREN, "Expected ')' after if condition");
        Block thenBranch = parseBody();
        Block elseBranch = match(TokenKind.ELSE) ? parseBody() : null;

        return new IfStmt(condition, thenBranch, elseBranch, range(start));
    }

    private ReturnStmt parseReturnStmt() {
        int start = start();

        if (!match(TokenKind.RETURN)) throw new AssertionError();
        Expr value = check(TokenKind.SEMICOLON) ? null : parseExpr();
        expect(TokenKind.SEMICOLON, "Expected ';' after return statement");

        return new ReturnStmt(value, range(start));
    }

    private AssignStmt parseAssignStmt() {
        int start = start();

        if (!check(TokenKind.IDENT)) throw new AssertionError();
        String ident = advance().lexeme();
        expect(TokenKind.ASSIGN, "Expected '=' after variable identifier");
        Expr value = parseExpr();
        expect(TokenKind.SEMICOLON, "Expected ';' after assign statement");

        return new AssignStmt(ident, value, range(start));
    }

    private Expr parseExpr() {
        return parseExpr(parseOperand(), BinaryOp.MIN_PRECEDENCE);
    }

    private Expr parseExpr(Expr left, int minPrecedence) {
        int start = left.range().start();

        while (BINARY_OPS.containsKey(peek(0).kind())) {
            BinaryOp op = BINARY_OPS.get(peek(0).kind());
            if (op.precedence < minPrecedence) break;
            advance();

            Expr right = parseOperand();
            right = parseExpr(right, op.precedence + 1);

            left = new BinaryExpr(left, op, right, range(start));
        }

        return left;
    }

    private Expr parseOperand() {
        return UNARY_OPS.containsKey(peek(0).kind())
                ? parseUnaryExpr()
                : parsePrimaryExpr();
    }

    private Expr parseUnaryExpr() {
        int start = start();

        UnaryOp op = UNARY_OPS.get(advance().kind());
        if (op == null) throw new AssertionError();
        Expr operand = parseOperand();

        return new UnaryExpr(op, operand, range(start));
    }

    private Expr parsePrimaryExpr() {
        TokenKind kind = peek(0).kind();

        if (kind == TokenKind.IDENT && peek(1).kind() == TokenKind.LPAREN) {
            return parseCallExpr();
        }
        else if (kind == TokenKind.IDENT) {
            return parseVarExpr();
        }
        else if (LITERALS.contains(kind)) {
            return parseLiteral();
        }
        else if (kind == TokenKind.LPAREN) {
            advance();
            Expr expr = parseExpr();
            expect(TokenKind.RPAREN, "Expected ')' after expression");
            return expr;
        }

        throw exception("Unexpected token \"" + peek(0).lexeme() + "\"");
    }

    private CallExpr parseCallExpr() {
        int start = start();

        if (!check(TokenKind.IDENT)) throw new AssertionError();
        String ident = advance().lexeme();
        if (!match(TokenKind.LPAREN)) throw new AssertionError();

        List<Expr> args = new ArrayList<>();
        if (!check(TokenKind.RPAREN)) {
            do {
                args.add(parseExpr());
            } while (match(TokenKind.COMMA));
        }

        expect(TokenKind.RPAREN, "Expected ')' after call arguments");

        return new CallExpr(ident, args, range(start));
    }

    private VarExpr parseVarExpr() {
        int start = start();

        if (!check(TokenKind.IDENT)) throw new AssertionError();
        String ident = advance().lexeme();

        return new VarExpr(ident, range(start));
    }

    private Expr parseLiteral() {
        int start = start();

        Token token = advance();
        TokenKind kind = token.kind();
        if (!LITERALS.contains(kind)) throw new AssertionError();
        String lexeme = token.lexeme();

        return switch (kind) {
            case STR_LIT -> new StrLiteral(lexeme, range(start));
            case INT_LIT -> new IntLiteral(lexeme, range(start));
            case FLOAT_LIT -> new FloatLiteral(lexeme, range(start));
            case TRUE -> new BoolLiteral(true, range(start));
            case FALSE -> new BoolLiteral(false, range(start));

            default -> throw new AssertionError();
        };
    }
}
