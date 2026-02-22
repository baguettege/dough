package io.github.bagu.dough.runtime;

import io.github.bagu.dough.type.DoughType;
import io.github.bagu.dough.type.PrimitiveType;

public record DoughBool(
        boolean value
) implements DoughValue {
    @Override
    public DoughType type() {
        return PrimitiveType.BOOL;
    }

    @Override
    public String toString() {
        return String.valueOf(value);
    }
}
