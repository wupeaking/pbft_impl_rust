use thiserror::Error;
use std::io;
use ring::error;

#[derive(Error, Debug)]
pub enum CounchError {
    #[error("IO连接错误 {0}")]
    IOError(#[from] io::Error),

    #[error("帧解析错误 {0}")]
    FrameParse(String),

    #[error("加密错误 {0}")]
    CryptoError(String),

    #[error("调用Ring加密模块出错")]
    CryptoRingErr,

    #[error("未知错误 {0}")]
    Unknown(String),
}

 impl From<ring::error::Unspecified> for CounchError {
     fn from(_: ring::error::Unspecified) -> Self { CounchError::CryptoRingErr }
 }