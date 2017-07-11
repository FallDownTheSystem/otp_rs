use std::env;
use std::io::{self, Read};

fn decrypt(encrypted_message: &[u8], key_str: &str) -> String {
	String::from_utf8(encrypted_message.iter().cloned().zip(key_str.bytes().cycle()).map(|(message_byte, key)| message_byte ^ key).collect()).unwrap()
}

fn main() {
	if let Some(key) = env::args().nth(1) {
		let mut stdin = io::stdin();
		let mut buffer = String::new();
		stdin.read_to_string(&mut buffer).unwrap();
		let encrypted_message: Vec<_> = buffer.trim().split(",").map(|s| s.trim().parse().unwrap()).collect();
		let decrypted_string = decrypt(&encrypted_message, &key);
		print!("{}", decrypted_string);
	} else {
		println!("Give key pls.");
	}
}
