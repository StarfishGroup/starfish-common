use anyhow::Result;
use base64::prelude::*;
use ring::hmac;

pub fn hmac_sha256_to_base64<T: AsRef<[u8]>>(key: T, msg: T) -> String {
	let key = hmac::Key::new(hmac::HMAC_SHA256, key.as_ref());
	let signature = hmac::sign(&key, msg.as_ref());
	BASE64_STANDARD.encode(signature.as_ref())
}

pub fn base64_encode<T: AsRef<[u8]>>(msg: T) -> String {
	BASE64_STANDARD.encode(msg)
}

pub fn base64_decode<T: AsRef<[u8]>>(msg: T) -> Result<Vec<u8>> {
	let result = BASE64_STANDARD.decode(msg)?;
	Ok(result)
}
