use core_runtime::RuntimeMode;
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SetupRequest {
    pub marketplace_name: String,
    pub admin_name: String,
    pub admin_email: String,
    pub admin_password: SecretString,
}

#[derive(Debug, Clone)]
pub struct ValidatedSetup {
    pub marketplace_name: String,
    pub admin_name: String,
    pub admin_email: String,
}

#[derive(Debug, Clone)]
pub struct SetupContext {
    pub base_url: String,
    pub runtime_mode: RuntimeMode,
    pub core_version: String,
    pub storage_path: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SetupValidationError {
    #[error("marketplace name is required")]
    MarketplaceNameRequired,
    #[error("admin name is required")]
    AdminNameRequired,
    #[error("admin email is invalid")]
    AdminEmailInvalid,
    #[error("admin password must be at least 12 characters")]
    AdminPasswordTooShort,
}

pub fn validate_setup_request(
    request: &SetupRequest,
) -> Result<ValidatedSetup, SetupValidationError> {
    let marketplace_name = request.marketplace_name.trim();
    if marketplace_name.is_empty() {
        return Err(SetupValidationError::MarketplaceNameRequired);
    }

    let admin_name = request.admin_name.trim();
    if admin_name.is_empty() {
        return Err(SetupValidationError::AdminNameRequired);
    }

    let admin_email = request.admin_email.trim().to_lowercase();
    if !admin_email.contains('@') || admin_email.starts_with('@') || admin_email.ends_with('@') {
        return Err(SetupValidationError::AdminEmailInvalid);
    }

    if request.admin_password.expose_secret().chars().count() < 12 {
        return Err(SetupValidationError::AdminPasswordTooShort);
    }

    Ok(ValidatedSetup {
        marketplace_name: marketplace_name.to_string(),
        admin_name: admin_name.to_string(),
        admin_email,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_setup_request() {
        let request = SetupRequest {
            marketplace_name: " Market ".to_string(),
            admin_name: " Admin ".to_string(),
            admin_email: "ADMIN@example.com ".to_string(),
            admin_password: SecretString::from("very-secure-password".to_string()),
        };

        let validated = validate_setup_request(&request).expect("valid setup request");
        assert_eq!(validated.marketplace_name, "Market");
        assert_eq!(validated.admin_name, "Admin");
        assert_eq!(validated.admin_email, "admin@example.com");
    }

    #[test]
    fn rejects_short_password() {
        let request = SetupRequest {
            marketplace_name: "Market".to_string(),
            admin_name: "Admin".to_string(),
            admin_email: "admin@example.com".to_string(),
            admin_password: SecretString::from("short".to_string()),
        };

        assert!(matches!(
            validate_setup_request(&request),
            Err(SetupValidationError::AdminPasswordTooShort)
        ));
    }
}
