# Installation

There are two ways to install and run the `ralix` CLI:

## Using `cargo install`

You can install the `ralix` CLI directly from the source if you have the repository cloned.

```bash
cargo install --path .
```

After installation, you can run the `ralix` command from anywhere in your terminal:

```bash
ralix --help
```

## Running from source

If you have the source code, you can also run the CLI directly using `cargo run`:

```bash
cargo run -- --help
```

This will build and run the latest version of the code. All the arguments to the `ralix` CLI should be passed after `--`.
