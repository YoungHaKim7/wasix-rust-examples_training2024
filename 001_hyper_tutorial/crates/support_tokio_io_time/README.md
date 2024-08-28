# `cargo t -p support_tokio_io_time -- --nocapture`


```bash
$ cargo t -p support_tokio_io_time -- --nocapture
   Compiling support_tokio_io_time v0.1.0 (/home/y/my_project/rust_lang/111111ru/wasix-rust-examples_training2024/001_hyper_tutorial/crates/support_tokio_io_time)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.23s
     Running unittests src/lib.rs (/home/y/my_project/rust_lang/111111ru/wasix-rust-examples_training2024/001_hyper_tutorial/target/debug/deps/support_tokio_io_time-f3ebd8a6729f1e95)

running 4 tests
ReadExact { reader: Cursor { inner: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], pos: 0 }, buf: ReadBuf { filled: 0, initialized: 13, capacity: 13 }, _pin: PhantomPinned }
WriteAll { writer: Cursor { inner: [], pos: 0 }, buf: [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], _pin: PhantomPinned }
Cursor { inner: [], pos: 0 }
test tests::test_tokio_io_read ... ok
test tests::test_tokio_io_write ... ok
test tests::test_tokio_executor ... ok
test tests::test_tokio_timer_sleep ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

   Doc-tests support_tokio_io_time

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```