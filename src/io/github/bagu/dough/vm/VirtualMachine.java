package io.github.bagu.dough.vm;

import io.github.bagu.dough.bytecode.Chunk;
import io.github.bagu.dough.bytecode.Opcodes;
import io.github.bagu.dough.bytecode.TypeTag;
import io.github.bagu.dough.codegen.Bytecode;
import io.github.bagu.dough.printer.Disassembler;
import io.github.bagu.dough.printer.Instruction;
import io.github.bagu.dough.printer.InstructionPrinter;
import io.github.bagu.dough.runtime.*;

import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;

public final class VirtualMachine {
    private final boolean trace;

    private final Bytecode bytecode;

    private final CallFrame[] callFrames = new CallFrame[256];
    private final DoughValue[] stack = new DoughValue[1024];
    private final DoughValue[] globals = new DoughValue[256];

    private CallFrame frame;

    private int sp = 0; // stack pointer

    public VirtualMachine(Bytecode bytecode, boolean trace) {
        this.bytecode = Objects.requireNonNull(bytecode);
        this.trace = trace;
    }

    public VirtualMachine(Bytecode bytecode) {
        this(bytecode, false);
    }

    private byte readByte() {
        return frame.chunk.code()[frame.ip++];
    }

    private short readShort() {
        byte[] code = frame.chunk.code();

        byte b1 = code[frame.ip++];
        byte b2 = code[frame.ip++];

        return (short) ((b1 & 0xFF) << 8 | (b2& 0xFF));
    }

    private long readLong() {
        byte[] code = frame.chunk.code();

        byte b1 = code[frame.ip++];
        byte b2 = code[frame.ip++];
        byte b3 = code[frame.ip++];
        byte b4 = code[frame.ip++];
        byte b5 = code[frame.ip++];
        byte b6 = code[frame.ip++];
        byte b7 = code[frame.ip++];
        byte b8 = code[frame.ip++];

        return (long) (b1 & 0xFF) << 56 |
                (long) (b2 & 0xFF) << 48 |
                (long) (b3 & 0xFF) << 40 |
                (long) (b4 & 0xFF) << 32 |
                (long) (b5 & 0xFF) << 24 |
                (long) (b6 & 0xFF) << 16 |
                (long) (b7 & 0xFF) << 8 |
                (long) (b8 & 0xFF);
    }

    private DoughValue pop() {
        if (sp <= 0)
            throw new DoughRuntimeException("Stack underflow");

        return stack[--sp];
    }

    private void push(DoughValue value) {
        if (sp >= stack.length)
            throw new DoughRuntimeException("Stack overflow");

        stack[sp++] = value;
    }

