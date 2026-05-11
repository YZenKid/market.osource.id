use crate::{
    validate_upload, PutObjectInput, ReadObject, StorageError, StorageProvider, StoredObject,
};
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

    pub async fn ensure_root(&self) -> Result<(), StorageError> {
        tokio::fs::create_dir_all(&self.root).await?;
        Ok(())
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

    async fn read_object(
        &self,
        bucket: &str,
        object_key: &str,
    ) -> Result<ReadObject, StorageError> {
        let target = self.safe_path(bucket, object_key)?;
        let bytes = tokio::fs::read(target).await?;
        Ok(ReadObject {
            bucket: bucket.to_string(),
            object_key: object_key.to_string(),
            bytes,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FileVisibility;

    fn test_root() -> PathBuf {
        std::env::temp_dir().join(format!("market-osource-storage-{}", Uuid::new_v4()))
    }

    #[tokio::test]
    async fn read_object_rejects_traversal_paths() {
        let root = test_root();
        let provider = LocalStorageProvider::new(&root);

        assert!(provider
            .read_object("payment-proofs", "../secret.png")
            .await
            .is_err());
        assert!(provider
            .read_object("../bucket", "proof.png")
            .await
            .is_err());

        let _ = tokio::fs::remove_dir_all(root).await;
    }

    #[tokio::test]
    async fn read_object_reads_previously_stored_private_object() {
        let root = test_root();
        let provider = LocalStorageProvider::new(&root);
        let stored = provider
            .put_object(PutObjectInput {
                bucket: "payment-proofs".to_string(),
                original_filename: "proof.png".to_string(),
                bytes: vec![0x89, b'P', b'N', b'G', 1, 2, 3],
                visibility: FileVisibility::PrivateAdmin,
            })
            .await
            .expect("object should store");

        let read = provider
            .read_object(&stored.bucket, &stored.object_key)
            .await
            .expect("object should read");

        assert_eq!(read.bytes, vec![0x89, b'P', b'N', b'G', 1, 2, 3]);
        let _ = tokio::fs::remove_dir_all(root).await;
    }
}
