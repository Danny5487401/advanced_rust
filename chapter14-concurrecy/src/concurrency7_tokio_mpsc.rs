use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // mpsc 多生产者，单消费者模式
    let (sx, mut rx) = mpsc::channel(32);
    let sx2 = sx.clone();

    tokio::spawn(async move {
        sx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        sx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
