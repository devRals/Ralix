# `ast` command

The `ast` command parses a Ralix source file and prints its Abstract Syntax Tree (AST) in JSON format.

## Usage

```bash
ralix ast [OPTIONS] <SOURCE_FILE>
```

### Arguments

*   `<SOURCE_FILE>`: The path to the source file to parse.

### Options

*   `-o, --output <FILE>`: The file to write the AST to. If not provided, the AST is printed to the console.

## Description

The `ast` command is a useful tool for developers who want to inspect the structure of their code. It takes a source file, parses it, and then serializes the resulting AST into a pretty-printed JSON string.

## Examples

To print the AST of a file named `main.ralix` to the console:

```bash
ralix ast main.ralix
```

To save the AST to a file named `ast.json`:

```bash
ralix ast -o ast.json main.ralix
```