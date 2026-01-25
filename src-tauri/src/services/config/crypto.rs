use base64;
use base64::{engine::general_purpose, Engine};
use chacha20poly1305::{aead::Aead, AeadCore, ChaCha20Poly1305, Key, KeyInit};
use rand::RngCore;
use rand_core::OsRng;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::config::{CryptoError, CryptoResult};

/// 加密密钥长度（32字节）
pub const KEY_LENGTH: usize = 32;
/// 随机数长度（12字节）
pub const NONCE_LENGTH: usize = 12;

/// 加密上下文
#[derive(Clone)]
pub struct CryptoContext {
    encryption_key: Arc<Mutex<[u8; KEY_LENGTH]>>,
}

impl CryptoContext {
    /// 创建新的加密上下文
    pub async fn new(encryption_key: [u8; KEY_LENGTH]) -> Self {
        Self {
            encryption_key: Arc::new(Mutex::new(encryption_key)),
        }
    }

    /// 生成随机加密密钥
    pub fn generate_random_key() -> [u8; KEY_LENGTH] {
        let mut key = [0u8; KEY_LENGTH];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }

    /// 加密配置值
    pub async fn encrypt_value(&self, value: &str) -> CryptoResult<String> {
        let encryption_key = self.encryption_key.lock().await;

        let cipher = ChaCha20Poly1305::new(Key::from_slice(&*encryption_key));

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher.encrypt(&nonce, value.as_bytes()).map_err(|e| CryptoError::EncryptFailed(e.to_string()))?;

        let mut combined = Vec::new();
        combined.extend_from_slice(nonce.as_slice());
        combined.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(combined))
    }

    /// 解密配置值
    pub async fn decrypt_value(&self, encrypted_value: &str) -> CryptoResult<String> {
        let encryption_key = self.encryption_key.lock().await;
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&*encryption_key));

        // 解码 Base64
        let combined = general_purpose::STANDARD.decode(encrypted_value).map_err(|e| CryptoError::Other(e.to_string()))?;

        if combined.len() < NONCE_LENGTH {
            return Err(CryptoError::Other("Invalid encrypted data".to_string()));
        }

        // 分离 nonce 和 ciphertext
        let nonce = &combined[..NONCE_LENGTH];
        let ciphertext = &combined[NONCE_LENGTH..];

        // 解密
        let plaintext = cipher.decrypt(nonce.into(), ciphertext).map_err(|e| CryptoError::Other(e.to_string()))?;

        String::from_utf8(plaintext).map_err(|e| CryptoError::Other(e.to_string()))
    }

    // /// 更新加密密钥
    // pub async fn update_encryption_key(&self, new_key: [u8; KEY_LENGTH]) {
    //     let mut encryption_key = self.encryption_key.lock().await;
    //     *encryption_key = new_key;
    // }
}
