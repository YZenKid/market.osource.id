use axum::{
    extract::{ConnectInfo, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use core_app::AppState;
use core_auth::{CSRF_COOKIE_NAME, CSRF_HEADER_NAME};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::{IpAddr, SocketAddr};

use crate::authz::{read_cookie, resolve_authenticated_actor};

pub const SESSION_COOKIE_NAME: &str = "market_session";
const SESSION_MAX_AGE_SECONDS: i64 = 60 * 60 * 24 * 7;

pub(crate) struct CreatedSessionCookie {
    pub header_value: HeaderValue,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/csrf", get(csrf))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

#[derive(Debug, Deserialize)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: SecretString,
}

#[derive(Debug, Serialize)]
pub struct AuthUserResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub role: AuthRoleResponse,
}

#[derive(Debug, Serialize)]
pub struct AuthRoleResponse {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user: AuthUserResponse,
    pub session_expires_at: chrono::DateTime<Utc>,
    pub gate_d_note: &'static str,
}

#[derive(Debug, Serialize)]
pub struct CsrfResponse {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthErrorResponse {
    pub error: &'static str,
    pub message: String,
    pub gate_d_note: Option<&'static str>,
}

async fn login(
    State(state): State<AppState>,
    ConnectInfo(peer_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<LoginRequestBody>,
) -> Result<(HeaderMap, Json<AuthUserResponse>), (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    // TODO: Replace this single-process limiter with a distributed, proxy-aware limiter once
    // reverse proxy trusted headers and multi-node deployment are wired.
    let normalized_email = body.email.trim().to_ascii_lowercase();
    let rate_key = login_rate_limit_key(peer_addr, &headers, &normalized_email);
    if !state.rate_limiter.check(rate_key).allowed {
        audit_auth_event(
            state.db_pool.as_ref(),
            None,
            "auth.login",
            "rate_limited",
            &[("reason", Value::String("too_many_attempts".to_string()))],
        )
        .await;
        return Err(rate_limited());
    }

    let Some(pool) = &state.db_pool else {
        return Err(auth_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for authentication".to_string(),
            None,
        ));
    };

    let Some(user) = core_db::find_active_user_by_email_with_role(pool, &normalized_email)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, "failed to look up login user");
            auth_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "login_failed",
                "login failed; check backend logs for details".to_string(),
                None,
            )
        })?
    else {
        audit_auth_event(
            Some(pool),
            None,
            "auth.login",
            "failure",
            &[("reason", Value::String("invalid_credentials".to_string()))],
        )
        .await;
        return Err(invalid_credentials());
    };

    let password_valid =
        core_auth::verify_password(&body.password, &user.password_hash).map_err(|error| {
            tracing::warn!(error = %error, "failed to verify password hash");
            auth_error(
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
                "invalid email or password".to_string(),
                None,
            )
        })?;
    if !password_valid {
        audit_auth_event(
            Some(pool),
            Some(user.id),
            "auth.login",
            "failure",
            &[("reason", Value::String("invalid_credentials".to_string()))],
        )
        .await;
        return Err(invalid_credentials());
    }

    let created_session = create_session_cookie(pool, user.id, state.config.cookie_secure)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, "failed to create session");
            auth_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "session_create_failed",
                "session could not be created".to_string(),
                None,
            )
        })?;

    audit_auth_event(
        Some(pool),
        Some(user.id),
        "auth.login",
        "success",
        &[("session_created", Value::Bool(true))],
    )
    .await;

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, created_session.header_value);

    Ok((headers, Json(auth_user_response(user))))
}

