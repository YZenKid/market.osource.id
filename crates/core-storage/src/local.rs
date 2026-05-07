use crate::{validate_upload, PutObjectInput, StorageError, StorageProvider, StoredObject};
use async_trait::async_trait;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LocalStorageProvider {
    root: PathBuf,
}

impl LocalStorageProvider {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn safe_path(&self, bucket: &str, object_key: &str) -> Result<PathBuf, StorageError> {
        if bucket.is_empty()
            || object_key.is_empty()
            || bucket.contains("..")
            || object_key.contains("..")
            || bucket.contains('/')
            || bucket.contains('\\')
            || object_key.contains('/')
            || object_key.contains('\\')
        {
            return Err(StorageError::InvalidFile("invalid object path".to_string()));
        }
        Ok(self.root.join(bucket).join(object_key))
    }
}

#[async_trait]
impl StorageProvider for LocalStorageProvider {
    async fn put_object(&self, input: PutObjectInput) -> Result<StoredObject, StorageError> {
        let validation = validate_upload(&input.original_filename, &input.bytes)?;
        let checksum = hex::encode(Sha256::digest(&input.bytes));
        let extension = Path::new(&input.original_filename)
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("bin")
            .to_ascii_lowercase();
        let object_key = format!("{}.{}", Uuid::new_v4(), extension);
        let target = self.safe_path(&input.bucket, &object_key)?;
        if let Some(parent) = target.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&target, &input.bytes).await?;
        Ok(StoredObject {
            bucket: input.bucket,
            object_key,
            size_bytes: input.bytes.len() as u64,
            checksum,
            mime_type: validation.mime_type,
            visibility: input.visibility,
        })
    }

    async fn delete_object(&self, bucket: &str, object_key: &str) -> Result<(), StorageError> {
        let target = self.safe_path(bucket, object_key)?;
        if target.exists() {
            tokio::fs::remove_file(target).await?;
        }
        Ok(())
    }
}
