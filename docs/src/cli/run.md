# `run` command

The `run` command is used to execute a Ralix script.

## Usage

```bash
ralix run [FILE]
```

### Arguments

*   `[FILE]`: The path to the script file to execute.

## Description

The `run` command executes a Ralix script from a file. If no file is provided, it will attempt to run a project, but this feature is not yet implemented.

## Examples

To run a script file named `main.ralix`:

```bash
ralix run main.ralix
```