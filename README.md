<p align="center">
  <img width=50px src="https://user-images.githubusercontent.com/67513038/228839577-3c3be948-d204-4245-b2a7-1cc96b18230b.svg" />
  <img width=55px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" />
</p>

<hr>

# link

- [Rust_WASM(preview 01](https://github.com/bytecodealliance/wasi-rs)
  - [Rust WASM preview 01](#experimental-wasi-api-bindings-for-rust)

- [wat-> wasm변환](#wat2wasm)
- [Rust에 WASM설치하기wasm-supportrust](#wasm-supportrust)
  - [rust_wasm기초](#rust_wasm)

- [wit-bindgen라이브러리 사용법_러스트코드 기초](#a-language-binding-generator-for-webassembly-interface-types)

<hr>

- [(Rust_WASM)wasip Tutorial&examples예시 정리(rustup target add wasm32-wasip1)](#wasix-rust-examples_training2024)
  - [wasm-tools](#wasm-tools)

<hr>

- [Rust(wasm32-wasip2)로 넘어가는 버젼별 계획](#changes-to-rusts-wasi-targets)
  - [wasip1부터 wasip2까지 잘 정리된 블로그(wasm32-wasip1 & wasm32-wasip2)](#wasip1부터-wasip2까지-잘-정리된-블로그wasm32-wasip1--wasm32-wasip2)

- WASI preview 0~2 History
  - [wasip0부터 ~ wasip2까지 History](#wasip0부터-wasip2까지-history)

<hr>

- 외국인 Tutorial
  - [(240905)How to Build A Full Stack Rust Dashboard App with Leptos, Actix Web & SurrealDB (Full Tutorial) | WhiteSponge](#240905how-to-build-a-full-stack-rust-dashboard-app-with-leptos-actix-web--surrealdb-full-tutorial--whitesponge)
  - [HTMX Rust | Shourya Sharma](#htmx-rust--shourya-sharma)

<hr>

# 초기에는 여기에 정리했음(Rust_WASM)
- https://github.com/YoungHaKim7/Rust_WASM

<hr>

# Experimental WASI API bindings for Rust[|🔝|](#link)
- wasip1(WASM Preview1)
  - https://github.com/bytecodealliance/wasi-rs

<hr>

# wat2wasm[|🔝|](#link)

- wit -> wasm으로 변환 가능

https://github.com/WebAssembly/wabt

# WASM support(Rust)[|🔝|](#link)

- https://component-model.bytecodealliance.org/introduction.html

- https://wasi.dev/

  - install

  ```bash

  rustup target add wasm32-wasi
  ```

  ```
  cargo build --target wasm32-wasi
  wasmer run target/wasm32-wasi/debug/cve-rs.wasm
  ```
  https://wasmer.io/

  https://github.com/Speykious/cve-rs


  ```bash
  curl https://get.wasmer.io -sSfL | sh
  ```

  - Run in CLI

  ```bash
  wasmer run cowsay "Hello world"
  ```

  ```
  pdating bash profile /Users/g/.config/fish/config.fish
  we've added the following to your /Users/g/.config/fish/config.fish
  If you have a different profile please add the following:
  ```

  - Wasmer
  ```bash
  export WASMER_DIR="/Users/g/.wasmer"
  [ -s "$WASMER_DIR/wasmer.sh" ] && source "$WASMER_DIR/wasmer.sh"
  check: wasmer 4.2.7 installed successfully ✓
  wasmer will be available the next time you open the terminal.
  If you want to have the commands available now please execute:
  ```

# Rust_WASM[|🔝|](#link)

eBook https://docs.wasmtime.dev/

- Examples https://github.com/bytecodealliance/wasmtime/tree/main/examples

- wasmtime install
  - https://github.com/bytecodealliance/wasmtime

- build
```
cargo build --target wasm32-wasi --release
```

- install
```
rustup target add wasm32-wasi
```

- https://stackoverflow.com/questions/74968490/the-wasm32-wasi-target-may-not-be-installed-while-it-is-installed


- C++로 만든거 wasm실행하기
  - eBook1 https://wasmedge.org/docs/start/overview
    - eBook https://wasmedge.org/docs/category/build-wasmedge-from-source/

- C++로 만든거 https://github.com/WebAssembly/wabt

- WASM 최적화 시키기 인터프리터 모드에서 더 빠르게 최적화
  - https://wasmedge.org/docs/start/build-and-run/cli#call-a-webassembly-function-compiled-from-rust

# Call a WebAssembly function compiled from Rust

The add program is written in Rust and contains an exported add() function. You can compile it into WebAssembly and use wasmedge to call the add() function. In this example, you will see how it is done from the CLI. It is often used when you embed WasmEdge into another host application, and need to call a WASM function from the host.

You will need to have the Rust compiler installed, and then use the following command to build the WASM bytecode file from the Rust source code.

```
cargo build --target wasm32-wasi --release
```

You can execute wasmedge in reactor mode to invoke the add() function with two i32 integer input parameters.

```
wasmedge --reactor add.wasm add 2 2
```

- https://wasmedge.org/docs/start/build-and-run/cli#call-a-webassembly-function-compiled-from-rust

<hr>


# WASIX - HTMX Example[|🔝|](#link)

- https://wasix.org/

```bash
cargo install cargo-wasix
```
- https://wasix.org/docs/language-guide/rust/installation

## Overview

This is a simple example of using [htmx](https://htmx.org/) with [axum](https://github.com/tokio-rs/axum) and [askama](https://github.com/djc/askama) to create a simple web application.

This example was inspired by [this example](https://github.com/JoeyMckenzie/axum-htmx-templates/tree/main) by [JoeyMckenzie](https://github.com/JoeyMckenzie).

## Pre-requisites

- [Rust](https://rustup.rs/)
- [`cargo-wasix`](https://github.com/wasix-org/cargo-wasix)
- [`wasmer`](https://wasmer.io/)

## Local Development

### Building tailwind css

```shell
$ cargo make styles
```

> This runs a watch task that will rebuild the css when the `styles/tailwind.css` file is changed.

### Running the Development Server

```shell
$ cargo make run
```

> This will run a daemon that will rebuild the project when the source code is changed.

## Building

To run this example, you need to have [Rust] installed.

```shell
$ cargo wasix build --release
```

## Running

### Using source code

```shell
$ wasmer run target/wasm32-wasmer-wasi/release/wasix-htmx.wasm --net --mapdir /assets:./assets
```

### Using Published Package

```shell
$ wasmer run wasmer/wasix-htmx --net
```

Now you can open your browser and go to http://127.0.0.1:8080 to access the website.

This package is available on [wasmer registry](https://wasmer.io/wasmer/wasix-htmx) as `wasmer/wasix-htmx`.

# wasix-rust-examples_training2024
- https://github.com/wasix-org/wasix-rust-examples
  - http://wasix.org/docs/language-guide/rust/examples
    - hyper examples
      - https://github.com/hyperium/hyper/tree/master/examples
      - https://hyper.rs/guides/1/client/basic/

<hr>

- https://benw.is/posts/compiling-rust-to-wasi

- Plugins with Rust and WASI Preview 2 2024-07-16 01:00:00 UTC
  - https://benw.is/posts/plugins-with-rust-and-wasi

<hr>

- https://github.com/WebAssembly/wasi-sockets

- wasi
  - https://github.com/WebAssembly/WASI

<hr>

# A language binding generator for WebAssembly interface types[|🔝|](#link)
- https://github.com/bytecodealliance/wit-bindgen

- Guest: Rust
  - The Rust compiler supports a native `wasm32-wasip1` target and can be added to any `rustup`-based toolchain with:

```bash
rustup target add wasm32-wasip1
```

- In order to compile a wasi dynamic library, the following must be added to the `Cargo.toml` file:

```toml
[lib]
crate-type = ["cdylib"]
```

- Projects can then depend on `wit-bindgen` by executing:

```bash
cargo add wit-bindgen
```

- WIT files are currently added to a `wit/` folder adjacent to your `Cargo.toml` file. Example code using this then looks like:

```rs
// src/lib.rs

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "host",
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyHost;

impl Guest for MyHost {
    fn run() {
        print("Hello, world!");
    }
}

// export! defines that the `MyHost` struct defined below is going to define
// the exports of the `world`, namely the `run` function.
export!(MyHost);
```

- By using `cargo expand` or `cargo doc` you can also explore the generated code. If there's a bug in wit-bindgen and the generated bindings do not compile or if there's an error in the generated code (which is probably also a bug in `wit-bindgen`), you can use `WIT_BINDGEN_DEBUG=1` as an environment variable to help debug this.

- This project can then be built with:

```
cargo build --target wasm32-wasip1
wasm-tools component new ./target/wasm32-wasip1/debug/my-project.wasm \
    -o my-component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
```


- This creates a `my-component.wasm` file which is suitable to execute in any component runtime. Using `wasm-tools` you can inspect the binary as well, for example inferring the WIT world that is the component:

```
wasm-tools component wit my-component.wasm
# world my-component {
#  import print: func(msg: string)
#  export run: func()
# }
```

- which in this case, as expected, is the same as the input world.

- https://github.com/bytecodealliance/wit-bindgen


# wasm-tools[|🔝|](#link)

- https://github.com/bytecodealliance/wasm-tools

- https://github.com/bytecodealliance/wasm-tools/blob/main/examples/wasm-smith.rs

```rs
use arbitrary::Unstructured;
use wasm_smith::{Config, Module};
use wasmprinter::print_bytes;

fn test_wasm_smith() {
    let seed = "W3B4553MB1Y!!!!!!!!!!!!!!!!!!!!!!!!!!";
    let mut u = Unstructured::new(seed.as_bytes());
    if let Ok(module) = Module::new(Config::default(), &mut u) {
        let wasm_buffer = module.to_bytes();
        if let Ok(wat) = print_bytes(wasm_buffer) {
            println!("{}", wat);
        }
    }
}

fn main() {
    test_wasm_smith();
}
```

<hr>

# Changes to Rust's WASI targets[|🔝|](#link)

Apr. 9, 2024 · Yosh Wuyts

- https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html

<table>
<thead>
<tr>
<th>date</th>
<th>Rust Stable</th>
<th>Rust Beta</th>
<th>Rust Nightly</th>
<th>Notes</th>
</tr>
</thead>
<tbody>
<tr>
<td>2024-02-08</td>
<td>1.76</td>
<td>1.77</td>
<td>1.78</td>
<td><code>wasm32-wasip1</code> available on nightly</td>
</tr>
<tr>
<td>2024-03-21</td>
<td>1.77</td>
<td>1.78</td>
<td>1.79</td>
<td><code>wasm32-wasip1</code> available on beta</td>
</tr>
<tr>
<td>2024-05-02</td>
<td>1.78</td>
<td>1.79</td>
<td>1.80</td>
<td><code>wasm32-wasip1</code> available on stable</td>
</tr>
<tr>
<td>2024-06-13</td>
<td>1.79</td>
<td>1.80</td>
<td>1.81</td>
<td>warn if <code>wasm32-wasi</code> is used on nightly</td>
</tr>
<tr>
<td>2024-07-25</td>
<td>1.80</td>
<td>1.81</td>
<td>1.82</td>
<td>warn if <code>wasm32-wasi</code> is used on beta</td>
</tr>
<tr>
<td>2024-09-05</td>
<td>1.81</td>
<td>1.82</td>
<td>1.83</td>
<td>warn if <code>wasm32-wasi</code> is used on stable</td>
</tr>
<tr>
<td>2024-10-17</td>
<td>1.82</td>
<td>1.83</td>
<td>1.84</td>
<td><code>wasm32-wasi</code> unavailable on nightly</td>
</tr>
<tr>
<td>2024-11-28</td>
<td>1.83</td>
<td>1.84</td>
<td>1.85</td>
<td><code>wasm32-wasi</code> unavailable on beta</td>
</tr>
<tr>
<td>2025-01-09</td>
<td>1.84</td>
<td>1.85</td>
<td>1.86</td>
<td><code>wasm32-wasi</code> unavailable on stable</td>
</tr>
</tbody>
</table>

- Conclusion
  - In this post we've discussed the upcoming updates to Rust's WASI targets. Come Rust 1.78 the wasm32-wasip1 (tier 2) and wasm32-wasip2 (tier 3) targets will be added. In Rust 1.81 we will begin warning if wasm32-wasi is being used. And in Rust 1.84, the existing wasm32-wasi target will be removed. This will free up wasm32-wasi to eventually be used for a WASI 1.0 target. Users will have 8 months to switch to the new target name when they update their Rust toolchains.
  - The wasm32-wasip2 target marks the start of native support for WASI 0.2. In order to target it today from Rust, people are encouraged to use cargo-component tool instead. The plan is to eventually graduate wasm32-wasip2 to a tier-2 target, at which point cargo-component will be upgraded to support it natively instead.
  - With WASI 0.2 finally stable, it's an exciting time for WebAssembly development. We're happy for Rust to begin implementing native support for WASI 0.2, and we're excited about what this will enable people to build.

- 결론
  - 이 게시물에서 우리는 러스트의 WASI 대상에 대한 다가오는 업데이트에 대해 논의했습니다. 러스트 1.78로 와스프1(2단계) 및 와스프2(3단계) 대상이 추가됩니다. 러스트 1.81에서는 wasm32-와스프2(3단계) 대상이 사용되고 있는지 경고하기 시작합니다. 그리고 러스트 1.84에서는 기존의 wasm32-와스프 대상이 제거됩니다. 이를 통해 wasm32-와스프는 최종적으로 WASI 1.0 대상에 사용될 수 있습니다. 사용자는 러스트 툴체인을 업데이트할 때 새로운 대상 이름으로 전환할 수 있는 8개월의 시간을 갖게 됩니다.
  - wasm32-wasip2 타겟은 WASI 0.2에 대한 네이티브 지원의 시작을 나타냅니다. 오늘날 러스트에서 타겟으로 삼기 위해서, 사람들은 대신 카고 컴포넌트 툴을 사용하는 것이 좋습니다. 이 계획은 최종적으로 wasm32-wasip2를 Tier-2 타겟으로 졸업시키는 것이며, 이 시점에서 카고 컴포넌트는 대신 네이티브 지원을 위해 업그레이드 될 것입니다.
  - WASI 0.2가 마침내 안정화됨에 따라, WebAssembly 개발을 위한 흥미진진한 시기가 되었습니다. Rust가 WASI 0.2에 대한 기본 지원을 구현하기 시작한 것을 기쁘게 생각하며, 이를 통해 사람들이 무엇을 구축할 수 있을지 기대됩니다.

<hr>

# wasip1부터 wasip2까지 잘 정리된 블로그(wasm32-wasip1 & wasm32-wasip2)[|🔝|](#link)
- wasip2의 약자(WASI preview 2)
  - https://benw.is/posts/plugins-with-rust-and-wasi

<hr>

# wasip0부터 wasip2까지 history[|🔝|](#link)

- wasi preview0 ~ 2 history
- preview 0과 preview1
  - https://github.com/WebAssembly/WASI/blob/main/legacy/README.md
- preview 2
  - https://github.com/WebAssembly/WASI

- WebAssembly targets: change in default target-features
- Sept. 24, 2024 · Alex Crichton on behalf of The Compiler Team
  - https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features.html
 
<hr>

# HTMX Rust | Shourya Sharma[|🔝|](#link)
- https://youtube.com/playlist?list=PL7-lpx4W4IU3HLB01mb0Y2QOftxNCoz_Z&si=vv8GGzGp6fvMYhVg

<hr>

# (240905)How to Build A Full Stack Rust Dashboard App with Leptos, Actix Web & SurrealDB (Full Tutorial) | WhiteSponge[|🔝|](#link)
- https://youtu.be/DQB-cJPYChg?si=I9OimG7SW5XAz25N

<hr>
