use crate::StorageError;

pub const MAX_UPLOAD_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct UploadValidation {
    pub mime_type: String,
}

pub fn validate_upload(filename: &str, bytes: &[u8]) -> Result<UploadValidation, StorageError> {
    if bytes.is_empty() {
        return Err(StorageError::InvalidFile("file is empty".to_string()));
    }
    if bytes.len() > MAX_UPLOAD_BYTES {
        return Err(StorageError::InvalidFile(
            "file exceeds max size".to_string(),
        ));
    }
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err(StorageError::InvalidFile(
            "filename is not allowed".to_string(),
        ));
    }
    let extension = filename
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if extension == "svg" {
        return Err(StorageError::InvalidFile(
            "svg uploads are not allowed".to_string(),
        ));
    }
    let mime_type = match extension.as_str() {
        "jpg" | "jpeg" if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) => "image/jpeg",
        "png" if bytes.starts_with(&[0x89, b'P', b'N', b'G']) => "image/png",
        "webp" if bytes.len() > 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" => {
            "image/webp"
        }
        _ => {
            return Err(StorageError::InvalidFile(
                "unsupported or spoofed file type".to_string(),
            ))
        }
    };
    Ok(UploadValidation {
        mime_type: mime_type.to_string(),
    })
}
