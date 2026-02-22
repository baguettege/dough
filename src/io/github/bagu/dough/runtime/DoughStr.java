package io.github.bagu.dough.runtime;

import io.github.bagu.dough.type.DoughType;
import io.github.bagu.dough.type.PrimitiveType;

import java.util.Objects;

public record DoughStr(
        String value
) implements DoughValue {
    public DoughStr {
        Objects.requireNonNull(value);
    }

    @Override
    public DoughType type() {
        return PrimitiveType.STR;
    }

    @Override
    public DoughValue add(DoughValue other) {
        if (other instanceof DoughStr str) {
            return new DoughStr(value + str.value);
        }

        return DoughValue.super.add(other);
    }

    @Override
    public String toString() {
        return "\"" + value + "\"";
    }
}
