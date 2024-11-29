

pub const TEST_REST_API: &str = "https://api-testnet.bybit.com";
pub const TEST_WSS_API: &str = "wss://stream-testnet.bybit.com/v5/public/linear";

pub const PROD_REST_API: &str = "https://api.bybit.com";
pub const PROD_WSS_API: &str = "wss://stream.bybit.com/v5/public/linear";

use std::sync::atomic::{AtomicUsize, Ordering};

static ENV: AtomicUsize = AtomicUsize::new(0);

pub fn enable_prod(enabled: bool) {
    if enabled {
        ENV.store(1, Ordering::SeqCst);
    } else {
        ENV.store(0, Ordering::SeqCst);
    }
}

pub fn wss_api() -> String {
    let env = ENV.load(Ordering::SeqCst);
    if env == 1 {
        PROD_WSS_API.to_string()
    } else {
        TEST_WSS_API.to_string()
    }
}

pub fn rest_api() -> String {
    let env = ENV.load(Ordering::SeqCst);
    if env == 1 {
        PROD_REST_API.to_string()
    } else {
        TEST_REST_API.to_string()
    }
}




