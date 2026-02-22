package io.github.bagu.dough.printer;

import java.util.Objects;

public record Instruction(
        int offset,
        byte opcode,
        byte[] operands,
        int length
) {
    public Instruction {
        if (offset < 0) throw new IllegalArgumentException(String.valueOf(offset));
        Objects.requireNonNull(operands);
        if (length < 0) throw new IllegalArgumentException(String.valueOf(length));
    }
}
