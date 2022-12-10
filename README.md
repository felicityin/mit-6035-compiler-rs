# MIT 6.035 Compiler in Rust

Rust implementation of compiler project in [mit 6.035 computer language engineering (Spring 2010)](https://ocw.mit.edu/courses/6-035-computer-language-engineering-spring-2010/)

# Build

```bash
cargo build
```

# Test

```bash
cargo test
```

# Run

```bash
cargo run
```

# Components

1. Scanner and Parser (Front End)
    - use [lalrpop](https://github.com/lalrpop/lalrpop)
    - scanner: splits source file input to tokens
        - tokens can be operator, keyword, literal, string, or identifier
        - non-tokens such as white spaces are discarded in this phase
        - malformed tokens are reported and aborts compilation process
    - parser: reads tokens and check if it conforms to the language spec
        - matching braces
	- semicolons
	- not verified: type, function/variable name
	- outputs AST

2. Semantic Checker (Front End)
    - checks various non-context-free constraints: e.g. type compatibility
    - builds symbol table that keeps user-defined types and location of each identifier
    - outputs IR

3. Code Generation (Back-end) [todo]
    - generate _unoptimized_ x86-64 assembly
    - object code conforming to ABI (Application Binary Interface)

4. Data Flow Analysis (Back-end) [todo]
    - optimization pass

5. Optimizer (Back-end) [todo]
    - multiple data flow optimization pass
