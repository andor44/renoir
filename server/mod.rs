extern crate sodiumoxide;
extern crate serialize;
extern crate sync;
extern crate extra;

use std::io::{Listener, Acceptor};
use std::io::{MemReader, BufReader};
use std::io::net::ip::{SocketAddr};
use std::io::net::tcp::{TcpListener, TcpStream};
use std::from_str::{from_str};
use capnp::message::{MallocMessageBuilder, MessageBuilder, DEFAULT_READER_OPTIONS, MessageReader};
use capnp::serialize_packed;

use sync::Arc;
use sodiumoxide::crypto::asymmetricbox::{PublicKey, SecretKey, open, seal, gen_keypair};
use helpers::{KeyBytes, bytes_to_pubkey, bytes_to_nonce};
use super::comm;

fn process_new_connection(client: TcpStream, my_pubkey: Arc<PublicKey>, my_privkey: Arc<SecretKey>) {
	use capnp::serialize_packed;
	use capnp::message::{MallocMessageBuilder, MessageBuilder, DEFAULT_READER_OPTIONS, MessageReader};

	let mut client = client;
	println!("New connection from '{:s}'", client.peer_name().unwrap().to_str());

	let hellopack = client.read_bytes(9).unwrap();
	if (hellopack.as_slice() == comm::bare_msgs::HELLOBYTES) {
		println!("Connection hello received");
	}
	else {
		fail!("sad :(");
	}

	client.write(comm::bare_msgs::HELLOBYTES);
	client.write(my_pubkey.get().key_bytes());

	let client_pubkey = ~{
		let reader = serialize_packed::new_reader_unbuffered(&mut client, DEFAULT_READER_OPTIONS).unwrap();
		let pack = reader.get_root::<comm::pack_capnp::Pack::Reader>();

		let mut bufreader = BufReader::new(pack.get_data());
		let datareader = serialize_packed::new_reader_unbuffered(&mut bufreader, DEFAULT_READER_OPTIONS).unwrap();
		let packdata = datareader.get_root::<comm::pack_capnp::PackData::Reader>();

		match packdata.which() {
			Some(comm::pack_capnp::PackData::Pubkey(key)) => bytes_to_pubkey(key),
			_ => { println!("Client didn't send pubkey, disconnecting"); return; }
		}
		
	};

	println!("got client key:\n{:?}", client_pubkey);

	let mut clientname = ~"";
	loop {
		let reader = serialize_packed::new_reader_unbuffered(&mut client, DEFAULT_READER_OPTIONS).unwrap();
		let pack = reader.get_root::<comm::pack_capnp::Pack::Reader>();

		let nonce = bytes_to_nonce(pack.get_nonce());
		let databytes = match open(pack.get_data(), &nonce, client_pubkey, my_privkey.get()) {
			Some(bytes) => bytes,
			None => { println!("WARNING! Decrypt failed! "); continue; }
		};

		let mut bufreader = BufReader::new(databytes);
		let datareader = serialize_packed::new_reader_unbuffered(&mut bufreader, DEFAULT_READER_OPTIONS).unwrap();
		let packdata = datareader.get_root::<comm::pack_capnp::PackData::Reader>();

		match packdata.which() {
			Some(comm::pack_capnp::PackData::Login(login)) => { 
				println!("{:s} logged in", login.get_name());
				clientname = login.get_name().to_owned();
			},
			Some(comm::pack_capnp::PackData::Message(message)) => { 
				println!("<{:s}>{:s}", clientname, message.get_message());
			},
			Some(comm::pack_capnp::PackData::Quit(reason)) => { 
				println!("quitreason: {:s}", reason); 
				break; 
			},
			_ => println!("wut")
		}	
	}
}

fn writer_task() {
	
}

fn setup_sync_task() {

}

pub fn main() {
	sodiumoxide::init();
	
	let (s_pubkey, s_privkey) = gen_keypair();
	let a_pubkey = Arc::new(s_pubkey);
	let a_privkey = Arc::new(s_privkey);
	let bind_addr : SocketAddr = from_str("0.0.0.0:44944").unwrap();
	let s_listener = TcpListener::bind(bind_addr).ok().expect("Failed to bind");
	let mut s_acceptor = s_listener.listen().ok().expect("Failed to create connection listener");

	loop {
		let pub_clone = a_pubkey.clone();
		let priv_clone = a_privkey.clone();

		match s_acceptor.accept() {
			Ok(c_sock) => spawn(proc(){process_new_connection(c_sock, pub_clone, priv_clone)}),
			Err(err) => println!("Failed connection attempt: {:?}", err)
		}
	}

}