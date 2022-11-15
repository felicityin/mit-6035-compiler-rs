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
    - Scanner: splits source file input to tokens
        - Tokens can be operator, keyword, literal, string, or identifier.
        - Non-tokens such as white spaces are discarded in this phase.
        - Malformed tokens are reported and aborts compilation process.
    - Parser: reads tokens and check if it conforms to the language spec.
        - matching braces
	- semicolons
	- Not verified: type, function/variable name
	- Outputs a kind of tree structure (not AST)

2. Semantic Checker (Front End)
    - checks various non-context-free constraints: e.g. type compatibility
    - builds symbol table that keeps user-defined types and location of each identifier
    - Outputs IR

3. Code Generation (Back-end)
    - generate _unoptimized_ x86-64 assembly
    - Object code conforming to ABI (Application Binary Interface)

4. Data Flow Analysis (Back-end)
    - optimization pass

# Reference
Modified based on https://github.com/tlsdmstn56/mit-6-035-compiler-rs
