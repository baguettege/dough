package io.github.bagu.dough.main;

import io.github.bagu.dough.codegen.Bytecode;
import io.github.bagu.dough.codegen.Compiler;
import io.github.bagu.dough.exception.DiagnosticFormatter;
import io.github.bagu.dough.exception.DoughException;
import io.github.bagu.dough.lexer.Lexer;
import io.github.bagu.dough.parser.Parser;
import io.github.bagu.dough.printer.ProgramPrinter;
import io.github.bagu.dough.semantic.AnalysisResult;
import io.github.bagu.dough.semantic.SemanticAnalyser;
import io.github.bagu.dough.vm.VirtualMachine;

final class Main {
    private Main() {
        throw new UnsupportedOperationException();
    }

    public static void main(String[] args) {
        String src = """
                var x: int = 10;
                var y: float = 3.14;
                var flag: bool = true;
                var msg: str = "hello";
                
                func add(a: int, b: int): int {
                    var result: int = a + b;
                    return result;
                }
                
                func main(): void {
                    var z: int = add(x, 1 + 2);
                    if (z > 5) {
                        var neg: int = -z;
                    } else {
                        var w: int = z * 2;
                    }
                    while (flag) {
                        flag = false;
                    }
                }
                """;

        String src2 = """
var a: int = 1;
var b: int = 1;
var temp: int = 0;
var count: int = 0;
var limit: int = 30;
var running: bool = true;
var big: bool = false;

func add(x: int, y: int): int {
    return x + y;
}

func isOver(n: int, threshold: int): bool {
    return threshold < n;
}

func double(n: int): int {
    return n + n;
}

func main(): void {
    while (running) {
        temp = add(a, b);
        a = b;
        b = temp;
        count = count + 1;

        if (isOver(b, 100)) {
            big = true;
        }

        if (isOver(count, limit)) {
            running = false;
        }

        var d: int = double(b);
        var e: int = add(d, count);

        if (isOver(e, 5000)) {
            e = add(e, 1);
        } else {
            e = add(e, 0);
        }
    }
}
    """;

        String src3 = """
var result: int = 0;
var running: bool = true;
var count: int = 0;

func multiply(a: int, b: int): int {
    return a * b;
}

func isEven(n: int): bool {
    return (n * 2) > n;
}

func main(): void {
    var x: int = multiply(3, 7);
    var y: int = multiply(x, 2);

    if (x > 10) {
        result = x + y;
    } else {
        result = x - y;
    }

    while (running) {
        count = count + 1;
        if (count > 4) {
            running = false;
        }
    }
}
                """;

        String src4 = """
var x: int = 5;
var y: int = 10;
var flag: bool = true;
var msg: str = "hello";

func sum(a: int, b: int): int {
    return a + b;
}

func main(): void {
    var z: int = sum(x, y);
    if (z > 10) {
        var neg: int = -z;
    } else {
        var doubleZ: int = z * 2;
    }
    while (flag) {
        flag = false;
    }
    var s: str = msg;
}
                """;

        String s = """
                func sum(a: int, b: int): int {
                    return a + b;
                }
                
                func main(): void {
                    var i: int = 1;
                    while (i + 2 != 4) {
                        i = i + 1;
                    }
                }
                """;

        try {
            Lexer lexer = new Lexer(s);
            Parser parser = new Parser(lexer.lex());
            AnalysisResult analysis = new SemanticAnalyser(parser.parse()).analyse();
            Bytecode bytecode = new Compiler(analysis).compile();

            System.out.println(ProgramPrinter.print(bytecode) + "\n");

            System.out.println("RUNNING");
            new VirtualMachine(bytecode, true).run();
        } catch (DoughException e) {
            System.err.println(new DiagnosticFormatter(e, s).format());
        }
    }
}
