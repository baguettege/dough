package io.github.bagu.dough.bytecode;

public final class Opcodes {
    private Opcodes() {}

    public static final byte NOP  = 0x00;
    public static final byte HALT = 0x01;

    public static final byte ADD = 0x02;
    public static final byte SUB = 0x03;
    public static final byte MUL = 0x04;
    public static final byte DIV = 0x05;
    public static final byte NEG = 0x06;

    public static final byte PUSH = 0x07;
    public static final byte POP  = 0x08;

    public static final byte LD = 0x09; // load local
    public static final byte ST = 0x0A; // store local

    public static final byte GLD = 0x0B; // load global
    public static final byte GST = 0x0C; // store global

    public static final byte EQ  = 0x0D;
    public static final byte LT  = 0x0E;
    public static final byte NOT = 0x0F;
    public static final byte AND = 0x10;
    public static final byte OR  = 0x11;

    public static final byte JMP  = 0x12;
    public static final byte JF   = 0x13; // jump if false
    public static final byte CALL = 0x14;
    public static final byte RET  = 0x15;
}
