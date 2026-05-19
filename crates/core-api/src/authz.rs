use crate::routes::auth::{AuthErrorResponse, SESSION_COOKIE_NAME};
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderMap, StatusCode},
    Json,
};
use core_app::AppState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedActor {
    pub user_id: uuid::Uuid,
    pub role_code: String,
    pub role_name: String,
}

impl From<core_db::CurrentSessionRecord> for AuthenticatedActor {
    fn from(session: core_db::CurrentSessionRecord) -> Self {
        Self {
            user_id: session.user_id,
            role_code: session.role_code,
            role_name: session.role_name,
        }
    }
}

impl FromRequestParts<AppState> for AuthenticatedActor {
    type Rejection = (StatusCode, Json<AuthErrorResponse>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        resolve_authenticated_actor(state, &parts.headers).await
    }
}

pub async fn resolve_authenticated_actor(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<AuthenticatedActor, (StatusCode, Json<AuthErrorResponse>)> {
    let Some(pool) = &state.db_pool else {
        return Err(auth_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "database_unavailable",
            "database is required for authentication".to_string(),
        ));
    };

    let raw_token = read_cookie(headers, SESSION_COOKIE_NAME).ok_or_else(unauthenticated)?;
    let session_hash = core_auth::hash_session_token(&raw_token);
    let session = core_db::get_current_session_by_hash(pool, &session_hash)
        .await
        .map_err(|error| {
            tracing::warn!(error = %error, "failed to read authenticated session");
            auth_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "session_lookup_failed",
                "session lookup failed; check backend logs for details".to_string(),
            )
        })?
        .ok_or_else(unauthenticated)?;

    Ok(session.into())
}

pub fn require_super_admin(
    actor: &AuthenticatedActor,
) -> Result<(), (StatusCode, Json<AuthErrorResponse>)> {
    if actor.role_code == "super_admin" {
        Ok(())
    } else {
        Err(auth_error(
            StatusCode::FORBIDDEN,
            "forbidden",
            "super admin access is required".to_string(),
        ))
    }
}

/// Allow `super_admin` or `admin`. Rejects `karyawan`, `seller`, and any other role.
pub fn require_admin_or_super(
    actor: &AuthenticatedActor,
) -> Result<(), (StatusCode, Json<AuthErrorResponse>)> {
    match actor.role_code.as_str() {
        "super_admin" | "admin" => Ok(()),
        _ => Err(auth_error(
            StatusCode::FORBIDDEN,
            "forbidden",
            "admin or super admin access is required".to_string(),
        )),
    }
}

/// Allow any operator role: `super_admin`, `admin`, `karyawan`, `seller`.
/// Rejects unauthenticated and any unknown role code.
pub fn require_operator(
    actor: &AuthenticatedActor,
) -> Result<(), (StatusCode, Json<AuthErrorResponse>)> {
    match actor.role_code.as_str() {
        "super_admin" | "admin" | "karyawan" | "seller" => Ok(()),
        _ => Err(auth_error(
            StatusCode::FORBIDDEN,
            "forbidden",
            "operator access is required".to_string(),
        )),
    }
}

/// Returns true when the actor is brand-scoped (seller or karyawan).
/// Used by handlers that need to decide between full-access and brand-filtered queries.
pub fn is_brand_scoped(actor: &AuthenticatedActor) -> bool {
    matches!(actor.role_code.as_str(), "seller" | "karyawan")
}

/// Returns true when the actor has full cross-brand read access (super_admin or admin).
pub fn is_full_access(actor: &AuthenticatedActor) -> bool {
    matches!(actor.role_code.as_str(), "super_admin" | "admin")
}

fn unauthenticated() -> (StatusCode, Json<AuthErrorResponse>) {
    auth_error(
        StatusCode::UNAUTHORIZED,
        "unauthenticated",
        "a valid session is required".to_string(),
    )
}

fn auth_error(
    status: StatusCode,
    error: &'static str,
    message: String,
) -> (StatusCode, Json<AuthErrorResponse>) {
    (
        status,
        Json(AuthErrorResponse {
            error,
            message,
            gate_d_note: None,
        }),
    )
}

pub fn read_cookie(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    cookie_header.split(';').find_map(|part| {
        let (name, value) = part.trim().split_once('=')?;
        (name == cookie_name && !value.is_empty()).then(|| value.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_runtime::{AppConfig, RuntimeMode};
    use secrecy::SecretString;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn test_state() -> AppState {
        AppState::new(AppConfig {
            runtime_mode: RuntimeMode::Vps,
            bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
            base_url: "http://127.0.0.1:7301".to_string(),
            database_url: SecretString::from("".to_string()),
            storage_path: std::env::temp_dir().to_string_lossy().to_string(),
            cors_allowed_origins: Vec::new(),
            cookie_secure: false,
        })
    }

    #[tokio::test]
    async fn auth_helper_rejects_no_cookie_before_session_lookup() {
        let mut state = test_state();
        state.db_pool = Some(sqlx::PgPool::connect_lazy("postgres://example.invalid/db").unwrap());

        let error = resolve_authenticated_actor(&state, &HeaderMap::new())
            .await
            .unwrap_err();

        assert_eq!(error.0, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn super_admin_helper_rejects_non_super_admin() {
        let actor = AuthenticatedActor {
            user_id: uuid::Uuid::nil(),
            role_code: "seller".to_string(),
            role_name: "Seller".to_string(),
        };

        assert_eq!(
            require_super_admin(&actor).unwrap_err().0,
            StatusCode::FORBIDDEN
        );
    }

    #[test]
    fn admin_or_super_helper_accepts_admin_and_super_admin() {
        for (role_code, role_name) in [
            ("super_admin", "Super Admin"),
            ("admin", "Admin"),
        ] {
            let actor = AuthenticatedActor {
                user_id: uuid::Uuid::nil(),
                role_code: role_code.to_string(),
                role_name: role_name.to_string(),
            };

            assert!(require_admin_or_super(&actor).is_ok());
        }
    }

    #[test]
    fn admin_or_super_helper_rejects_brand_scoped_roles() {
        for (role_code, role_name) in [("seller", "Seller"), ("karyawan", "Karyawan")] {
            let actor = AuthenticatedActor {
                user_id: uuid::Uuid::nil(),
                role_code: role_code.to_string(),
                role_name: role_name.to_string(),
            };

            assert_eq!(
                require_admin_or_super(&actor).unwrap_err().0,
                StatusCode::FORBIDDEN
            );
        }
    }

    #[test]
    fn role_scope_helpers_follow_expected_matrix() {
        let seller = AuthenticatedActor {
            user_id: uuid::Uuid::nil(),
            role_code: "seller".to_string(),
            role_name: "Seller".to_string(),
        };
        let karyawan = AuthenticatedActor {
            user_id: uuid::Uuid::nil(),
            role_code: "karyawan".to_string(),
            role_name: "Karyawan".to_string(),
        };
        let admin = AuthenticatedActor {
            user_id: uuid::Uuid::nil(),
            role_code: "admin".to_string(),
            role_name: "Admin".to_string(),
        };

        assert!(is_brand_scoped(&seller));
        assert!(is_brand_scoped(&karyawan));
        assert!(!is_brand_scoped(&admin));
        assert!(is_full_access(&admin));
        assert!(!is_full_access(&seller));
    }
}
