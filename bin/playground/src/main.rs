fn main() {
    const SRC: &'static str = "\
func is_even(n: int): bool {
    if n == 0 {
        return true;
    }
    return is_odd(n - 1);
}

func is_odd(n: int): bool {
    if n == 0 {
        return false;
    }
    return is_even(n - 1);
}

func count_down(n: int): int {
    let result: int = 0;
    while n > 0 {
        result = result + 1;
        n = n - 1;
    }
    return result;
}

func max(a: int, b: int): int {
    if a > b {
        return a;
    } else {
        return b;
    }
}

func main() {
    let a: int = count_down(5);
    let b: int = max(a, 3);
    let c: bool = is_even(b);
    return;
}
    ";

    let program = compile(SRC);
    let disasm = bytecode::disasm_program(&program).unwrap();
    println!("{}", disasm);
}

fn compile(input: &str) -> bytecode::Program {
    let tokens = lexer::lex(input).unwrap();
    let untyped = parser::parse(&tokens).unwrap();
    let typed = semantic::analyze(&untyped).unwrap();
    let bytecode = codegen::compile(&typed);
    bytecode
}
