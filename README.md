# Rust Faasm workspace

This is project is for writing [Faasm](https://github.com/lsds/Faasm) Rust functions. 

Both Faasm and `faasm-sys` are submodules of this project. Make sure
you update submodules when cloning (`git submodules update --init`).

## Quick-start

Set up Faasm:

```bash
# Start local cluster
./bin/start-faasm.sh
``` 

Compile this project (make sure you have the right toolchain, see Rust notes below):

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

To compile to wasm, it's recommended that you have a nightly toolchain set up, i.e.:

```bash
rustup update
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Usage

### Requirements

* See [faasm-sys](https://github.com/mfournial/faasm-sys#requirements) requirements

### Faasm local development

If you have a Faasm [local development](https://github.com/lsds/Faasm/blob/master/docs/local_dev.md) 
environment set up (advanced), you can run this project with:

```bash
./bin/run-local.sh
```
