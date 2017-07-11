extern crate data_encoding;
use std::env;
use std::io::{self, Read};
use data_encoding::hex;

fn decrypt(e_msg: &[u8], key: &str) -> String {
	String::from_utf8(e_msg.iter().cloned().zip(key.bytes().cycle()).map(|(msg_byte, key_byte)| msg_byte ^ key_byte).collect()).unwrap()
}

fn main() {
	if let Some(key) = env::args().nth(1) {
		let mut stdin = io::stdin();
		let mut buffer = String::new();
		stdin.read_to_string(&mut buffer).unwrap();
		let e_msg = hex::decode(buffer.trim().as_ref()).unwrap();
		let decrypted_string = decrypt(&e_msg, &key);
		print!("{}", decrypted_string);
	} else {
		println!("Give key pls.");
	}
}
