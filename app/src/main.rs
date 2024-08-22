use chapter01::{add, debug, info, init_config};
use chapter02::print_feature;

fn main() {

    let result = add(1,2);
    println!("Hello, world! {}",result);
    print_feature();
    init_config();
    info!("tracing log");


}
