//pub use pack_capnp::{Pack, PackData};
pub mod pack_capnp;

pub mod bare_msgs {
	pub static HELLOBYTES : [u8, ..9] = [72, 69, 76, 76, 79, 68, 69, 115, 117];
}

/*

pub struct NonceWrap(Nonce);
pub struct PublicKeyWrap(PublicKey);

#[deriving(Encodable, Decodable)]
pub enum PackData {
	Pubkey(PublicKeyWrap),
	Login(~str),
	Message(~str),

	ServerResponse(int),

	Quit(~str)
}

impl PackData {
	pub fn json_encode(&self) -> ~[u8] {
		let mut buf = MemWriter::new();
		{
			let mut enc = json::Encoder::new(&mut buf);
			self.encode(&mut enc);
		}
		buf.inner()
	}
}

#[deriving(Encodable, Decodable)]
pub struct Pack {
    nonce: NonceWrap,
    data: ~[u8] // PackData
}

impl Pack {
	pub fn json_encode(&self) -> ~[u8] {
		let mut buf = MemWriter::new();
		{
			let mut enc = json::Encoder::new(&mut buf);
			self.encode(&mut enc);
		}
		buf.inner()
		return ~[0];
	}
}

impl<T: Encoder> Encodable<T> for NonceWrap {
	fn encode(&self, encoder: &mut T) {
		let &NonceWrap(Nonce(arr)) = self;
		encoder.emit_seq(NONCEBYTES, |encoder| {
			for (i, elem) in arr.iter().enumerate() {
				encoder.emit_seq_elt(i, |encoder| elem.encode(encoder));
			}
		})
	}
}

impl<T: Decoder> Decodable<T> for NonceWrap {
	fn decode(decoder: &mut T) -> NonceWrap {
		decoder.read_seq(|decoder, len| {
			let mut bytes: [u8, ..NONCEBYTES] = [0, ..NONCEBYTES];
			for i in range(0u, len) {
				bytes[i] = decoder.read_seq_elt(i, |decoder| Decodable::decode(decoder));
			}
			NonceWrap(Nonce(bytes))
		})
	}
}

impl<T: Encoder> Encodable<T> for PublicKeyWrap {
	fn encode(&self, encoder: &mut T) {
		let &PublicKeyWrap(PublicKey(arr)) = self;
		encoder.emit_seq(PUBLICKEYBYTES, |encoder| {
			for (i, elem) in arr.iter().enumerate() {
				encoder.emit_seq_elt(i, |encoder| elem.encode(encoder));
			}
		})
	}
}

impl<T: Decoder> Decodable<T> for PublicKeyWrap {
	fn decode(decoder: &mut T) -> PublicKeyWrap {
		decoder.read_seq(|decoder, len| {
			let mut bytes: [u8, ..PUBLICKEYBYTES] = [0, ..PUBLICKEYBYTES];
			for i in range(0u, len) {
				bytes[i] = decoder.read_seq_elt(i, |decoder| Decodable::decode(decoder));
			}
			PublicKeyWrap(PublicKey(bytes))
		})
	}
}
*/