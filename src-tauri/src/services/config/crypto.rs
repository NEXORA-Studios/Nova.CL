use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, KeyInit, Nonce};
use rand::RngCore;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 加密密钥长度（32字节）
pub const KEY_LENGTH: usize = 32;
/// 随机数长度（12字节）
pub const NONCE_LENGTH: usize = 12;
/// 密钥ID
pub const CONFIG_ENCRYPTION_KEY_ID: &str = "config_encryption_key";

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
    pub async fn encrypt_value(&self, value: &str) -> Result<String, Box<dyn std::error::Error>> {
        let encryption_key = self.encryption_key.lock().await;
        let cipher = ChaCha20Poly1305::new(encryption_key.into());
        
        // 生成随机 nonce
        let mut nonce_bytes = vec![0u8; NONCE_LENGTH];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // 加密
        let ciphertext = cipher.encrypt(nonce, value.as_bytes().into())?;
        
        // 组合 nonce 和 ciphertext，使用 Base64 编码
        let mut combined = Vec::new();
        combined.extend(nonce_bytes);
        combined.extend(ciphertext);
        
        Ok(base64::encode(combined))
    }
    
    /// 解密配置值
    pub async fn decrypt_value(&self, encrypted_value: &str) -> Result<String, Box<dyn std::error::Error>> {
        let encryption_key = self.encryption_key.lock().await;
        let cipher = ChaCha20Poly1305::new(encryption_key.into());
        
        // 解码 Base64
        let combined = base64::decode(encrypted_value)?;
        if combined.len() < NONCE_LENGTH {
            return Err("Invalid encrypted data".into());
        }
        
        // 分离 nonce 和 ciphertext
        let nonce = &combined[..NONCE_LENGTH];
        let ciphertext = &combined[NONCE_LENGTH..];
        
        // 解密
        let plaintext = cipher.decrypt(nonce.into(), ciphertext.into())?;
        
        Ok(String::from_utf8(plaintext)?) 
    }
    
    /// 更新加密密钥
    pub async fn update_encryption_key(&self, new_key: [u8; KEY_LENGTH]) {
        let mut encryption_key = self.encryption_key.lock().await;
        *encryption_key = new_key;
    }
}