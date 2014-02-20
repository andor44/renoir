extern crate sodiumoxide;
extern crate extra;
extern crate serialize;

use sodiumoxide::crypto;
use sodiumoxide::crypto::asymmetricbox::{PublicKey, seal, gen_nonce, PUBLICKEYBYTES, NONCEBYTES, open};

use std::io::{MemWriter, BufWriter};
use std::io::net::tcp::{TcpStream};
use std::io::net::ip::{SocketAddr};
use std::io::timer::sleep;
use std::vec::MutableCloneableVector;
use extra::json;
use serialize::{Encodable};

use super::comm;
use helpers::{KeyBytes, bytes_to_pubkey};

use capnp::{serialize_packed, serialize};
use capnp::message::{MallocMessageBuilder, MessageBuilder, DEFAULT_READER_OPTIONS, MessageReader};

fn create_pack() {

}

pub fn main() {
	let (c_pubkey, c_privkey) = crypto::asymmetricbox::gen_keypair();
	println!("my pubkey:{:?}\n\n", c_pubkey.key_bytes());
	let server_addr : SocketAddr = from_str("127.0.0.1:44944").expect("Unable to parse address");
	let mut server_conn = TcpStream::connect(server_addr).ok().expect("Failed to connect");
	server_conn.write(comm::bare_msgs::HELLOBYTES);

	let initial_response = server_conn.read_bytes(9).unwrap();
	if (initial_response.as_slice() != comm::bare_msgs::HELLOBYTES) {
		fail!("Wrong hello response from server");
	}
	println!("Got good hello response from server, waiting for public key");

	let mut recv_key : [u8, ..crypto::asymmetricbox::PUBLICKEYBYTES] = [0, ..PUBLICKEYBYTES];
	recv_key.copy_from(server_conn.read_bytes(PUBLICKEYBYTES).unwrap());
	let server_pubkey = PublicKey(recv_key);
	println!("Server signature is: {:?}", server_pubkey);
	{
		let nonce = gen_nonce();
		let mut message = MallocMessageBuilder::new_default();
		let pack = message.init_root::<comm::pack_capnp::Pack::Builder>();
		pack.init_nonce(NONCEBYTES);
		pack.set_nonce(nonce.key_bytes());
		let mut real_msg = MallocMessageBuilder::new_default();
		let pdata = real_msg.init_root::<comm::pack_capnp::PackData::Builder>();
		pdata.init_pubkey(PUBLICKEYBYTES);
		pdata.set_pubkey(c_pubkey.key_bytes());
		let mut buf = MemWriter::new();
		serialize_packed::write_packed_message_unbuffered(&mut buf, &real_msg);
		//let enc = seal(buf.unwrap(), &nonce, &server_pubkey, &c_privkey);
		let b2 = buf.unwrap();
		pack.init_data(b2.len());
		pack.set_data(b2);
		serialize_packed::write_packed_message_unbuffered(&mut server_conn, &message);
	}
	
	{
		let nonce = gen_nonce();
		let mut message = MallocMessageBuilder::new_default();
		let pack = message.init_root::<comm::pack_capnp::Pack::Builder>();
		pack.init_nonce(NONCEBYTES);
		pack.set_nonce(nonce.key_bytes());
		let mut real_msg = MallocMessageBuilder::new_default();
		let pdata = real_msg.init_root::<comm::pack_capnp::PackData::Builder>();
		pdata.init_login();
		let login = pdata.init_login();
		login.set_name("andor");
		let mut buf = MemWriter::new();
		serialize_packed::write_packed_message_unbuffered(&mut buf, &real_msg);
		let b2 = buf.unwrap();
		let enc = seal(b2, &nonce, &server_pubkey, &c_privkey);
		pack.init_data(enc.len());
		pack.set_data(enc);
		serialize_packed::write_packed_message_unbuffered(&mut server_conn, &message);
	}

	sleep(100000);

	{
		let mut message = MallocMessageBuilder::new_default();
		let pack = message.init_root::<comm::pack_capnp::PackData::Builder>();

		pack.set_quit("lelz!!11oneone");

		serialize_packed::write_packed_message_unbuffered(&mut server_conn, &message);
	}

}