use crate::http::Signature;
use hmac::{Hmac, Mac};
use sha2::{digest::InvalidLength, Sha256};

pub fn sign(payload: &str, signature: &Signature) -> Result<String, InvalidLength> {
    match signature {
        Signature::Hmac(signature) => sign_hmac(payload, &signature.api_secret),
    }
}

fn sign_hmac(payload: &str, key: &str) -> Result<String, InvalidLength> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.to_string().as_bytes())?;

    mac.update(payload.to_string().as_bytes());
    let result = mac.finalize();
    Ok(format!("{:x}", result.into_bytes()))
}
