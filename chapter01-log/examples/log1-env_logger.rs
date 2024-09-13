use log::{debug, error, info, log_enabled, Level};

fn main() {
    // 注意，env_logger 必须尽可能早的初始化
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}

//  RUST_LOG=info cargo run --color=always --example log1-std --manifest-path ./chapter01-log/Cargo.toml
