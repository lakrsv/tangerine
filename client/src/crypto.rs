use aes_gcm::{
    Aes256Gcm, KeyInit,
};

pub struct Crypto<'a> {
    nonce: &'a str,
    cipher: Aes256Gcm,
}

impl<'a> Crypto<'a> {
    pub fn build(key: &str, nonce: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        //let nonce = Nonce::from_slice(nonce.as_bytes());
        let cipher = Aes256Gcm::new_from_slice(key.as_bytes())?;
        Ok(Crypto { nonce, cipher })
    }

    pub fn cipher(&self) -> &Aes256Gcm {
        &self.cipher
    }

    pub fn nonce(&self) -> &str {
        self.nonce
    }
}
