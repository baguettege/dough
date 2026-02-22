package io.github.bagu.dough.ast;

public enum BinaryOp {
    MUL(5, "*"),
    DIV(5, "/"),

    ADD(4, "+"),
    SUB(4, "-"),

    GT(3, ">"),
    LT(3, "<"),
    GE(3, ">="),
    LE(3, "<="),

    EQ(2, "=="),
    NEQ(2, "!="),

    AND(1, "and"),

    OR(0, "or");

    public static final int MIN_PRECEDENCE = Integer.MIN_VALUE;

    public final int precedence;
    public final String symbol;

    BinaryOp(int precedence, String symbol) {
        this.precedence = precedence;
        this.symbol = symbol;
    }

    @Override
    public String toString() {
        return symbol;
    }
}
