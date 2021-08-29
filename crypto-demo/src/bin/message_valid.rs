use crypto::sha2::Sha256;
use crypto::mac::Mac;
use crypto::hmac::Hmac;
use std::iter::repeat;
use rustc_serialize::base64::{STANDARD, ToBase64};
use rustc_serialize::hex::ToHex;
use rand::{RngCore, rngs::OsRng};

/// 消息验证
/// python hmac_test.py 可检查消息是否合法
fn main() {
    // let input = "abcd";
    // let mut sha = Sha256::new();
    // sha.input_str(input);
    // println!("{}", sha.result_str());

    let mut key: Vec<u8> = repeat(0u8).take(32).collect();
    OsRng.fill_bytes(&mut key);
    let message = "abcd";
    println!("key: {}", key.to_base64(STANDARD));

    let mut hmac = Hmac::new(Sha256::new(), &key);
    hmac.input(message.as_bytes());
    println!("HMAC digest: {}", hmac.result().code().to_hex());
}
