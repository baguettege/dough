package io.github.bagu.dough.printer;

import io.github.bagu.dough.bytecode.Chunk;
import io.github.bagu.dough.codegen.Bytecode;

import static io.github.bagu.dough.bytecode.Opcodes.*;
import static io.github.bagu.dough.bytecode.TypeTag.*;

import java.util.*;

public final class Disassembler {
    public static Disassembly disassemble(Bytecode bytecode) {
        Objects.requireNonNull(bytecode);

        List<Instruction> topLevel = disassemble(bytecode.topLevel());
        Map<Integer, List<Instruction>> functions = new HashMap<>();

        int index = 0;
        for (Chunk func : bytecode.functions()) {
            List<Instruction> disassembled = disassemble(func);
            functions.put(index++, disassembled);
        }

        return new Disassembly(topLevel, functions);
    }

    public static List<Instruction> disassemble(Chunk chunk) {
        byte[] code = chunk.code();
        int ip = 0;

        List<Instruction> instructions = new ArrayList<>();

        while (ip < code.length) {
            int offset = ip;
            byte opcode = code[ip++];

            Instruction instruction = switch (opcode) {
                case NOP, HALT, ADD, SUB,
                     MUL, DIV, NEG, POP,
                     EQ, LT, NOT, AND,
                     OR -> new Instruction(offset, opcode,
                        new byte[0], ip - offset);

                case RET -> new Instruction(offset, opcode,
                        new byte[] {code[ip++]}, ip - offset);

                case LD, ST, GLD, GST,
                     JMP, JF -> new Instruction(offset, opcode,
                        new byte[] {code[ip++], code[ip++]}, ip - offset);

                case CALL -> new Instruction(offset, opcode,
                        new byte[] {code[ip++], code[ip++], code[ip++]}, ip - offset);

                case PUSH -> {
                    byte tag = code[ip++];
                    byte[] operands;

                    switch (tag) {
                        case STR -> {
                            byte hi = code[ip++];
                            byte lo = code[ip++];
                            short len = (short) ((hi & 0xFF) << 8 | (lo & 0xFF));

                            operands = new byte[len + 3];
                            operands[0] = tag;
                            operands[1] = hi;
                            operands[2] = lo;

                            for (int i = 3; i < len + 3; i ++) {
                                operands[i] = code[ip++];
                            }
                        }
                        case INT, FLOAT -> {
                            operands = new byte[9];
                            operands[0] = tag;

                            for (int i = 1; i < 9; i++) {
                                operands[i] = code[ip++];
                            }
                        }
                        case BOOL -> {
                            operands = new byte[] {tag, code[ip++]};
                        }

                        default -> throw new AssertionError(tag);
                    }

                    yield new Instruction(offset, opcode, operands, ip - offset);
                }

                default -> throw new AssertionError(opcode);
            };

            instructions.add(instruction);
        }

        return Collections.unmodifiableList(instructions);
    }
}
