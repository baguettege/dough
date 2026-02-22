package io.github.bagu.dough.printer;

import io.github.bagu.dough.token.Token;

import java.util.List;

public final class TokenDumper {
    private final List<Token> tokens;

    public TokenDumper(List<Token> tokens) {
        this.tokens = List.copyOf(tokens);
    }

    public String dump() {
        StringBuilder sb = new StringBuilder();

        for (Token token : tokens) {
            sb.append(token.kind())
                    .append("(");
            String lexeme = token.lexeme() != null
                    ? "\"" + token.lexeme() + "\""
                    : "";
            sb.append(lexeme)
                    .append(") ")
                    .append(token.range().start())
                    .append("-")
                    .append(token.range().end())
                    .append("\n");
        }

        return sb.toString().trim();
    }
}
