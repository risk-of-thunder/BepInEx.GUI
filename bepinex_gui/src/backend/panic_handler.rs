use std::panic;

pub fn init() {
    panic::set_hook(Box::new(|panic_info| {
        panic_info.payload().downcast_ref::<&str>().map_or_else(
            || {
                tracing::error!("panic occurred");
            },
            |s| {
                tracing::error!("panic occurred: {s:?}");
            },
        )
    }));
}
