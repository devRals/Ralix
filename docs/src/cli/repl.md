# `repl` command

The `repl` command starts the Read-Eval-Print-Loop (REPL), an interactive
programming environment for Ralix.

## Usage

```bash
ralix repl
```

### Options

- `--tui`: Use the experimental terminal user interface instead.

## Description

The `repl` command allows you to enter and execute Ralix code interactively.
The result of each expression is printed to the console.

There are two versions of the REPL:

- The terminal user interface (TUI) for a richer interactive experience,
  which can be activated with the `--tui` flag.
- The default legacy REPL is a simpler, line-by-line interpreter.

## Examples

To start the default REPL:

```bash
ralix repl
```

To start the TUI REPL:

```bash
ralix repl --tui
```
