package io.github.bagu.dough.bytecode;

public final class Chunk {
    private final byte[] code;

    public Chunk(byte[] code) {
        this.code = code.clone();
    }

    public byte[] code() {
        return code; // don't clone for performance
    }

    public int length() {
        return code.length;
    }
}
