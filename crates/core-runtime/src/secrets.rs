use secrecy::SecretString;

#[derive(Debug, Clone)]
pub struct SecretRef(pub String);

pub trait SecretProvider: Send + Sync {
    fn get(&self, key: &SecretRef) -> anyhow::Result<SecretString>;
}
