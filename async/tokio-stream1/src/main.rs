use async_stream::stream;
use futures::stream::StreamExt;
use std::time::Duration;
use std::time::Instant;
use tokio::task::yield_now;
use tokio::runtime::Builder;

async fn run_stream() {
    let result = stream! {
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
        tokio::time::sleep(Duration::from_millis(500)).await;
        d
    })
    .collect::<Vec<_>>()
    .await;
    dbg!(result);
}
fn main() {
    // let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(run_stream());
}
#[cfg(test)]
mod test {
    use tracing::Instrument;
    use super::*;
    #[test]
    fn tokio_current() {
        let _ = tracing_subscriber::FmtSubscriber::builder()
            .with_thread_ids(true)
            .try_init();
        let span = tracing::info_span!("single");
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(run_stream().instrument(span));
    }
    #[test]
    fn tokio_multi() {
        let _ = tracing_subscriber::FmtSubscriber::builder()
            .with_thread_ids(true)
            .try_init();
        let span = tracing::info_span!("multi");
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(run_stream().instrument(span));
    }
}
// 这个是因为 yeild_now 是让出执行片到当前线程的吧
// use tokio::task::yield_now 和 futures_lite::future::yield_now 这俩不一样。。。换成后者两种 runtime 输出就一致了