async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<HeaderMap, (StatusCode, Json<AuthErrorResponse>)> {
    require_csrf(&headers)?;
    let Some(pool) = &state.db_pool else {
        return Err(auth_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for authentication".to_string(),
            Some("CSRF validation is enforced for cookie-authenticated Gate D mutations."),
        ));
    };

    let actor = resolve_authenticated_actor(&state, &headers).await.ok();

    if let Some(raw_token) = read_cookie(&headers, SESSION_COOKIE_NAME) {
        let session_hash = core_auth::hash_session_token(&raw_token);
        core_db::revoke_session_by_hash(pool, &session_hash)
            .await
            .map_err(|error| {
                tracing::warn!(error = %error, "failed to revoke session");
                auth_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "logout_failed",
                    "logout failed; check backend logs for details".to_string(),
                    Some("CSRF validation is enforced for cookie-authenticated Gate D mutations."),
                )
            })?;
    }

    audit_auth_event(
        Some(pool),
        actor.as_ref().map(|actor| actor.user_id),
        "auth.logout",
        "success",
        &[("session_revoked", Value::Bool(true))],
    )
    .await;

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&expired_session_cookie(state.config.cookie_secure))
            .expect("expired session cookie value is static safe ASCII"),
    );
    Ok(response_headers)
}

async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<MeResponse>, (StatusCode, Json<AuthErrorResponse>)> {
    let Some(pool) = &state.db_pool else {
        return Err(auth_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for authentication".to_string(),
            None,
        ));
    };
    let raw_token = read_cookie(&headers, SESSION_COOKIE_NAME).ok_or_else(unauthenticated)?;
    let session_hash = core_auth::hash_session_token(&raw_token);

    let session = core_db::get_current_session_by_hash(pool, &session_hash)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, "failed to read current session");
            auth_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "session_lookup_failed",
                "session lookup failed; check backend logs for details".to_string(),
                None,
            )
        })?
        .ok_or_else(unauthenticated)?;

    Ok(Json(MeResponse {
        session_expires_at: session.session_expires_at,
        user: AuthUserResponse {
            id: session.user_id,
            name: session.name,
            email: session.email,
            role: AuthRoleResponse {
                code: session.role_code,
                name: session.role_name,
            },
        },
        gate_d_note: "CSRF token endpoint and validation are active for Gate D cookie mutations.",
    }))
}

async fn csrf(State(state): State<AppState>) -> (HeaderMap, Json<CsrfResponse>) {
    let token = core_auth::generate_csrf_token();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&csrf_cookie(&token, state.config.cookie_secure))
            .expect("CSRF cookie value is generated from safe characters"),
    );

    (headers, Json(CsrfResponse { token }))
}

fn auth_user_response(user: core_db::AuthUserRecord) -> AuthUserResponse {
    AuthUserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        role: AuthRoleResponse {
            code: user.role_code,
            name: user.role_name,
        },
    }
}

pub(crate) async fn create_session_cookie(
    pool: &sqlx::PgPool,
    user_id: uuid::Uuid,
    secure: bool,
) -> Result<CreatedSessionCookie, core_db::AuthRepositoryError> {
    let raw_token = core_auth::generate_session_token();
    let session_hash = core_auth::hash_session_token(&raw_token);
    let expires_at = Utc::now() + Duration::seconds(SESSION_MAX_AGE_SECONDS);
    core_db::create_session(pool, user_id, &session_hash, expires_at).await?;

    let header_value = HeaderValue::from_str(&session_cookie(&raw_token, secure))
        .expect("session cookie value is generated from safe characters");

    Ok(CreatedSessionCookie { header_value })
}

fn invalid_credentials() -> (StatusCode, Json<AuthErrorResponse>) {
    auth_error(
        StatusCode::UNAUTHORIZED,
        "invalid_credentials",
        "invalid email or password".to_string(),
        None,
    )
}

fn unauthenticated() -> (StatusCode, Json<AuthErrorResponse>) {
    auth_error(
        StatusCode::UNAUTHORIZED,
        "unauthenticated",
        "a valid session is required".to_string(),
        None,
    )
}

fn auth_error(
    status: StatusCode,
    error: &'static str,
    message: String,
    gate_d_note: Option<&'static str>,
) -> (StatusCode, Json<AuthErrorResponse>) {
    (
        status,
        Json(AuthErrorResponse {
            error,
            message,
            gate_d_note,
        }),
    )
}

