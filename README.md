# Rust Faasm workspace

Contains the `faasm-sys` (as a **submodule**) and `faasm-wrapper` crates.

## Usage

### Requirements

* See [faasm-sys](https://github.com/mfournial/faasm-sys#requirements) requirements

### Compile and run

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
