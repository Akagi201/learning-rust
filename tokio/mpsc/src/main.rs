mod global_queue;
use global_queue::enqueue;
use global_queue::QUEUE;

#[tokio::main]
async fn main() {
    // 向全局队列发送消息
    enqueue(31);
    QUEUE.send(42).unwrap();
    QUEUE.send(10).unwrap();
    QUEUE.send(99).unwrap();

    // 停顿一段时间，以确保所有消息都被处理
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
