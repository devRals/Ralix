# `repl` command

The `repl` command starts the Read-Eval-Print-Loop (REPL), an interactive programming environment for Ralix.

## Usage

```bash
ralix repl
```

### Options

*   `--legacy`: Use the simpler, line-by-line REPL without the advanced terminal UI.

## Description

The `repl` command allows you to enter and execute Ralix code interactively. The result of each expression is printed to the console.

There are two versions of the REPL:
*   The default REPL uses a terminal user interface (TUI) for a richer interactive experience.
*   The legacy REPL is a simpler, line-by-line interpreter, which can be activated with the `--legacy` flag.

## Examples

To start the default REPL:

```bash
ralix repl
```

To start the legacy REPL:

```bash
ralix repl --legacy
```