fn session_cookie(raw_token: &str, secure: bool) -> String {
    cookie_value(raw_token, SESSION_MAX_AGE_SECONDS, secure)
}

fn expired_session_cookie(secure: bool) -> String {
    cookie_value("", 0, secure)
}

fn csrf_cookie(token: &str, secure: bool) -> String {
    let secure_suffix = if secure { "; Secure" } else { "" };
    format!("{CSRF_COOKIE_NAME}={token}; SameSite=Lax; Path=/; Max-Age={SESSION_MAX_AGE_SECONDS}{secure_suffix}")
}

fn cookie_value(value: &str, max_age_seconds: i64, secure: bool) -> String {
    let secure_suffix = if secure { "; Secure" } else { "" };
    format!(
        "{SESSION_COOKIE_NAME}={value}; HttpOnly; SameSite=Lax; Path=/; Max-Age={max_age_seconds}{secure_suffix}"
    )
}

pub(crate) fn require_csrf(
    headers: &HeaderMap,
) -> Result<(), (StatusCode, Json<AuthErrorResponse>)> {
    let cookie_token = read_cookie(headers, CSRF_COOKIE_NAME).ok_or_else(csrf_error)?;
    let header_token = headers
        .get(CSRF_HEADER_NAME)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(csrf_error)?;

    if core_auth::csrf_tokens_match(&cookie_token, header_token) {
        Ok(())
    } else {
        Err(csrf_error())
    }
}

fn rate_limited() -> (StatusCode, Json<AuthErrorResponse>) {
    auth_error(
        StatusCode::TOO_MANY_REQUESTS,
        "rate_limited",
        "too many attempts; try again later".to_string(),
        None,
    )
}

fn login_rate_limit_key(
    peer_addr: SocketAddr,
    headers: &HeaderMap,
    normalized_email: &str,
) -> String {
    format!(
        "login:{}:{normalized_email}",
        client_identity(peer_addr, headers)
    )
}

pub(crate) fn client_identity(peer_addr: SocketAddr, headers: &HeaderMap) -> String {
    // Primary identity is the direct peer socket. Forwarding headers are only considered
    // when the immediate peer is local loopback (bounded local proxy assumption).
    if peer_addr.ip().is_loopback() {
        forwarded_identity_from_headers(headers).unwrap_or_else(|| peer_addr.ip().to_string())
    } else {
        peer_addr.ip().to_string()
    }
}

fn forwarded_identity_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .and_then(parse_first_forwarded_ip)
        .or_else(|| headers.get("x-real-ip").and_then(parse_single_ip))
}

fn parse_first_forwarded_ip(value: &HeaderValue) -> Option<String> {
    let first = value.to_str().ok()?.split(',').next()?.trim();
    parse_ip(first)
}

fn parse_single_ip(value: &HeaderValue) -> Option<String> {
    parse_ip(value.to_str().ok()?.trim())
}

fn parse_ip(candidate: &str) -> Option<String> {
    candidate.parse::<IpAddr>().ok().map(|ip| ip.to_string())
}

async fn audit_auth_event(
    pool: Option<&sqlx::PgPool>,
    actor_user_id: Option<uuid::Uuid>,
    action: &str,
    result: &str,
    metadata_entries: &[(&str, Value)],
) {
    let Some(pool) = pool else {
        return;
    };
    let metadata = match core_db::safe_audit_metadata(metadata_entries) {
        Ok(metadata) => metadata,
        Err(error) => {
            tracing::warn!(error = %error, action, "audit metadata rejected");
            serde_json::json!({})
        }
    };

    if let Err(error) = core_db::write_audit_event(
        pool,
        core_db::AuditEventInput {
            actor_user_id,
            action: action.to_string(),
            target_type: "auth_session".to_string(),
            target_id: None,
            result: result.to_string(),
            metadata,
        },
    )
    .await
    {
        tracing::warn!(error = %error, action, "failed to write audit event");
    }
}

