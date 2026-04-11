# dough

A small statically typed language that compiles to bytecode and runs on a custom stack VM.
Built as a learning project in Rust.

## Example

```
func main() {
    let x: int = add(1, 2);
    let s: str = "hello dough!";
    
    while x < 10 {
        x = x + 1;
    }
    
    let b: bool = true;
    let f: float = 3.14;
    
    if b {
        b = false;
    } else {
        f = 1.718;
    }
    
    return;
}

func add(lhs: int, rhs: int): int {
    return lhs + rhs;
}
```

## Types
`int`, `float`, `bool`, `str`

## Features
- Static typing
- Functions, variables, while loops, if/else
- Forward declarations
- Mark-and-sweep GC
- Bytecode compilation

## Architecture
The compiler pipeline, across several crates:
- source
- lexer
- parser
- semantic analysis
- bytecode
- vm

## Usage
`dough <path>`
