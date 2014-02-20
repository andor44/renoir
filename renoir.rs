extern crate sodiumoxide;
extern crate extra;
extern crate serialize;
extern crate sync;
extern crate capnp;

use std::os;

mod server;
mod client;
mod comm;
mod helpers;

fn print_usage(prog_name: &str) {
	println!("{0} usage: {0} [server|client [ip]]", prog_name);
}

fn bench() {
	for _ in range(0, 1000) {
		std::task::spawn(proc() { client::main(); });
		std::io::timer::sleep(50);
	}
	std::io::timer::sleep(100000);
}

fn main() {
	if os::args().len() < 2 {
		print_usage(os::args()[0]);
		return;
	}

	match os::args()[1] {
		~"server" => server::main(),
		~"client" => client::main(),
		~"bench" => bench(),
		_ => print_usage(os::args()[0])
	}
}
