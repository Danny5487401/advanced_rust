use tokio::sync::watch;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // watch 单一生产者、多消费者的通道，只保留最后发送的值
    let (tx, mut rx) = watch::channel("hello");
    let mut rx2 = rx.clone();

    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("received1 = {:?}", *rx.borrow());
        }
    });

    tokio::spawn(async move {
        while rx2.changed().await.is_ok() {
            println!("received2 = {:?}", *rx2.borrow());
        }
    });

    tx.send("first").expect("send error!");
    sleep(Duration::from_millis(3000)).await;
    tx.send("second").expect("send error!");
    tx.send("third").expect("send error!");

    sleep(Duration::from_millis(3000)).await;
}
