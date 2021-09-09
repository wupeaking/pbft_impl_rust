use thiserror::Error;
use std::io;
// use rocksdb::Error;

#[derive(Error, Debug)]
pub enum CounchError {
    #[error("IO连接错误 {0}")]
    IOError(#[from] io::Error),
    #[error("帧解析错误 {0}")]
    FrameParse(String),
    #[error("数据库操作错误 {0}")]
    CryptoError(String),
    #[error("未知错误 {0}")]
    Unknown(String),
}