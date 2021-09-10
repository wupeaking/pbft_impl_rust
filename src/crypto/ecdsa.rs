use crate::error::CounchError;
use hex;
use p256::ecdsa::SigningKey;
use p256::ecdsa::VerifyingKey;
use p256::EncodedPoint;
use rand_core::OsRng;
use ring::{agreement, rand, signature};

/// ## 随机生成一对公钥 私钥
/// 返回值为 私钥 公钥 错误
pub fn generate_key_pairs() -> Result<(String, String), CounchError> {
    let rng = rand::SystemRandom::new();
    let key_pair =
        signature::EcdsaKeyPair::generate_pkcs8(&signature::ECDSA_P256_SHA256_FIXED_SIGNING, &rng)?;

    let private_key = agreement::EphemeralPrivateKey::generate(&agreement::ECDH_P256, &rng)?;

    let k = SigningKey::random(&mut OsRng);
    // let p = k.to_bytes();
    // let k2 = SigningKey::from_bytes(&p);
    let pub_k = VerifyingKey::from(&k);
    let pub_k = VerifyingKey::to_encoded_point(&pub_k, false).to_bytes();

    return Ok((hex::encode(k.to_bytes()), hex::encode(pub_k)));
}

pub fn load_private_key(hex_str: String) -> Result<SigningKey, CounchError> {
    let private_key_bytes = hex::decode(hex_str)?;
    let private_key = SigningKey::from_bytes(&private_key_bytes)?;
    return Ok(private_key);
}

pub fn load_pub_key(hex_str: String) -> Result<VerifyingKey, CounchError> {
    let pub_key_bytes = hex::decode(hex_str)?;
    let point = EncodedPoint::from_bytes(&pub_key_bytes)?;
    let pub_key = VerifyingKey::from_encoded_point(&point)?;
    // let private_key = SigningKey::from_bytes(&private_key_bytes)?;
    return Ok(pub_key);
}

pub fn sign(sign_key: &SigningKey, content: &[u8]) -> Result<Vec<u8>, CounchError> {
    use p256::ecdsa::signature::Signer;

    let sig_ret = Signer::sign(sign_key, content);
    use p256::ecdsa::signature::Signature;
    return Ok(Vec::<u8>::from(sig_ret.as_bytes()));
}

pub fn verifier_sign(
    ver_key: &VerifyingKey,
    msg: &[u8],
    signed_value: &[u8],
) -> Result<bool, CounchError> {
    use p256::ecdsa::signature::Verifier;
    use p256::ecdsa::signature::Signature;
    let sig_instance = Signature::from_bytes(signed_value)?;

    // VerifyingKey::verify(&self, msg, signature)
    if let Err(_) = ver_key.verify(msg, &sig_instance) {
        return Ok(false);
    }
    return Ok(true);
}

pub fn sha256(msg: &[u8]) -> Vec<u8> {
    use ring::digest;
    // let one_shot = digest::digest(&digest::SHA384, b"hello, world");
    let mut ctx = digest::Context::new(&digest::SHA256);
    ctx.update(msg);
    // ctx.update(b", ");
    // ctx.update(b"world");
    let multi_part = ctx.finish();
    Vec::<u8>::from(multi_part.as_ref())
}

//
#[cfg(test)]
mod tests {
    #[test]
    fn load_pub_key_test() {
        match super::
        load_pub_key("042b327372fd541fbe4e8b6670856ed684e767dcd5ff489016a93d3229b7053e3e29369989e675f6ab531161dba05039f836c016d442975093f1db624342318efc".to_string()){
            Ok(_) => {},
            Err(err) => {
                panic!("{}", err);
            }
        }
        // assert_eq!(2 + 2, 4);
    }

    #[test]
    fn load_pri_key_test() {
        match super::load_private_key(
            "d66c3328785cf75db3ada5a77ac7ad04098142615295b5457bc65ab8e5ff63ce".to_string(),
        ) {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn sign_verify_test() {
        // 生成私钥和公钥
        let (priv_key, pub_key) = super::generate_key_pairs().unwrap();
        let msg = "hello world";
        let msg = super::sha256(msg.as_bytes());
        // 使用私钥签名
        let signer_key = super::load_private_key(priv_key).unwrap();
        let signed_value = super::sign(&signer_key, &msg).unwrap();
        // 使用公钥验证签名
        let ver_key = super::load_pub_key(pub_key).unwrap();
        let ok = super::verifier_sign(&ver_key, &msg, &signed_value).unwrap();
        assert_eq!(ok, true);
    }
}
