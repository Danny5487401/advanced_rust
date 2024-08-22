// Expose the minimal log facade.
#[doc(inline)]
pub use tracing::{debug, error, info, trace, warn, Level};

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_config(){
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
