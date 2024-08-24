# Result

```bash
$ cargo wasix build
   Compiling wasix-threading v0.1.0 (/Users/gy-gyoung/my_project/Rust_Lang/111_rust/11111_oj/wasix-rust-examples_training2024/wasix-threading)
warning: unstable feature specified for `-Ctarget-feature`: `atomics`
  |
  = note: this feature is not stably supported; its behavior can change in the future

warning: `wasix-threading` (bin "wasix-threading") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.49s
info: Post-processing WebAssembly files
  Optimizing with wasm-opt
 Downloading precompiled wasm-opt version_113

$ cargo run

   Compiling wasix-threading v0.1.0 (/Users/gy-gyoung/my_project/Rust_Lang/111_rust/11111_oj/wasix-rust-examples_training2024/wasix-threading)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.00s
     Running `target/debug/wasix-threading`
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!


$ cargo wasix run
warning: unstable feature specified for `-Ctarget-feature`: `atomics`
  |
  = note: this feature is not stably supported; its behavior can change in the future

warning: `wasix-threading` (bin "wasix-threading") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `/Users/gy-gyoung/.cargo/bin/cargo-wasix target/wasm32-wasmer-wasi/debug/wasix-threading.wasm`
info: Post-processing WebAssembly files
     Running `target/wasm32-wasmer-wasi/debug/wasix-threading.wasm`
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!

```
