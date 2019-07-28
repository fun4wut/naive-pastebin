use crate::utils::env::CRYPT_KEY;
use crate::utils::time::*;
use short_crypt::ShortCrypt;

const VALID_BYTES: usize = 8; // 取NanoTime的前8位

lazy_static! {
    static ref SC: ShortCrypt = { ShortCrypt::new(&*CRYPT_KEY) };
}

#[inline]
/// 将纳秒化为Key
pub fn nano_to_key(nano: NanoTime) -> String {
    SC.encrypt_to_qr_code_alphanumeric(&nano.to_ne_bytes()[..VALID_BYTES])
}

/// 将key转化为纳秒
pub fn key_to_nano(key: &str) -> Option<NanoTime> {
    SC.decrypt_qr_code_alphanumeric(key)
        .ok()
        .and_then(|v: Vec<u8>| {
            if v.len() == VALID_BYTES {
                Some(v)
            } else {
                None
            }
        })
        .map(|v: Vec<u8>| {
            let mut arr: [u8; 16] = [0; 16];
            arr[..VALID_BYTES].copy_from_slice(v.as_slice());
            u128::from_ne_bytes(arr)
        })
}

