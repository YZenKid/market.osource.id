use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PutObjectInput {
    pub bucket: String,
    pub original_filename: String,
    pub bytes: Vec<u8>,
    pub visibility: FileVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredObject {
    pub bucket: String,
    pub object_key: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub mime_type: String,
    pub visibility: FileVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadObject {
    pub bucket: String,
    pub object_key: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileVisibility {
    Public,
    Authenticated,
    PrivateAdmin,
}

#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn put_object(&self, input: PutObjectInput) -> Result<StoredObject, StorageError>;
    async fn read_object(&self, bucket: &str, object_key: &str)
        -> Result<ReadObject, StorageError>;
    async fn delete_object(&self, bucket: &str, object_key: &str) -> Result<(), StorageError>;
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("invalid file: {0}")]
    InvalidFile(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
