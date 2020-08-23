#[macro_use]
extern crate clap;

use std::process;

fn main() {
    let matches = clap_app!(Ciphers =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Natalia Maximo <iam@natalia.dev>")
        (about: "Offers decryption and encryption of simple substitution ciphers")
        (after_help: "Use pipes to apply multiple ciphers!\n")
        (@arg TEXT: ... "The text to apply the ciphers to")
        (@arg encrypt: -e --encrypt "Applies the encryption process to the text")
        (@group ciphers => 
            (@arg caesar: -c --caesar "Uses the caesar cipher, which is a shift cipher of 3")
            (@arg atbash: -a --atbash "Uses the atbash cipher")
            (@arg vigenere: -v --vigenere [key] +takes_value "Uses the vigenere cipher")
            (@arg shift: -s --shift [by] +takes_value +allow_hyphen_values "Shift cipher with custom value")
        )
    )
    .get_matches();

    if let Err(e) = ciphers::run(matches) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}