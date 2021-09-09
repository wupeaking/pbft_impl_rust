use crate::error::CounchError;
use ring::{rand, signature};


/// ## 随机生成一对公钥 私钥 
/// 返回值为 私钥 公钥 错误 
fn generate_key_pairs() -> Result<(String, String), CounchError >{
    let rng = rand::SystemRandom::new();
    let key_pair = signature::EcdsaKeyPair::generate_pkcs8(&signature::ECDSA_P256_SHA256_FIXED_SIGNING,
         &rng)?;

    if true {
        return Ok(("".to_string(), "".to_string()));
    }
	return Err(CounchError::CryptoError("xxxx".to_string()));
}

// 