package io.github.bagu.dough.vm;

import io.github.bagu.dough.bytecode.Chunk;

final class CallFrame {
    final Chunk chunk;
    int ip;
    final int base;

    CallFrame(Chunk chunk, int base) {
        this.chunk = chunk;
        this.ip = 0;
        this.base = base;
    }
}
