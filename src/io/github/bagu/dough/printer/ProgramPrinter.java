package io.github.bagu.dough.printer;

import io.github.bagu.dough.codegen.Bytecode;

public final class ProgramPrinter {
    public static String print(Bytecode bytecode) {
        StringBuilder sb = new StringBuilder();
        sb.append("TOP LEVEL\n");

        Disassembly disassembly = Disassembler.disassemble(bytecode);

        disassembly.topLevel().forEach(instruction ->
                sb.append(InstructionPrinter.print(instruction)).append("\n"));

        disassembly.functions().forEach((index, instructions) -> {
            sb.append("\nFUNCTION ").append(index).append("\n");
            instructions.forEach(instruction ->
                    sb.append(InstructionPrinter.print(instruction)).append("\n"));
        });

        return sb.toString().trim();
    }
}
