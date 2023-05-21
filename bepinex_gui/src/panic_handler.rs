use std::panic;

pub fn init() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            tracing::error!("panic occurred: {s:?}");
        } else {
            tracing::error!("panic occurred");
        }
    }));
}
