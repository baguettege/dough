package io.github.bagu.dough.type;

public enum PrimitiveType implements DoughType {
    STR("str"),
    INT("int"),
    FLOAT("float"),
    BOOL("bool"),
    VOID("void");

    private final String name;

    PrimitiveType(String name) {
        this.name = name;
    }

    @Override
    public String toString() {
        return name;
    }
}
