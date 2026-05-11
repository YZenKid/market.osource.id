use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

pub fn generate_session_token() -> String {
    let mut bytes = [0_u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_session_token(raw_token: &str) -> String {
    hex::encode(Sha256::digest(raw_token.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_hash_is_sha256_hex_of_raw_token() {
        assert_eq!(
            hash_session_token("raw-session-token"),
            "e6c276c51996dfa4b71f39f34f5f1a5a8f116e29eb538fab6403dd689631c622"
        );
    }

    #[test]
    fn generated_session_token_is_not_raw_hash_material() {
        let token = generate_session_token();
        assert!(!token.is_empty());
        assert_ne!(token, hash_session_token(&token));
        assert_eq!(hash_session_token(&token).len(), 64);
    }
}
