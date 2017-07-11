use std::env::args;
use std::io::{self, Read};

fn encrypt(message: &str, key_str: &str) -> Vec<u8> {
	message.bytes().zip(key_str.bytes().cycle()).map(|(message_byte, key)| message_byte ^ key).collect()
}

fn main() {
	let mut args = args();
	if let Some(key) = args.nth(1) {
		let mut stdin = io::stdin();
		let mut buffer = String::new();
		stdin.read_to_string(&mut buffer).unwrap();
		println!("{:?}", encrypt(buffer.trim(), &key));
	} else {
		println!("Give key pls.");
	}
}
