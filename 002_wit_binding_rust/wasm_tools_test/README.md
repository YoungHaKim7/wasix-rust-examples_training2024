# Result

```bash

$ cargo build --target wasm32-wasip1

$ wasmtime *.wasm
```

- `wasmtime wasm_toos_test.wasm`

```bash

$ wasmtime wasm_tools_test.wasm
(module
  (type (;0;) (func (param i64 f32 i32 i32 i32 i32 i32 i32)))
)
```

- `rustup show`

```bash
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/gy/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
wasm32-wasi
wasm32-wasip1
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
```
