#[macro_use]
extern crate clap;

use std::process;

fn main() {
    let matches = clap_app!(ciphers =>
        (version: "0.1.0")
        (author: "Natalia Maximo <iam@natalia.dev>")
        (about: "Offers decryption and encryption of simple substitution ciphers")
        (@arg TEXT: "The text to apply the ciphers to")
        (@arg encrypt: -e --encrypt "Applies the encryption process to the text")
        (@arg caesar: -c --caesar "Uses the caesar cipher")
        (@arg atbash: -a --atbash "Uses the atbash cipher")
    ).get_matches();

    if let Err(e) = ciphers::run(matches) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}