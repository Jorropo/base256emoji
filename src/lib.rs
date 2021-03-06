use std::fmt;

#[derive(Debug, Clone)]
pub struct DecodeError {
	codepoint: char,
	index: usize,
}

impl fmt::Display for DecodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{} at index {} is not part of the alphabet",
			self.codepoint, self.index
		)
	}
}

pub type Alphabet = [char; 256];

pub trait Base {
	const ALPHABET: Alphabet;

	fn get_index(c: char) -> Option<u8>;

	fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
		let s = input.chars().count();
		let mut output = vec![0; s];
		for (i, c) in input.chars().enumerate() {
			output[i] = Self::get_index(c).ok_or(DecodeError {
				codepoint: c,
				index: i,
			})?;
		}

		Ok(output)
	}

	fn encode(input: &[u8]) -> String {
		let s = input
			.iter()
			.map(|&x| Self::ALPHABET[x as usize].len_utf8())
			.sum();
		let mut output: Vec<u8> = vec![0; s];
		let mut i = 0;
		for &v in input.iter() {
			let c = Self::ALPHABET[v as usize];
			c.encode_utf8(&mut output[i..]);
			i += c.len_utf8();
		}

		String::from_utf8(output).unwrap()
	}
}

#[derive(Debug, Default)]
pub struct Emoji;

macro_rules! gen_alphabet {
	($name:ident, $alphabet:literal) => {
		impl Base for $name {
			const ALPHABET: Alphabet = const_str::to_char_array!($alphabet);

			fn get_index(c: char) -> Option<u8> {
				match_lookup::gen_char_match!(c, $alphabet).map(|c| c as u8)
			}
		}
	};
}

gen_alphabet!(Emoji, "๐๐ชโ๐ฐ๐๐๐๐๐๐๐๐๐๐๐๐๐โ๐ป๐ฅ๐พ๐ฟ๐โค๐๐คฃ๐๐๐๐ญ๐๐๐๐๐๐ฅ๐ฅฐ๐๐๐๐ข๐ค๐๐๐ช๐โบ๐๐ค๐๐๐๐๐น๐คฆ๐๐โโจ๐คท๐ฑ๐๐ธ๐๐๐๐๐๐๐๐๐คฉ๐๐๐ค๐๐ฏ๐๐๐ถ๐๐คญโฃ๐๐๐๐ช๐๐ฅ๐๐๐ฉ๐ก๐คช๐๐ฅณ๐ฅ๐คค๐๐๐ณโ๐๐๐ด๐๐ฌ๐๐๐ท๐ป๐โญโ๐ฅบ๐๐๐ค๐ฆโ๐ฃ๐๐โน๐๐๐ โ๐๐บ๐๐ป๐๐๐๐๐น๐ฃ๐ซ๐๐๐ต๐ค๐๐ด๐ค๐ผ๐ซโฝ๐คโ๐๐คซ๐๐ฎ๐๐ป๐๐ถ๐๐ฒ๐ฟ๐งก๐โก๐๐โโ๐๐ฐ๐คจ๐ถ๐ค๐ถ๐ฐ๐๐ข๐ค๐๐จ๐จ๐คฌโ๐๐บ๐ค๐๐๐ฑ๐๐ถ๐ฅดโถโกโ๐๐ธโฌ๐จ๐๐ฆ๐ท๐บโ ๐๐๐ต๐๐คฒ๐ค ๐คง๐๐ต๐๐ง๐พ๐๐๐ค๐๐คฏ๐ทโ๐ง๐ฏ๐๐๐ค๐๐โ๐ด๐ฃ๐ธ๐๐๐ฅ๐คข๐๐ก๐ฉ๐๐ธ๐ป๐ค๐คฎ๐ผ๐ฅต๐ฉ๐๐๐ผ๐๐ฃ๐ฅ");

#[cfg(test)]
mod tests {
	use crate::{Base, Emoji};

	#[test]
	fn byte1_rt() {
		let mut org = vec![0u8; 1];
		for i in 0..255 {
			org[0] = i;
			let e = Emoji::encode(&org);
			let r = match Emoji::decode(e.as_str()) {
				Ok(x) => x,
				Err(e) => {
					panic!("{}", e);
				}
			};
			assert_eq!(org, r)
		}
	}

	#[test]
	fn index() {
		for (i, &c) in Emoji::ALPHABET.iter().enumerate() {
			println!("i{}:{}, {:?}", i, c, Emoji::get_index(c));
			assert!(i as u8 == Emoji::get_index(c).unwrap());
		}
	}

	#[test]
	fn juan() {
		let hi = "hi juan!";
		let encoded = Emoji::encode(&hi.as_bytes().to_owned());
		assert_eq!("๐ด๐๐๐ฌ๐ค๐คค๐ป๐", encoded);
		println!("{}: {}", hi, encoded);
	}

	#[test]
	fn reference() {
		let hi = "yes mani !";
		let encoded = Emoji::encode(&hi.as_bytes().to_owned());
		assert_eq!("๐โ๐๐๐ท๐คค๐ป๐๐๐", encoded);
		println!("{}: {}", hi, encoded);
	}
}
