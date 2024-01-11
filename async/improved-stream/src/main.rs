// [dependencies]
// async-io = "*"
// async-stream = "*"
// futures = "*"
// futures-lite = "*"

fn main() {
    slow::run();
    improve1::run();
    improve2::run();
}

use std::time::{Duration, Instant};

use async_io::Timer;
use async_stream::stream;
use futures_lite::future::yield_now;

mod slow {
    use async_io::block_on;
    use futures::StreamExt;

    use super::*;
    pub fn run() {
        let result = block_on(async {
            stream! {
                loop {
                    yield async {
                        let start = Instant::now();
                        yield_now().await;
                        Instant::now().duration_since(start)
                    };
                }
            }
            .buffered(5)
            .take(5)
            .then(|d| async move {
                Timer::after(Duration::from_millis(500)).await;
                d
            })
            .collect::<Vec<_>>()
            .await
        });
        println!(".buffered(5): {result:#?}");
        // [
        //     612.875µs,
        //     501.832917ms,
        //     1.002531209s,
        //     1.503673417s,
        //     2.004864417s,  <---- ???
        // ]
    }
}

mod improve1 {
    use async_io::block_on;
    use futures_lite::StreamExt;

    use super::*;
    pub fn run() {
        let result = block_on(async {
            stream! {
                loop {
                    yield async {
                        let start = Instant::now();
                        yield_now().await;
                        Instant::now().duration_since(start)
                    };
                }
            }
            // .buffered(5)
            .take(5)
            .then(|d| async move {
                Timer::after(Duration::from_millis(500)).await;
                d.await
            })
            .collect::<Vec<_>>()
            .await
        });
        println!("improve1: {result:#?}");
        // [
        //     28.04µs,
        //     25.921µs,
        //     22.931µs,
        //     25.87µs,
        //     22.911µs,
        // ]
    }
}

mod improve2 {
    use futures_lite::{future::block_on, StreamExt};

    use super::*;
    pub fn run() {
        let result = block_on(async {
            stream! {
                loop {
                    yield async {
                        let start = Instant::now();
                        yield_now().await;
                        Instant::now().duration_since(start)
                    };
                }
            }
            // .buffered(5)
            .take(5)
            .then(|d| async move {
                Timer::after(Duration::from_millis(500)).await;
                d.await
            })
            .collect::<Vec<_>>()
            .await
        });
        println!("improve2: {result:#?}");
        // [
        //     1.5µs,
        //     1.36µs,
        //     1.78µs,
        //     1.52µs,
        //     1.12µs,
        // ]
    }
}
