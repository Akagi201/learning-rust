use tokio::sync::mpsc;

// 创建全局队列的通道
lazy_static::lazy_static! {
    pub static ref QUEUE: mpsc::UnboundedSender<i32> = {
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

pub fn enqueue(item: i32) {
    QUEUE.send(item).ok();
}
