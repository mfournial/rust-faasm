# Rust Faasm workspace

This is a project outline for writing serverless Rust functions for 
[Faasm](https://github.com/lsds/Faasm). 

Contains the `faasm-sys` (as a **submodule**) and `faasm-wrapper` crates.

## Quick-start

Run a local Faasm cluster:

```bash
git clone https://github.com/lsds/Faasm faasm
cd faasm
docker-compose up -d
``` 

Open the Faasm CLI:

```bash
./bin/cli.sh
```

Compile the Hello World example in this project:

```bash
cd workspace
cargo build --target wasm32-unknown-unknown
```

Upload and run:

```bash
inv upload rust hello --rust
inv invoke rust hello
```

## Usage

### Requirements

* See [faasm-sys](https://github.com/mfournial/faasm-sys#requirements) requirements

### Compile and run

If you have a Faasm [local development](https://github.com/lsds/Faasm/blob/master/docs/local_dev.md) 
environment set up (advanced), you can run Rust functions as follows:

```bash
cd workspace
cargo build --target wasm32-unknown-unknown
FUNCTION=faasm_wrapper
MAIN_WRAPPER_DIR=/usr/local/code/faasm/wasm/rust/$FUNCTION
mkdir -p MAIN_WRAPPER_DIR
cp target/wasm32-unknown-unknown/debug/faasm-wrapper.wasm $MAIN_WRAPPER_DIR/function.wasm
codegen_func rust $FUNCTION
simple_runner rust $FUNCTION
```
