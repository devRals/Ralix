# Installation

## Using `cargo install`

> [!IMPORTANT]
> For this step ensure you have the [Rust toolchain](https://rustup.rs/) installed.

You can install the `ralix` CLI directly from the source if you have the
repository cloned.

```bash
cargo install --path .
```

Or if you don't even care about the repository (like I usually do)
you can quickly install `ralix` using:

```bash
cargo install ralix
```

After installation, you can run the `ralix` command from anywhere in your terminal:

```bash
ralix --version
```

## Running from source

If you have the source code, you can also run the CLI directly using `cargo run`:

```bash
cargo run -- --help
```

This will build and run the latest version of the code. All the arguments to
the `ralix` CLI should be passed after `--`.
