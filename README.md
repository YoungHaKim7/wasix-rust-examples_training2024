# WASIX - HTMX Example

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

# A language binding generator for WebAssembly interface types 
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

<hr>

# Changes to Rust's WASI targets

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
