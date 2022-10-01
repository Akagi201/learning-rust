use std::rc::Rc;
use tokio::sync::mpsc;
use tokio::task;
use tokio::task::yield_now;

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    if false {
        // say_world().await;
        // Calling `say_world()` does not execute the body of `say_world()`.
        let op = say_world();

        // This println! comes first
        println!("hello");

        // Calling `.await` on `op` starts executing `say_world`.
        op.await;
    }

    if false {
        let handle = tokio::spawn(async { "return value" });

        // do something

        let out = handle.await.unwrap();
        println!("Got {}", out);
    }

    if false {
        let v = vec![1, 2, 3];

        task::spawn(async move {
            println!("Here's a vec: {:?}", v);
        })
        .await
        .unwrap();
    }

    if false {
        tokio::spawn(async {
            // The scope forces `rc` to drop before `.await`.
            {
                let rc = Rc::new("hello");
                println!("{}", rc);
            }

            // `rc` is no longer used. It is **not** persisted when
            // the task yields to the scheduler
            yield_now().await;
        })
        .await
        .unwrap();
    }

    // {
    //     // wrong
    //     tokio::spawn(async {
    //         let rc = Rc::new("hello");

    //         // `rc` is used after `.await`. It must be persisted to
    //         // the task's state.
    //         yield_now().await;

    //         // println!("{}", rc);
    //     }).await.unwrap();
    // }

    {
        let (tx, mut rx) = mpsc::channel(32);
        let tx2 = tx.clone();

        tokio::spawn(async move {
            tx.send("sending from first handle").await.unwrap();
        });

        tokio::spawn(async move {
            tx2.send("sending from second handle").await.unwrap();
        });

        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
        }
    }
}
