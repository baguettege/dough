package io.github.bagu.dough.lexer;

import io.github.bagu.dough.source.SourceRange;
import io.github.bagu.dough.token.Token;
import io.github.bagu.dough.token.TokenKind;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Objects;

public final class Lexer {
    private static final char EOF = '\0';
    private static final Map<String, TokenKind> KEYWORDS;

    static {
        KEYWORDS = Map.ofEntries(
                Map.entry("str", TokenKind.STR),
                Map.entry("int", TokenKind.INT),
                Map.entry("float", TokenKind.FLOAT),
                Map.entry("bool", TokenKind.BOOL),
                Map.entry("void", TokenKind.VOID),

                Map.entry("true", TokenKind.TRUE),
                Map.entry("false", TokenKind.FALSE),

                Map.entry("var", TokenKind.VAR),
                Map.entry("func", TokenKind.FUNC),

                Map.entry("if", TokenKind.IF),
                Map.entry("else", TokenKind.ELSE),
                Map.entry("while", TokenKind.WHILE),
                Map.entry("return", TokenKind.RETURN),

                Map.entry("and", TokenKind.AND),
                Map.entry("or", TokenKind.OR),
                Map.entry("not", TokenKind.NOT)
        );
    }

    private final String src;
    private int current = 0;

    public Lexer(String src) {
        this.src = Objects.requireNonNull(src);
    }

    public List<Token> lex() {
        current = 0;
        List<Token> tokens = new ArrayList<>();

        while (!atEnd()) {
            Token token = scanToken();
            if (token.kind() == TokenKind.EOF) break;
            tokens.add(token);
        }

        tokens.add(token(TokenKind.EOF, null, current));

        return tokens;
    }

    private SourceRange range(int start) {
        return new SourceRange(start, current);
    }

    private Token token(TokenKind kind, String lexeme, int start) {
        return new Token(kind, lexeme, range(start));
    }

    private boolean atEnd() {
        return current >= src.length();
    }

    private char advance() {
        return atEnd()
                ? EOF
                : src.charAt(current++);
    }

    private char peek(int offset) {
        int index = current + offset;
        return (index >= src.length())
                ? EOF
                : src.charAt(index);
    }

    private boolean check(char expected) {
        return peek(0) == expected;
    }

    private boolean match(char expected) {
        boolean matches = check(expected);
        if (matches) advance();
        return matches;
    }

    private void skip(int count) {
        for (int i = 0; i < count; i++) {
            advance();
        }
    }

    private Token scanToken() {
        while (true) {
            skipWhitespaces();
            skipComments();

            char peek = peek(0);
            if (Character.isWhitespace(peek) ||
                    (peek == '/' && peek(1) == '*')) {
                continue;
            }
            break;
        }

        if (atEnd()) return token(TokenKind.EOF, null, current);

        int start = current;
        char peek = peek(0);

        if (peek == '"')
            return scanString();

        if (peek == '_' || Character.isLetter(peek))
            return scanIdent();

        if (Character.isDigit(peek))
            return scanNumber();

        skip(1);
        return switch (peek) {
            case '+' -> token(TokenKind.PLUS, "+", start);
            case '-' -> token(TokenKind.MINUS, "-", start);
            case '*' -> token(TokenKind.STAR, "*", start);
            case '/' -> token(TokenKind.SLASH, "/", start);

            case '(' -> token(TokenKind.LPAREN, "(", start);
            case ')' -> token(TokenKind.RPAREN, ")", start);
            case '{' -> token(TokenKind.LBRACE, "{", start);
            case '}' -> token(TokenKind.RBRACE, "}", start);
            case ',' -> token(TokenKind.COMMA, ",", start);
            case ':' -> token(TokenKind.COLON, ":", start);
            case ';' -> token(TokenKind.SEMICOLON, ";", start);

            case '=' -> match('=')
                    ? token(TokenKind.EQ, "==", start)
                    : token(TokenKind.ASSIGN, "=", start);
            case '<' -> match('=')
                    ? token(TokenKind.LE, "<=", start)
                    : token(TokenKind.LT, "<", start);
            case '>' -> match('=')
                    ? token(TokenKind.GE, ">=", start)
                    : token(TokenKind.GT, ">", start);

            case '!' -> {
                if (match('='))
                    yield token(TokenKind.NEQ, "!=", start);
                throw new LexerException("Unexpected character '" + peek + "'", range(start));
            }

            default -> throw new LexerException("Unexpected character '" + peek + "'", range(start));
        };
    }

    private void skipWhitespaces() {
        while (Character.isWhitespace(peek(0))) {
            skip(1);
        }
    }

    private void skipComments() {
        if (check('/') && peek(1) == '*') {
            skip(2);
            int start = current;

            while (!atEnd() && !(check('*') && peek(1) == '/')) {
                skip(1);
            }

            if (atEnd()) throw new LexerException("Unclosed block comment", range(start));
            skip(2);
        }
    }

    private Token scanString() {
        if (!match('"')) throw new AssertionError();

        int start = current;
        StringBuilder sb = new StringBuilder();

        while (!atEnd() && !check('"')) {
            if (check('\\')) {
                char escape = switch (peek(1)) {
                    case '\\' -> '\\';
                    case '"' -> '\"';
                    case 'n' -> '\n';

                    default -> throw new LexerException("Unknown escape", range(current));
                };

                sb.append(escape);
                skip(2);
            } else {
                sb.append(advance());
            }
        }

        if (atEnd()) throw new LexerException("Unclosed string literal", range(start));
        skip(1);

        return token(TokenKind.STR_LIT, sb.toString(), start);
    }

    private Token scanIdent() {
        if (!check('_') && !Character.isLetter(peek(0)))
            throw new AssertionError();

        int start = current;
        StringBuilder sb = new StringBuilder();

        while (!atEnd() && (check('_') || Character.isLetterOrDigit(peek(0)))) {
            sb.append(advance());
        }

        String lexeme = sb.toString();
        TokenKind kind = KEYWORDS.getOrDefault(lexeme, TokenKind.IDENT);
        return token(kind, lexeme, start);
    }

    private Token scanNumber() {
        if (!Character.isDigit(peek(0))) throw new AssertionError();

        int start = current;
        StringBuilder sb = new StringBuilder();
        boolean hasDot = false;

        while (!atEnd() && (check('.') || Character.isDigit(peek(0)))) {
            if (check('.')) {
                if (hasDot)
                    throw new LexerException("Multiple decimal points in float literal", range(start));
                if (!Character.isDigit(peek(1)))
                    throw new LexerException("Expected digit after decimal point", range(start));

                hasDot = true;
            }

            sb.append(advance());
        }

        TokenKind kind = hasDot ? TokenKind.FLOAT_LIT : TokenKind.INT_LIT;
        return token(kind, sb.toString(), start);
    }
}