fn csrf_error() -> (StatusCode, Json<AuthErrorResponse>) {
    auth_error(
        StatusCode::FORBIDDEN,
        "csrf_token_invalid",
        "a valid CSRF cookie and X-CSRF-Token header are required".to_string(),
        Some("Fetch /api/auth/csrf before browser cookie mutations and submit its token in X-CSRF-Token."),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn session_cookie_uses_http_only_lax_path_and_secure_when_configured() {
        let cookie = session_cookie("raw-token", true);
        assert!(cookie.starts_with("market_session=raw-token;"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("Max-Age=604800"));
        assert!(cookie.contains("Secure"));
    }

    #[test]
    fn session_cookie_omits_secure_for_insecure_local_config() {
        let cookie = session_cookie("raw-token", false);
        assert!(!cookie.contains("Secure"));
    }

    #[test]
    fn csrf_cookie_uses_lax_path_and_secure_without_http_only_when_configured() {
        let cookie = csrf_cookie("csrf-token", true);
        assert!(cookie.starts_with("market_csrf=csrf-token;"));
        assert!(!cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("Max-Age=604800"));
        assert!(cookie.contains("Secure"));
    }

    #[test]
    fn csrf_cookie_omits_secure_for_insecure_local_config() {
        let cookie = csrf_cookie("csrf-token", false);
        assert!(!cookie.contains("Secure"));
    }

    #[test]
    fn csrf_validation_accepts_matching_cookie_and_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("market_csrf=token-123"),
        );
        headers.insert(CSRF_HEADER_NAME, HeaderValue::from_static("token-123"));

        assert!(require_csrf(&headers).is_ok());
    }

    #[test]
    fn csrf_validation_rejects_missing_or_mismatched_tokens() {
        let mut mismatch = HeaderMap::new();
        mismatch.insert(
            header::COOKIE,
            HeaderValue::from_static("market_csrf=token-123"),
        );
        mismatch.insert(CSRF_HEADER_NAME, HeaderValue::from_static("token-456"));

        assert_eq!(
            require_csrf(&HeaderMap::new()).unwrap_err().0,
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            require_csrf(&mismatch).unwrap_err().0,
            StatusCode::FORBIDDEN
        );
    }

    #[test]
    fn csrf_gate_d_note_marks_foundation_active() {
        assert!(MeResponse {
            user: AuthUserResponse {
                id: uuid::Uuid::nil(),
                name: "User".to_string(),
                email: "user@example.com".to_string(),
                role: AuthRoleResponse {
                    code: "seller".to_string(),
                    name: "Seller".to_string(),
                },
            },
            session_expires_at: Utc::now(),
            gate_d_note:
                "CSRF token endpoint and validation are active for Gate D cookie mutations.",
        }
        .gate_d_note
        .contains("active"));
    }

    #[test]
    fn client_identity_uses_peer_ip_by_default() {
        let headers = HeaderMap::new();
        let peer = SocketAddr::from(([203, 0, 113, 10], 44321));

        assert_eq!(client_identity(peer, &headers), "203.0.113.10");
    }

    #[test]
    fn client_identity_ignores_forwarded_headers_for_non_loopback_peer() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("198.51.100.23"));
        let peer = SocketAddr::from(([203, 0, 113, 10], 44321));

        assert_eq!(client_identity(peer, &headers), "203.0.113.10");
    }

    #[test]
    fn client_identity_allows_forwarded_for_from_loopback_peer() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("198.51.100.23, 10.0.0.9"),
        );
        let peer = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

        assert_eq!(client_identity(peer, &headers), "198.51.100.23");
    }

    #[test]
    fn client_identity_falls_back_to_loopback_when_header_invalid() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("bad-ip"));
        headers.insert("x-real-ip", HeaderValue::from_static("also-bad"));
        let peer = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

        assert_eq!(client_identity(peer, &headers), "127.0.0.1");
    }
}
