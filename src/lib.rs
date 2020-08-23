extern crate clap;
use clap::ArgMatches;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::string::FromUtf8Error;

pub fn run(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    let text = match args.values_of("TEXT") {
        Some(v) => v
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<_>>()
            .join(" "),
        None => "".to_string(),
    };

    let mut ciphertext = text;

    if ciphertext.len() == 0 {
        io::stdin().read_to_string(&mut ciphertext)?;
        ciphertext = ciphertext.trim().to_ascii_lowercase().to_string();
    }

    if args.is_present("caesar") {
        ciphertext = caesar(&ciphertext, args.is_present("encrypt"))?;
    } else if args.is_present("atbash") {
        ciphertext = atbash(&ciphertext)?;
    } else if args.is_present("vigenere") {
        let key = &args.value_of("vigenere").unwrap_or("").to_ascii_lowercase();
        ciphertext = vigenere(&ciphertext, key, args.is_present("encrypt"))?;
    } else if args.is_present("shift") {
        let by = args.value_of("shift").unwrap_or("0").parse::<i8>()?;
        ciphertext = shift(&ciphertext, by)?;
    }

    println!("{}", ciphertext);
    Ok(())
}

pub fn caesar(text: &str, encrypt: bool) -> Result<String, FromUtf8Error> {
    if encrypt {
        shift(text, 3)
    } else {
        shift(text, -3)
    }
}

pub fn shift(text: &str, by: i8) -> Result<String, FromUtf8Error> {
    translate_string(text.bytes(), make_shift(by))
}

fn make_shift(by: i8) -> impl Fn(&Vec<u8>) -> HashMap<u8, u8> {
    move |alphabet: &Vec<u8>| {
        let shift: usize;
        if by < 0 {
            shift = alphabet.len() - by.abs() as usize;
        } else {
            shift = by as usize;
        }

        let mut translate = HashMap::new();

        let shifted: Vec<_> = alphabet
            .iter()
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
    text: impl Iterator<Item = u8>,
    translate_fn: impl Fn(&Vec<u8>) -> HashMap<u8, u8>,
) -> Result<String, FromUtf8Error> {
    let alphabet: Vec<_> = (b'a'..=b'z').collect();

    let translate = translate_fn(&alphabet);

    map_translate(text, translate)
}

fn map_translate(
    text: impl Iterator<Item = u8>,
    translate_map: HashMap<u8, u8>,
) -> Result<String, FromUtf8Error> {
    Ok(String::from_utf8(
        text.map(|c| {
            if let Some(&ch) = translate_map.get(&c) {
                ch
            } else {
                c
            }
        })
        .collect::<Vec<_>>(),
    )?)
}

pub fn atbash(text: &str) -> Result<String, FromUtf8Error> {
    let translation = |alphabet: &Vec<u8>| {
        let mut translate = HashMap::new();

        for (&front, &back) in alphabet.iter().zip(alphabet.iter().rev()) {
            translate.insert(front, back);
        }
        translate
    };

    translate_string(text.bytes(), translation)
}

fn numeric_decrypt(text: &str) -> Result<String, String> {
    let translation = |alphabet: &Vec<u8>| {
        let mut translate = HashMap::new();
        for (index, &letter) in alphabet.iter().enumerate() {
            translate.insert(index as u8, letter);
        }
        translate
    };
    let text = text
        .split_whitespace()
        .map(|s| s.split('-'))
        .map(|i| i.map(|s| s.parse::<u8>().unwrap_or(0)))
        .map(|i| translate_string(i, translation).unwrap_or(String::from("ERROR")))
        .collect::<Vec<_>>()
        .join(" ");

    if text.contains("ERROR") {
        Err("Something has gone wrong".to_string())
    } else {
        Ok(text)
    }
}

pub fn vigenere(text: &str, key: &str, enc: bool) -> Result<String, FromUtf8Error> {
    let alphabet: Vec<_> = (b'a'..=b'z').collect();

    let mut reverse_alphabet = HashMap::new();
    let mut shift_dicts = HashMap::new();
    for (index, &letter) in alphabet.iter().enumerate() {
        reverse_alphabet.insert(letter, index);
    }

    let mut key = key.bytes().cycle();

    let result: Vec<_> = text
        .bytes()
        .map(|letter| {
            // Return character if it's not in the alphabet so we don't consume a key index
            if let None = reverse_alphabet.get(&letter) {
                return letter;
            }

            let shift = key.next().unwrap(); // Unwrapping is safe since cycle is infinite
            let shift = *reverse_alphabet.get(&shift).unwrap();
            let mut shift = shift as i8;
            if !enc {
                shift = -shift;
            }

            // Get the translation dictionary for this key index
            let shift_dict = match shift_dicts.get(&shift) {
                Some(d) => d,
                None => match shift_dicts.insert(shift, make_shift(shift as i8)(&alphabet)) {
                    _ => shift_dicts.get(&shift).unwrap(),
                },
            };

            if let Some(&c) = shift_dict.get(&letter) {
                c
            } else {
                letter
            }
        })
        .collect();

    Ok(String::from_utf8(result)?)
}
