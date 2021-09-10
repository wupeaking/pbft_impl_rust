// mod model;
use pbft_impl_rust::model::Account;
use pbft_impl_rust::model::address;
use pbft_impl_rust::model::amount;
use pbft_impl_rust::crypto::ecdsa;
use protobuf::Message;
use hex;
fn main() {
    let mut acc = Account::new();
    let mut addr = address::new();
    let code = vec![1 as u8; 10];
    let mut balance = amount::new();
    balance.set_amount("12000".to_string());

    addr.set_address("aaaa".to_string());
    acc.set_id(addr);
    acc.set_code(code);
    acc.set_balance(balance);

    let buf = match acc.write_to_bytes() {
        Ok(buf) => buf,
        Err(err) => {
            println!("{:?}", err);
            vec![]
        }
    };
    println!("hex encode: {}", hex::encode(buf));
    let key_pair = match ecdsa::generate_key_pairs() {
        Ok(pairs) => pairs,
        Err(err) => {
            panic!("{:?}", err);
        }
    };
    println!("private: {:?}, pub: {:?}", key_pair.0, key_pair.1);
    
}

