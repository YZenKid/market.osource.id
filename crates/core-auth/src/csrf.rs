use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;

pub const CSRF_COOKIE_NAME: &str = "market_csrf";
pub const CSRF_HEADER_NAME: &str = "x-csrf-token";

pub fn generate_csrf_token() -> String {
    let mut bytes = [0_u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn csrf_tokens_match(cookie_token: &str, header_token: &str) -> bool {
    let cookie_bytes = cookie_token.as_bytes();
    let header_bytes = header_token.as_bytes();

    let mut diff = cookie_bytes.len() ^ header_bytes.len();
    let max_len = cookie_bytes.len().max(header_bytes.len());
    for index in 0..max_len {
        let left = cookie_bytes.get(index).copied().unwrap_or(0);
        let right = header_bytes.get(index).copied().unwrap_or(0);
        diff |= usize::from(left ^ right);
    }

    diff == 0 && !cookie_token.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_csrf_token_is_url_safe_and_non_empty() {
        let token = generate_csrf_token();

        assert!(!token.is_empty());
        assert!(!token.contains('='));
        assert!(token
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'));
    }

    #[test]
    fn csrf_tokens_match_when_equal() {
        assert!(csrf_tokens_match("token-123", "token-123"));
    }

    #[test]
    fn csrf_tokens_do_not_match_when_different() {
        assert!(!csrf_tokens_match("token-123", "token-456"));
        assert!(!csrf_tokens_match("token-123", "token-123-extra"));
        assert!(!csrf_tokens_match("", ""));
    }
}
