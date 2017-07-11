extern crate data_encoding;
use std::env::args;
use std::io::{self, Read};
use data_encoding::hex;

fn encrypt(msg: &str, key: &str) -> Vec<u8> {
	msg.bytes().zip(key.bytes().cycle()).map(|(msg_byte, key_byte)| msg_byte ^ key_byte).collect()
}

fn main() {
	let mut args = args();
	if let Some(key) = args.nth(1) {
		let mut stdin = io::stdin();
		let mut buffer = String::new();
		stdin.read_to_string(&mut buffer).unwrap();
		let e_msg = encrypt(buffer.trim(), &key);
		println!("{}", hex::encode(&e_msg));
	} else {
		println!("Give key pls.");
	}
}
