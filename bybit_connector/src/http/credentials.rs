
#[derive(PartialEq, Eq, Clone)]
pub struct Credentials {
    pub api_key: String,
    pub signature: Signature,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Signature {
    Hmac(HmacSignature),
}

#[derive(PartialEq, Eq, Clone)]
pub struct HmacSignature {
    pub api_secret: String,
}

#[derive(PartialEq, Eq, Clone)]
pub struct RsaSignature {
    pub key: String,
    pub password: Option<String>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Ed25519Signature {
    pub key: String,
}

impl Credentials {
    pub fn from_hmac(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Credentials {
            api_key: api_key.into(),
            signature: Signature::Hmac(HmacSignature {
                api_secret: api_secret.into(),
            }),
        }
    }
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("api_key", &"[redacted]")
            .finish()
    }
}
