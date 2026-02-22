package io.github.bagu.dough.ast;

public enum UnaryOp {
    NOT("not"),
    NEG("-");

    public final String symbol;

    UnaryOp(String symbol) {
        this.symbol = symbol;
    }

    @Override
    public String toString() {
        return symbol;
    }
}
