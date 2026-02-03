use base64;
use base64::{engine::general_purpose, Engine};
use chacha20poly1305::{aead::Aead, AeadCore, ChaCha20Poly1305, Key, KeyInit};
use log::{debug, error, warn};
use rand::RngCore;
use rand_core::OsRng;
use std::sync::Arc;
use tauri::AppHandle;
use tauri_plugin_keyring::KeyringExt;
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
    pub fn get_or_generate_random_key(app_handle: AppHandle, service: &str, user: &str) -> [u8; KEY_LENGTH] {
        let key_vec = match app_handle.keyring().get_secret(service, user) {
            Ok(Some(key)) => key,
            Ok(None) => {
                let mut key = [0u8; KEY_LENGTH];
                rand::thread_rng().fill_bytes(&mut key);
                match app_handle.keyring().set_secret(service, user, &key) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("生成新 Encryption Salt 失败：向系统设置加密凭据失败。详细信息: {:?}", e);
                        panic!("生成新 Encryption Salt 失败：向系统设置加密凭据失败。详细信息: {:?}", e);
                    }
                };
                key.to_vec()
            }
            Err(e) => {
                error!("读取已有的 Encryption Salt 失败：发生意外错误。详细信息: {:?}", e);
                panic!("读取已有的 Encryption Salt 失败：发生意外错误。详细信息: {:?}", e);
                // let mut key = [0u8; KEY_LENGTH];
                // rand::thread_rng().fill_bytes(&mut key);
                // match app_handle.keyring().set_secret(service, user, &key) {
                //     Ok(_) => (),
                //     Err(e) => {
                //         error!("生成新 Encryption Salt 失败：向系统设置加密凭据失败。详细信息: {:?}", e);
                //         panic!("生成新 Encryption Salt 失败：向系统设置加密凭据失败。详细信息: {:?}", e);
                //     }
                // };
                // key.to_vec()
            }
        };

        let len = key_vec.len();
        match key_vec.try_into() {
            Ok(k) => k,
            Err(_) => {
                error!("生成新 Encryption Salt 失败：从系统读取的 key 长度不合法");
                panic!("生成新 Encryption Salt 失败：key 必须是 {} 字节", KEY_LENGTH);
            }
        }
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
