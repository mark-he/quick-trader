

pub const TEST_REST_API: &str = "https://api-testnet.bybit.com";
pub const TEST_WSS_API: &str = "wss://stream-testnet.bybit.com";

pub const PROD_REST_API: &str = "https://api.bybit.com";
pub const PROD_WSS_API: &str = "wss://stream.bybit.com";

use std::sync::atomic::{AtomicUsize, Ordering};

static ENV: AtomicUsize = AtomicUsize::new(0);

pub static mut PROXY: Option<String> = None;
pub fn get_proxy() -> String {
    unsafe {
        if PROXY.is_some() {
            return PROXY.as_ref().unwrap().clone();
        }
    }
    return "".to_string();
}

pub fn set_proxy(proxy: &str) {
    unsafe {
        PROXY = Some(proxy.to_string());
    }
}

pub fn is_proxy() -> bool {
    unsafe {
        return PROXY.is_some();
    }
}

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




