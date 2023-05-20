use tokio::sync::mpsc;

// 创建全局队列的通道
lazy_static::lazy_static! {
    static ref QUEUE: mpsc::UnboundedSender<i32> = {
        // 创建通道
        let (sender, mut receiver) = mpsc::unbounded_channel();

        // 创建一个异步任务来处理接收到的消息
        tokio::spawn(async move {
            while let Some(item) = receiver.recv().await {
                // 处理接收到的消息
                println!("Received: {}", item);
                // 这里可以执行其他操作
            }
        });

        sender
    };
}

#[tokio::main]
async fn main() {
    // 向全局队列发送消息
    QUEUE.send(42).unwrap();
    QUEUE.send(10).unwrap();
    QUEUE.send(99).unwrap();

    // 停顿一段时间，以确保所有消息都被处理
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}
