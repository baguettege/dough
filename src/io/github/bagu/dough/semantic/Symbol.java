package io.github.bagu.dough.semantic;

import io.github.bagu.dough.type.DoughType;

import java.util.List;
import java.util.Objects;

public sealed interface Symbol
        permits Symbol.Function, Symbol.Global, Symbol.Local {
    record Local(
            DoughType type,
            int slot
    ) implements Symbol {
        public Local {
            Objects.requireNonNull(type);
            if (slot < 0) throw new IllegalArgumentException(String.valueOf(slot));
        }
    }

    record Global(
            DoughType type,
            int index
    ) implements Symbol {
        public Global {
            Objects.requireNonNull(type);
            if (index < 0) throw new IllegalArgumentException(String.valueOf(index));
        }
    }

    record Function(
            List<DoughType> paramTypes,
            DoughType returnType,
            int index
    ) implements Symbol {
        public Function {
            paramTypes = List.copyOf(paramTypes);
            Objects.requireNonNull(returnType);
            if (index < 0) throw new IllegalArgumentException(String.valueOf(index));
        }
    }
}
