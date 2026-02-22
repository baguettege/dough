package io.github.bagu.dough.printer;

import java.nio.charset.StandardCharsets;

import static io.github.bagu.dough.bytecode.Opcodes.*;
import static io.github.bagu.dough.bytecode.TypeTag.*;

public final class InstructionPrinter {
    private static final int MAX_PADDING = 16;

    private static String toHex(int i) {
        return String.format("%04x", i);
    }

    private static String toHex(byte b) {
        return String.format("%02x", b);
    }

    private static short toShort(byte b1, byte b2) {
        return (short) ((b1 & 0xFF) << 8 | (b2 & 0xFF));
    }

    private static long toLong(byte[] operands) {
        return (long) (operands[1] & 0xFF) << 56 |
                (long) (operands[2] & 0xFF) << 48 |
                (long) (operands[3] & 0xFF) << 40 |
                (long) (operands[4] & 0xFF) << 32 |
                (long) (operands[5] & 0xFF) << 24 |
                (long) (operands[6] & 0xFF) << 16 |
                (long) (operands[7] & 0xFF) << 8  |
                (long) (operands[8] & 0xFF);
    }

    public static String print(Instruction instruction) {
        StringBuilder sb = new StringBuilder();

        byte opcode = instruction.opcode();
        byte[] operands = instruction.operands();

        String hexOffset = toHex(instruction.offset());
        String hexOpcode = toHex(opcode);
        String[] hexOperands = new String[operands.length];

        for (int i = 0; i < operands.length; i++) {
            hexOperands[i] = toHex(instruction.operands()[i]);
        }

        sb.append(hexOffset).append(" ").append(hexOpcode).append(" ");
        for (String operand : hexOperands) {
            sb.append(operand).append(" ");
        }

        int bytesUsed = 1 + hexOperands.length;
        int bytesLeft = MAX_PADDING - bytesUsed;

        if (bytesLeft >= 0) {
            sb.append("   ".repeat(bytesLeft));
        }

        switch (instruction.opcode()) {
            case NOP -> sb.append("NOP");
            case HALT -> sb.append("HALT");
            case ADD -> sb.append("ADD");
            case SUB -> sb.append("SUB");
            case MUL -> sb.append("MUL");
            case DIV -> sb.append("DIV");
            case NEG -> sb.append("NEG");
            case POP -> sb.append("POP");
            case EQ -> sb.append("EQ");
            case LT -> sb.append("LT");
            case NOT -> sb.append("NOT");
            case AND -> sb.append("AND");
            case OR -> sb.append("OR");

            case RET -> sb.append("RET ").append(operands[0]);

            case LD -> sb.append("LD ").append(toShort(operands[0], operands[1]));
            case ST -> sb.append("ST ").append(toShort(operands[0], operands[1]));
            case GLD -> sb.append("GLD ").append(toShort(operands[0], operands[1]));
            case GST -> sb.append("GST ").append(toShort(operands[0], operands[1]));
            case JMP -> sb.append("JMP ").append(toShort(operands[0], operands[1]));
            case JF -> sb.append("JF ").append(toShort(operands[0], operands[1]));
            case CALL -> sb.append("CALL ").append(toShort(operands[0], operands[1])).append(" ").append(operands[2]);

            case PUSH -> {
                sb.append("PUSH ");
                byte tag = operands[0];

                switch (tag) {
                    case STR -> {
                        int len = (operands[1] & 0xFF) << 8 | (operands[2] & 0xFF);
                        byte[] b = new byte[len];

                        System.arraycopy(operands, 3, b, 0, len);
                        String s = new String(b, StandardCharsets.UTF_8);

                        sb.append("\"").append(s).append("\"");
                    }
                    case INT -> sb.append(toLong(operands));
                    case FLOAT -> sb.append(Double.longBitsToDouble(toLong(operands)));
                    case BOOL -> sb.append(operands[1] != 0);
                }
            }
        }

        return sb.toString();
    }
}
