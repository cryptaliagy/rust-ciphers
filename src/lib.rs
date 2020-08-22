extern crate clap;
use clap::ArgMatches;

use std::error::Error;
use std::collections::HashMap;
use std::string::FromUtf8Error;

pub fn run(args: ArgMatches) -> Result<(), Box<dyn Error>>{
    let text = args.value_of("TEXT").unwrap_or("").to_ascii_lowercase();

    let mut ciphertext: String = String::from(text);

    if args.is_present("caesar") {
        ciphertext = caesar(&ciphertext, args.is_present("encrypt"))?;
    }
    if args.is_present("atbash") {
        ciphertext = atbash(&ciphertext)?;
    }

    println!("{}", ciphertext);
    Ok(())
}

fn caesar(text: &str, encrypt: bool) -> Result<String, FromUtf8Error> {
    if encrypt {
        shift(text, 3)
    }
    else {
        shift(text, -3)
    }
}

fn shift(text: &str, by: i8) -> Result<String, FromUtf8Error> {
    translate_string(text.as_bytes().iter(), make_shift(by))
}

fn make_shift(by: i8) -> impl Fn(Vec<u8>) -> HashMap<u8, u8> {
    move |alphabet: Vec<u8>| {
        let shift: usize;
        if by < 0 {
            shift = alphabet.len() - by.abs() as usize;
        }
        else {
            shift = by as usize;
        }

        let mut translate = HashMap::new();

        let shifted: Vec<_> = alphabet.iter()
            .skip(shift)
            .chain(alphabet.iter().take(shift))
            .collect();

        for (&original, &translated) in alphabet.iter().zip(shifted) {
            translate.insert(original, translated);
        }

        translate
    }  
}

fn translate_string<'a>(
    text: impl Iterator<Item=&'a u8>, 
    translate_fn: impl Fn(Vec<u8>) -> HashMap<u8, u8>
) -> Result<String, FromUtf8Error> {
    let alphabet: Vec<_> = (b'a'..=b'z').collect();
    
    let translate = translate_fn(alphabet);

    Ok(String::from_utf8(
        text
            .map(|&c| {
                if let Some(&ch) = translate.get(&c) {
                    ch
                } else {
                    c
                }
            }).collect::<Vec<_>>()
    )?)
}

fn atbash(text: &str) -> Result<String, FromUtf8Error> {
    let translation = |alphabet: Vec<u8>| {
        let mut translate = HashMap::new();

        for (&front, &back) in alphabet.iter().zip(alphabet.iter().rev()) {
            translate.insert(front, back);
        }
        translate
    };

    translate_string(text.as_bytes().iter(), translation)
}

fn numeric_decrypt(text: &str) -> Result<String, String> {
    let translation = |alphabet: Vec<u8>| {
        let mut translate = HashMap::new();
        for (index, &letter) in alphabet.iter().enumerate() {
            translate.insert(index as u8, letter);
        }
        translate
    };
    let text = text.split_whitespace()
        .map(|s| s.split('-'))
        .map(|i| {
            i.map(|s| s.parse::<u8>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .map(|i| translate_string(i.iter(), translation)
            .unwrap_or(String::from("ERROR")))
        .collect::<Vec<_>>()
        .join(" ");

    if text.contains("ERROR") {
        Err("Something has gone wrong".to_string())
    }
    else {
        Ok(text)
    }
}
