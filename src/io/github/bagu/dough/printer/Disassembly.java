package io.github.bagu.dough.printer;

import java.util.List;
import java.util.Map;

public record Disassembly(
        List<Instruction> topLevel,
        Map<Integer, List<Instruction>> functions
) {
    public Disassembly {
        topLevel = List.copyOf(topLevel);
        functions = Map.copyOf(functions);
    }
}
