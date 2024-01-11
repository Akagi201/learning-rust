use std::time::{Duration, Instant};

use futures::stream::{self, StreamExt};
use tokio::{runtime::Builder, task::yield_now};

fn gen_taks() -> impl futures::Future<Output = ()> {
    async move {
        let start = Instant::now();
        yield_now().await;
        let end = Instant::now();
        println!("time: {:?}", end - start);
    }
}
async fn run_stream() {
    let iter = stream::iter({
        let mut v = Vec::new();
        for _ in 0..10 {
            v.push(gen_taks());
        }
        v
    });
    iter.buffered(5)
        .take(5)
        .then(|d| async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            d
        })
        .collect::<Vec<_>>()
        .await;
}
fn main() {
    // let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(run_stream());
}
