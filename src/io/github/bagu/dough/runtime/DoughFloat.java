package io.github.bagu.dough.runtime;

import io.github.bagu.dough.type.DoughType;
import io.github.bagu.dough.type.PrimitiveType;

public record DoughFloat(
        double value
) implements DoughValue {
    @Override
    public DoughType type() {
        return PrimitiveType.FLOAT;
    }

    @Override
    public DoughValue add(DoughValue other) {
        if (other instanceof DoughFloat flt) {
            return new DoughFloat(value + flt.value);
        }

        return DoughValue.super.add(other);
    }

    @Override
    public DoughValue sub(DoughValue other) {
        if (other instanceof DoughFloat flt) {
            return new DoughFloat(value - flt.value);
        }

        return DoughValue.super.sub(other);
    }

    @Override
    public DoughValue mul(DoughValue other) {
        if (other instanceof DoughFloat flt) {
            return new DoughFloat(value * flt.value);
        }

        return DoughValue.super.mul(other);
    }

    @Override
    public DoughValue div(DoughValue other) {
        if (other instanceof DoughFloat flt) {
            return new DoughFloat(value / flt.value);
        }

        return DoughValue.super.div(other);
    }

    @Override
    public String toString() {
        return String.valueOf(value);
    }
}
