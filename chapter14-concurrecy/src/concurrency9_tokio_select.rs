use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        let _ = tx1.send("one");
    });

    tokio::spawn(async move {
        let _ = tx2.send("two");
    });

    tokio::select! {// select! 语句在两个channels上等待,并将va1绑定到任务返回的值上. 当其中任一 tx1 或者 tx2 完成时，与之相关的块就会执行.
      val = rx1 => {
        println!("rx1 completed first with {:?}", val);
      }
      val = rx2 => {
        println!("rx2 completed first with {:?}", val);
      }
    }
}
