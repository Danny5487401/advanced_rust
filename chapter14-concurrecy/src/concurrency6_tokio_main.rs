#[tokio::main]
async fn main() {
    println!("hello");
}
/*
fn main() {
    tokio::runtime::Builder::new_multi_thread().enable_all()
        .build().unwrap()
        .block_on(async {
            // async main
            println!("hello");
        })
}

*/
