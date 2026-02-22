package io.github.bagu.dough.runtime;

import io.github.bagu.dough.type.DoughType;
import io.github.bagu.dough.type.PrimitiveType;

public record DoughInt(
        long value
) implements DoughValue {
    @Override
    public DoughType type() {
        return PrimitiveType.INT;
    }

    @Override
    public DoughValue add(DoughValue other) {
        if (other instanceof DoughInt i) {
            return new DoughInt(value + i.value);
        }

        return DoughValue.super.add(other);
    }

    @Override
    public DoughValue sub(DoughValue other) {
        if (other instanceof DoughInt i) {
            return new DoughInt(value - i.value);
        }

        return DoughValue.super.sub(other);
    }

    @Override
    public DoughValue mul(DoughValue other) {
        if (other instanceof DoughInt i) {
            return new DoughInt(value * i.value);
        }

        return DoughValue.super.mul(other);
    }

    @Override
    public DoughValue div(DoughValue other) {
        if (other instanceof DoughInt i) {
            return new DoughInt(value / i.value);
        }

        return DoughValue.super.div(other);
    }

    @Override
    public String toString() {
        return String.valueOf(value);
    }
}
