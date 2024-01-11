use std::time::{Duration, Instant};

use async_io::{block_on, Timer};
use async_stream::stream;
use futures::StreamExt;
use futures_lite::future::yield_now;
fn main() {
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
    dbg!(result);
    // [examples/buffered_stream.rs:26:5] result = [
    //     612.875µs,
    //     501.832917ms,
    //     1.002531209s,
    //     1.503673417s,
    //     2.004864417s,  <---- ???
    // ]
}
// 避免在 stream 里链入 FuturesOrdered / FuturesUnordered 以及相关的功能（比如 StreamExt::buffered），否则当 stream 存在背压时会有上述例子里反直觉的阻塞表现
// 这个就是 buffered 生成的 5 个 async task 跑到第一个 await pending 了 直到 then 才被 poll 了一次吧 所以每个才线性递增了 500ms
// 对的，Stream trait 缺乏必需的控制流程导致 FuturesUnordered::poll_next 被调用前无法及时处理已经被 wake 的 inner future，相关 issue 在这里：
// https://github.com/rust-lang/futures-rs/issues/2387
// 这导致 stream 的背压会被随机附加在 inner future 的任何一次 await 上，最终表现是每个 inner future 的异步调用耗时会飘忽不定难以排查
// 嗯 所以我自己写这种处理 碰到耗时的都是 spawn 一堆 然后 recv channel 的 全靠 tokio 自己玩
// 哦，我看了下这个 issue，它说的是 FuturesUnordered 每次只有 执行 poll_next 才会执行内部的 future，issue 里面是用 stream 来类比它的运作方式
// 哦，我看了下这个 issue，它说的是 FuturesUnordered 每次只有 执行 poll_next 才会执行内部的 future，issue 里面是用 stream 来类比它的运作方式
// 单要解决这个问题的话，把 yield_now 去掉就好，它会把 future 强制挂起
// https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html#the-story
// https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async/topic/blog.20post.3A.20.60for.20await.60.20and.20the.20battle.20of.20buffered.20streams
