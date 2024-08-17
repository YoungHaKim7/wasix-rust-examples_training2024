# wasix-rust-examples_training2024
- https://github.com/wasix-org/wasix-rust-examples
  - http://wasix.org/docs/language-guide/rust/examples

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
