use std::sync::OnceLock;

use base64::{engine::general_purpose as b64, Engine as _};
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use rand::{thread_rng, Rng};
use serde::{Deserializer, Serializer};

use crate::toml::error::ConfigError;

/// 加密密钥长度
const KEY_LENGTH: usize = 32;
/// Nonce 长度
const NONCE_LENGTH: usize = 12;
/// 认证标签长度
const TAG_LENGTH: usize = 16;

/// 加密密钥（全局单例）
static ENCRYPTION_KEY: OnceLock<Key> = OnceLock::new();

/// 初始化加密密钥
pub fn init_encryption_key() -> Result<(), ConfigError> {
    let key_hex = std::env::var("APP_ENCRYPTION_KEY")
        .map_err(|_| ConfigError::KeyGenerationError("Missing APP_ENCRYPTION_KEY".into()))?;

    let raw = hex::decode(key_hex)
        .map_err(|_| ConfigError::KeyGenerationError("Invalid hex key".into()))?;

    if raw.len() != KEY_LENGTH {
        return Err(ConfigError::KeyGenerationError(
            "Invalid key length (expected 32 bytes)".into(),
        ));
    }

    let key = Key::from_slice(&raw);
    ENCRYPTION_KEY
        .set(*key)
        .map_err(|_| ConfigError::KeyGenerationError("Key already initialized".into()))
}

/// 获取加密密钥
fn get_key() -> Result<&'static Key, ConfigError> {
    ENCRYPTION_KEY.get().ok_or_else(|| {
        ConfigError::KeyGenerationError("Encryption key not initialized".to_string())
    })
}

/// 加密字符串
pub fn encrypt_string(data: &str) -> Result<String, ConfigError> {
    let key = get_key()?;
    let cipher = ChaCha20Poly1305::new(key);

    // 生成随机 nonce
    let mut nonce_bytes = [0u8; NONCE_LENGTH];
    thread_rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from(nonce_bytes);

    // 加密数据
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes().as_ref())
        .map_err(|_| ConfigError::EncryptionError("Encryption failed".into()))?;

    // 组合 nonce 和密文
    let mut combined = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    // Base64 编码
    let encoded = b64::URL_SAFE_NO_PAD.encode(&combined);

    Ok(encoded)
}

/// 解密字符串
pub fn decrypt_string(encoded: &str) -> Result<String, ConfigError> {
    let key = get_key()?;
    let cipher = ChaCha20Poly1305::new(key);

    // Base64 解码
    let combined = b64::URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(ConfigError::Base64DecodeError)?;

    // 分离 nonce 和密文
    if combined.len() < NONCE_LENGTH + TAG_LENGTH {
        return Err(ConfigError::DecryptionError(
            "Invalid encrypted data".into(),
        ));
    }

    let nonce_bytes = &combined[0..NONCE_LENGTH];
    let nonce = Nonce::from_slice(nonce_bytes);
    let ciphertext = &combined[NONCE_LENGTH..];

    // 解密数据
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| ConfigError::DecryptionError("Decryption failed".into()))?;

    let plaintext = String::from_utf8(plaintext).map_err(ConfigError::Utf8Error)?;

    Ok(plaintext)
}

/// Serde 加密字段序列化辅助函数
pub fn serialize<S>(data: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let encrypted = encrypt_string(data).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&encrypted)
}

/// Serde 加密字段反序列化辅助函数
pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;
    let encrypted = String::deserialize(deserializer)?;
    decrypt_string(&encrypted).map_err(serde::de::Error::custom)
}

/// 加密字段的 Serde 支持
pub mod encrypted_field {
    pub use super::{deserialize, serialize};
}
