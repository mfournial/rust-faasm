# Rust Faasm workspace

This is project is for writing [Faasm](https://github.com/lsds/Faasm) Rust functions. 

Both Faasm and `faasm-sys` are submodules of this project. Make sure
you update submodules when cloning (`git submodules update --init`).

This repo contains the `faasm-wrapper` crate.

## Quick-start

Set up Faasm:

```bash
# Download Faasm libs
./bin/faasm-libs.sh

# Start local cluster
./bin/start-faasm.sh
``` 

Compile this project (if this fails, see the Rust notes below):

```bash
cd workspace
cargo build --target wasm32-unknown-unknown
```

Upload and run:

```bash
./bin/upload.sh
./bin/run.sh
```

## Rust Wasm

Until the wasm target is released, you need to have a nightly toolchain set up, i.e.:

```bash
rustup update
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
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
