# Ralix

**Ralix** is a lightweight, mid-level interpreter written in Rust. It combines
the familiarity of C-like syntax with the flexibility of optional semicolons
and high-level type-system features.

## Features

- **First-Class Types**: Types are treated as values, allowing for flexible
  type-level logic.
- **Flexible Syntax**: C-inspired curly-brace style, but with **optional
  semicolons** for a cleaner look.
- **Pointer Power**: Full support for pointer arithmetic and dereferencing
  operations (Raw/Unsafe).
- **Hybrid Expressions**: Supports both standard **Infix** (`1 + 2`)
  and **Prefix** (`-3`) math evaluations.
- **Built-in REPL**: An interactive Read-Eval-Print Loop for testing
  snippets on the fly.

## Installation

Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

```bash
git clone https://github.com/devRals/ralix
cd ralix
cargo install --path .

```

Then you should be able have an access to the **Ralix Cli** in your terminal

## Usage

### Interactive Mode (REPL)

To start the interactive shell:

```bash
ralix repl

```

## Language Cheat-Sheet

### Math & Expressions

Ralix is flexible with how it parses math:

```c
// Let binding definition
let x = 10 + 5;

// C like definition
int *y = &x;  // 50

```

### Pointers

Direct memory manipulation is possible:

```c
int a = 42;
int* ptr = &a;    // Reference
let value = *ptr; // Dereference

```

### Types as Values

```py
type[int] myType = int;
myType == typeof 1 + 1; // true
```

---

For more examples you can check you the [official Ralix website](https://devRals.github.io/Ralix)

## 🗺 Roadmap

- [x] **Scope Expressions**: Blocks that return values.
- [x] **Function Definitions**: Standard function declarations.
- [x] **Memory Safety**: Optional safe-wrappers for pointer operations.
- [ ] File Modules: Accessing every ralix file modules from one.
- [ ] **Standard Library**: Basic I/O and string manipulation.

## 🤝 Contributing

This is a personal project by **devRals**. Feel free to open an issue or submit
a pull request if you have ideas for the language specification!

## License

MIT