    public void run() {
        int fp = 0; // frame pointer
        frame = new CallFrame(bytecode.topLevel(), 0);
        callFrames[fp++] = frame;

        long startTime = System.currentTimeMillis();

        while (true) {
            int offsetSnapshot = frame.ip;
            Chunk chunkSnapshot = frame.chunk;

            byte[] code = frame.chunk.code();
            byte op = code[frame.ip++];

            switch (op) {
                case Opcodes.NOP -> {}
                case Opcodes.HALT -> {
                    if (trace) {
                        long elapsed = System.currentTimeMillis() - startTime;
                        System.out.println("Execution finished in " + elapsed + "ms");
                    }
                    return;
                }
                case Opcodes.ADD -> {
                    DoughValue b = pop();
                    DoughValue a = pop();
                    push(a.add(b));
                }
                case Opcodes.SUB -> {
                    DoughValue b = pop();
                    DoughValue a = pop();
                    push(a.sub(b));
                }
                case Opcodes.MUL -> {
                    DoughValue b = pop();
                    DoughValue a = pop();
                    push(a.mul(b));
                }
                case Opcodes.DIV -> {
                    DoughValue b = pop();
                    DoughValue a = pop();
                    push(a.div(b));
                }
                case Opcodes.NEG -> {
                    switch (pop()) {
                        case DoughStr ignored -> throw new AssertionError();
                        case DoughInt val -> {
                            long l = -(val.value());
                            push(new DoughInt(l));
                        }
                        case DoughFloat val -> {
                            double d = -(val.value());
                            push(new DoughFloat(d));
                        }
                        case DoughBool ignored -> throw new AssertionError();
                    }
                }
                case Opcodes.PUSH -> {
                    byte tag = code[frame.ip++];

                    switch (tag) {
                        case TypeTag.STR -> {
                            int len = readShort() & 0xFFFF;
                            byte[] bytes = new byte[len];

                            for (int i = 0; i < len; i++) {
                                bytes[i] = code[frame.ip++];
                            }

                            String s = new String(bytes, StandardCharsets.UTF_8);
                            push(new DoughStr(s));
                        }
                        case TypeTag.INT -> {
                            long l = readLong();
                            push(new DoughInt(l));
                        }
                        case TypeTag.FLOAT -> {
                            long l = readLong();
                            double d = Double.longBitsToDouble(l);
                            push(new DoughFloat(d));
                        }
                        case TypeTag.BOOL -> {
                            byte b = code[frame.ip++];
                            push(new DoughBool(b != 0));
                        }
                    }
                }
                case Opcodes.POP -> {
                    pop();
                }
                case Opcodes.LD -> {
                    short slot = readShort();
                    push(stack[frame.base + slot]);
                }
                case Opcodes.ST -> {
                    short index = readShort();
                    stack[frame.base + index] = pop();
                    if (frame.base + index >= sp) sp = frame.base + index + 1; // protect local slot from future pushes
                }
                case Opcodes.GLD -> {
                    short index = readShort();
                    push(globals[index]);
                }
                case Opcodes.GST -> {
                    short index = readShort();
                    globals[index] = pop();
                }
                case Opcodes.EQ -> {
                    DoughValue b = pop();

                    switch (pop()) {
                        case DoughStr a -> {
                            boolean eq = a.value().equals(((DoughStr) b).value());
                            push(new DoughBool(eq));
                        }
                        case DoughInt a -> {
                            boolean eq = a.value() == ((DoughInt) b).value();
                            push(new DoughBool(eq));
                        }
                        case DoughFloat a -> {
                            boolean eq = a.value() == ((DoughFloat) b).value();
                            push(new DoughBool(eq));
                        }
                        case DoughBool a -> {
                            boolean eq = a.value() == ((DoughBool) b).value();
                            push(new DoughBool(eq));
                        }
                    }
                }
                case Opcodes.LT -> {
                    DoughValue b = pop();

                    switch (pop()) {
                        case DoughStr ignored -> throw new AssertionError();
                        case DoughInt a -> {
                            boolean eq = a.value() < ((DoughInt) b).value();
                            push(new DoughBool(eq));
                        }
                        case DoughFloat a -> {
                            boolean eq = a.value() < ((DoughFloat) b).value();
                            push(new DoughBool(eq));
                        }
                        case DoughBool ignored -> throw new AssertionError();
                    }
                }
                case Opcodes.NOT -> {
                    DoughBool val = (DoughBool) pop();
                    push(new DoughBool(!val.value()));
                }
                case Opcodes.AND -> {
                    DoughBool b = (DoughBool) pop();
                    DoughBool a = (DoughBool) pop();
                    push(new DoughBool(a.value() && b.value()));
                }
                case Opcodes.OR -> {
                    DoughBool b = (DoughBool) pop();
                    DoughBool a = (DoughBool) pop();
                    push(new DoughBool(a.value() || b.value()));
                }
                case Opcodes.JMP -> {
                    short offset = readShort();
                    frame.ip += offset;
                }
                case Opcodes.JF -> {
                    DoughBool val = (DoughBool) pop();
                    short offset = readShort();

                    if (!val.value()) {
                        frame.ip += offset;
                    }
                }
                case Opcodes.CALL -> {
                    short index = readShort();
                    byte args = readByte();
                    int base = sp - args;

                    if (fp >= callFrames.length)
                        throw new DoughRuntimeException("Call stack overflow");

                    callFrames[fp++] = frame;
                    frame = new CallFrame(bytecode.functions()[index], base);
                }
                case Opcodes.RET -> {
                    boolean hasVal = readByte() != 0;
                    DoughValue returnVal = hasVal ? pop() : null;

                    sp = frame.base;
                    frame = callFrames[--fp];
                    if (hasVal) push(returnVal);
                }
            }

            if (trace) trace(chunkSnapshot, offsetSnapshot);
        }
    }

    private void trace(Chunk chunk, int offset) {
        List<Instruction> instructions = Disassembler.disassemble(chunk);
        Instruction current = null;

        for (Instruction i : instructions) {
            if (i.offset() == offset) current = i;
        }

        if (current == null) throw new AssertionError(offset);

        String out = InstructionPrinter.print(current)
                + "  |  sp=" + sp
                + "  |  stack=" + Arrays.toString(Arrays.copyOfRange(stack, 0, sp));
        System.out.println(out);
    }
}

/*
operand stack
global memory
call frame []
 */