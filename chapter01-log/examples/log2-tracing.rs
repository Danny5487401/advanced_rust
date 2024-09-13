use chapter01::init_config;
use log::info;

fn main() {
    init_config();
    info!("tracing log");
}
