use crypto::{aes, aes::KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;
use std::iter::repeat;
use rustc_serialize::base64::{STANDARD, ToBase64};
use rand::{RngCore, rngs::OsRng};

// 验证加密解密, 可调用 cipher.py 验证
fn main() {
    let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    OsRng.fill_bytes(&mut key);
    let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
    OsRng.fill_bytes(&mut nonce);
    println!("key: {}", key.to_base64(STANDARD));
    println!("nonce: {}", nonce.to_base64(STANDARD));

    // 加密器
    let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
    let sec = "abcd";
    let mut output: Vec<u8> = repeat(0u8).take(sec.len()).collect();
    cipher.process(sec.as_bytes(), &mut output[..]);
    println!("{}", output.to_base64(STANDARD));
}
