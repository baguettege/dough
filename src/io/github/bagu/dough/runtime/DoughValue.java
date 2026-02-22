package io.github.bagu.dough.runtime;

import io.github.bagu.dough.type.DoughType;

public sealed interface DoughValue permits DoughBool, DoughFloat, DoughInt, DoughStr {
    DoughType type();

    default DoughValue add(DoughValue other) {
        throw new UnsupportedOperationException("Cannot add " + this.type() +  " and " + other.type());
    }

    default DoughValue sub(DoughValue other) {
        throw new UnsupportedOperationException("Cannot subtract " + this.type() +  " and " + other.type());
    }

    default DoughValue mul(DoughValue other) {
        throw new UnsupportedOperationException("Cannot multiply " + this.type() +  " and " + other.type());
    }

    default DoughValue div(DoughValue other) {
        throw new UnsupportedOperationException("Cannot divide " + this.type() +  " and " + other.type());
    }
}
