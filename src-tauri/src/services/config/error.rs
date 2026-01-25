use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("加密失败: {0}")]
    EncryptFailed(String),

    #[error("其他加密错误: {0}")]
    Other(String),
}

pub type CryptoResult<T> = std::result::Result<T, CryptoError>;
