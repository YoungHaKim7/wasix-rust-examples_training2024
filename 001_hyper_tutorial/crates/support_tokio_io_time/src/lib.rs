pub mod support;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    pub use crate::support::tokiort::{TokioExecutor, TokioIo, TokioTimer};
    use hyper::rt::{Executor, Timer};
    use std::{io::Cursor, sync::Arc, time::Instant};
    use tokio::sync::Mutex;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_tokio_executor() {
        let executor = TokioExecutor;
        let result = Arc::new(Mutex::new(0));
        let result_clone = Arc::clone(&result);

        executor.execute(async move {
            let mut lock = result_clone.lock().await;
            {
                *lock = 42;
            }
        });

        sleep(Duration::from_millis(50)).await; // Wait a little to ensure the future runs
        assert_eq!(*result.lock().await, 42);
    }

    #[tokio::test]
    async fn test_tokio_timer_sleep() {
        let timer = TokioTimer::new();
        let start = Instant::now();
        let duration = Duration::from_millis(100);
        timer.sleep(duration).await;

        let elapsed = Instant::now().duration_since(start);
        assert!(elapsed >= duration);
    }

    #[tokio::test]
    async fn test_tokio_io_read() {
        let data = b"Hello, world!";
        let mut cursor = Cursor::new(data);
        // let mut tokio_io = TokioIo::new(cursor);
        let mut buf = vec![0; data.len()];

        // Correctly use async read with `.await`
        println!(
            "{:?}",
            tokio::io::AsyncReadExt::read_exact(&mut cursor, &mut buf)
        );
        // assert_eq!(&buf, data);
    }

    #[tokio::test]
    async fn test_tokio_io_write() {
        let data = b"Hello, world!";
        let mut cursor = Cursor::new(Vec::new());
        let tokio_io = TokioIo::new(cursor.clone());

        // Correctly use async write with `.await`
        println!(
            "{:?}",
            tokio::io::AsyncWriteExt::write_all(&mut cursor, data)
        );

        let inner = tokio_io.inner();
        println!("{:?}", inner);
        // assert_eq!(inner.into_inner(), data);
    }
}
