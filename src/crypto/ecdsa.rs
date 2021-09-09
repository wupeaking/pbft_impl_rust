use crate::error::CounchError;


/// ## 随机生成一对公钥 私钥 
/// 返回值为 私钥 公钥 错误 
fn generate_key_pairs() -> Result<(String, String), CounchError> {
    if true {
        return Ok(("".to_string(), "".to_string()));
    }
	return Err(CounchError::CryptoError("xxxx".to_string()));
}