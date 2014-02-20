extern crate sodiumoxide;
use sodiumoxide::crypto::asymmetricbox::{PublicKey, SecretKey, PUBLICKEYBYTES, NONCEBYTES, Nonce};
use std::vec::MutableCloneableVector;

pub trait KeyBytes {
	fn key_bytes<'a>(&'a self) -> &'a [u8];
}

impl KeyBytes for PublicKey {
	fn key_bytes<'a>(&'a self) -> &'a [u8] {
		match self {
			&PublicKey(ref bytes) => bytes.as_slice()
		}
	}
}

impl KeyBytes for SecretKey {
	fn key_bytes<'a>(&'a self) -> &'a [u8] {
		match self {
			&SecretKey(ref bytes) => bytes.as_slice()
		}
	}
}

impl KeyBytes for Nonce {
	fn key_bytes<'a>(&'a self) -> &'a [u8] {
		match self {
			&Nonce(ref bytes) => bytes.as_slice()
		}
	}
}

pub fn bytes_to_pubkey(bytes: &[u8]) -> PublicKey {
	let mut buf: [u8, ..PUBLICKEYBYTES] = [0, ..PUBLICKEYBYTES];
	buf.copy_from(bytes);
	PublicKey(buf)
}

pub fn bytes_to_nonce(bytes: &[u8]) -> Nonce {
	let mut buf: [u8, ..NONCEBYTES] = [0, ..NONCEBYTES];
	buf.copy_from(bytes);
	Nonce(buf)
}




/*pub trait GetInner<T> {
	fn get_inner<'a>(&'a self) -> &'a T;
}

impl<Nonce> GetInner<Nonce> for consts::NonceWrap {
	fn get_inner<'a>(&'a self) -> &'a Nonce {
		match *self {
			consts::NonceWrap(ref inner) => inner
		}
	}
}*/
