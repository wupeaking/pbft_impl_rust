use ring::error;
use std::io;
use thiserror::Error;
use hex;
// use p256::ecdsa::Error;
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

    #[error("hex解析错误")]
    HexError(#[from] hex::FromHexError),

    #[error("调用P256加密模块出错 {0}")]
    CryproP256Err(#[from] p256::ecdsa::Error),

    #[error("调用elliptic_curve加密模块出错 {0}")]
    CryproCureErr(#[from] p256::elliptic_curve::Error),

    #[error("未知错误 {0}")]
    Unknown(String),

    #[error("未实现此方法")]
    UnImpl,
}

impl From<ring::error::Unspecified> for CounchError {
    fn from(_: ring::error::Unspecified) -> Self {
        CounchError::CryptoRingErr
    }
}
