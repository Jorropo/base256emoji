#![recursion_limit = "10000"]
use std::fmt;

use heapless::{FnvIndexMap, IndexMap};
use once_cell::sync::OnceCell;
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
	// should be moved to std oncecell at some point
	const LOOKUP: OnceCell<FnvIndexMap<char, usize, 256>> = OnceCell::new();

	fn get_index(c: char) -> usize;

	fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
		let s = input.chars().count();
		let mut output = vec![0; s];
		for (i, c) in input.chars().enumerate() {
			let lookup = Self::LOOKUP.get_or_init(|| IndexMap::new());
			output[i] = match Self::ALPHABET.iter().position(|&x| x == c) {
				Some(c) => c as u8,
				None => {
					return Err(DecodeError {
						codepoint: c,
						index: i,
					});
				}
			};
		}

		Ok(output)
	}

	fn encode(input: &Vec<u8>) -> String {
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

// const fn gen_lookup<const S: usize>(input: [char; S]) -> heapless::FnvIndexMap<char, usize, S> {
// 	let mut lookup = FnvIndexMap::new();
// 	// cant use for loop in const!!
// 	// for (i, &c) in input.iter().enumerate() {
// 	// 	a.insert(c, i);
// 	// }
// 	let mut i = 0;
// 	while i < S {
// 		lookup.insert(input[i], i);
// 		i += 1;
// 	}
// 	lookup
// }


macro_rules! gen_alphabet {
	($name:ident, $first:literal, $($other:literal),*) => {
		impl Base for $name {
			const ALPHABET: [char; 256] = [$first, $($other,)*];

			fn get_index(c: char) -> usize {
				if matches!(c, $first) {
					0
				} else {
					gen_alphabet!(1, c, $($other),*)
				}
			}
		}

	};
	($i:expr, $c:ident, $first:literal) => {
		if matches!($c, $first) {
			$i
		} else {
			unreachable!()
		}

	};
	($i:expr, $c:ident, $first:literal, $($other:literal),*) => {
		if matches!($c, $first) {
			$i
		} else {
			gen_alphabet!($i+1, $c, $($other),*)
		}

	};
}

gen_alphabet!(
	Emoji,
	'🚀',
	'🪐',
	'☄',
	'🛰',
	'🌌',
	'🌑',
	'🌒',
	'🌓',
	'🌔',
	'🌕',
	'🌖',
	'🌗',
	'🌘',
	'🌍',
	'🌏',
	'🌎',
	'🐉',
	'☀',
	'💻',
	'🖥',
	'💾',
	'💿',
	'😂',
	'❤',
	'😍',
	'🤣',
	'😊',
	'🙏',
	'💕',
	'😭',
	'😘',
	'👍',
	'😅',
	'👏',
	'😁',
	'🔥',
	'🥰',
	'💔',
	'💖',
	'💙',
	'😢',
	'🤔',
	'😆',
	'🙄',
	'💪',
	'😉',
	'☺',
	'👌',
	'🤗',
	'💜',
	'😔',
	'😎',
	'😇',
	'🌹',
	'🤦',
	'🎉',
	'💞',
	'✌',
	'✨',
	'🤷',
	'😱',
	'😌',
	'🌸',
	'🙌',
	'😋',
	'💗',
	'💚',
	'😏',
	'💛',
	'🙂',
	'💓',
	'🤩',
	'😄',
	'😀',
	'🖤',
	'😃',
	'💯',
	'🙈',
	'👇',
	'🎶',
	'😒',
	'🤭',
	'❣',
	'😜',
	'💋',
	'👀',
	'😪',
	'😑',
	'💥',
	'🙋',
	'😞',
	'😩',
	'😡',
	'🤪',
	'👊',
	'🥳',
	'😥',
	'🤤',
	'👉',
	'💃',
	'😳',
	'✋',
	'😚',
	'😝',
	'😴',
	'🌟',
	'😬',
	'🙃',
	'🍀',
	'🌷',
	'😻',
	'😓',
	'⭐',
	'✅',
	'🥺',
	'🌈',
	'😈',
	'🤘',
	'💦',
	'✔',
	'😣',
	'🏃',
	'💐',
	'☹',
	'🎊',
	'💘',
	'😠',
	'☝',
	'😕',
	'🌺',
	'🎂',
	'🌻',
	'😐',
	'🖕',
	'💝',
	'🙊',
	'😹',
	'🗣',
	'💫',
	'💀',
	'👑',
	'🎵',
	'🤞',
	'😛',
	'🔴',
	'😤',
	'🌼',
	'😫',
	'⚽',
	'🤙',
	'☕',
	'🏆',
	'🤫',
	'👈',
	'😮',
	'🙆',
	'🍻',
	'🍃',
	'🐶',
	'💁',
	'😲',
	'🌿',
	'🧡',
	'🎁',
	'⚡',
	'🌞',
	'🎈',
	'❌',
	'✊',
	'👋',
	'😰',
	'🤨',
	'😶',
	'🤝',
	'🚶',
	'💰',
	'🍓',
	'💢',
	'🤟',
	'🙁',
	'🚨',
	'💨',
	'🤬',
	'✈',
	'🎀',
	'🍺',
	'🤓',
	'😙',
	'💟',
	'🌱',
	'😖',
	'👶',
	'🥴',
	'▶',
	'➡',
	'❓',
	'💎',
	'💸',
	'⬇',
	'😨',
	'🌚',
	'🦋',
	'😷',
	'🕺',
	'⚠',
	'🙅',
	'😟',
	'😵',
	'👎',
	'🤲',
	'🤠',
	'🤧',
	'📌',
	'🔵',
	'💅',
	'🧐',
	'🐾',
	'🍒',
	'😗',
	'🤑',
	'🌊',
	'🤯',
	'🐷',
	'☎',
	'💧',
	'😯',
	'💆',
	'👆',
	'🎤',
	'🙇',
	'🍑',
	'❄',
	'🌴',
	'💣',
	'🐸',
	'💌',
	'📍',
	'🥀',
	'🤢',
	'👅',
	'💡',
	'💩',
	'👐',
	'📸',
	'👻',
	'🤐',
	'🤮',
	'🎼',
	'🥵',
	'🚩',
	'🍎',
	'🍊',
	'👼',
	'💍',
	'📣',
	'🥂'
);

// impl Base for Emoji {
// 	const ALPHABET: Alphabet = [
// 		'🚀', '🪐', '☄', '🛰', '🌌', '🌑', '🌒', '🌓', '🌔', '🌕', '🌖', '🌗', '🌘', '🌍', '🌏',
// 		'🌎', '🐉', '☀', '💻', '🖥', '💾', '💿', '😂', '❤', '😍', '🤣', '😊', '🙏', '💕', '😭',
// 		'😘', '👍', '😅', '👏', '😁', '🔥', '🥰', '💔', '💖', '💙', '😢', '🤔', '😆', '🙄', '💪',
// 		'😉', '☺', '👌', '🤗', '💜', '😔', '😎', '😇', '🌹', '🤦', '🎉', '💞', '✌', '✨', '🤷',
// 		'😱', '😌', '🌸', '🙌', '😋', '💗', '💚', '😏', '💛', '🙂', '💓', '🤩', '😄', '😀', '🖤',
// 		'😃', '💯', '🙈', '👇', '🎶', '😒', '🤭', '❣', '😜', '💋', '👀', '😪', '😑', '💥', '🙋',
// 		'😞', '😩', '😡', '🤪', '👊', '🥳', '😥', '🤤', '👉', '💃', '😳', '✋', '😚', '😝', '😴',
// 		'🌟', '😬', '🙃', '🍀', '🌷', '😻', '😓', '⭐', '✅', '🥺', '🌈', '😈', '🤘', '💦', '✔',
// 		'😣', '🏃', '💐', '☹', '🎊', '💘', '😠', '☝', '😕', '🌺', '🎂', '🌻', '😐', '🖕', '💝',
// 		'🙊', '😹', '🗣', '💫', '💀', '👑', '🎵', '🤞', '😛', '🔴', '😤', '🌼', '😫', '⚽', '🤙',
// 		'☕', '🏆', '🤫', '👈', '😮', '🙆', '🍻', '🍃', '🐶', '💁', '😲', '🌿', '🧡', '🎁', '⚡',
// 		'🌞', '🎈', '❌', '✊', '👋', '😰', '🤨', '😶', '🤝', '🚶', '💰', '🍓', '💢', '🤟', '🙁',
// 		'🚨', '💨', '🤬', '✈', '🎀', '🍺', '🤓', '😙', '💟', '🌱', '😖', '👶', '🥴', '▶', '➡',
// 		'❓', '💎', '💸', '⬇', '😨', '🌚', '🦋', '😷', '🕺', '⚠', '🙅', '😟', '😵', '👎', '🤲',
// 		'🤠', '🤧', '📌', '🔵', '💅', '🧐', '🐾', '🍒', '😗', '🤑', '🌊', '🤯', '🐷', '☎', '💧',
// 		'😯', '💆', '👆', '🎤', '🙇', '🍑', '❄', '🌴', '💣', '🐸', '💌', '📍', '🥀', '🤢', '👅',
// 		'💡', '💩', '👐', '📸', '👻', '🤐', '🤮', '🎼', '🥵', '🚩', '🍎', '🍊', '👼', '💍', '📣',
// 		'🥂',
// 	];
// }

#[cfg(test)]
mod tests {
	use crate::Base;
	use crate::Emoji;

	#[test]
	fn byte1_rt() {
		let mut org = vec![0u8; 1];
		for i in 0..255 {
			org[0] = i;
			let r = match Emoji::decode(Emoji::encode(&org).as_str()) {
				Ok(x) => x,
				Err(e) => {
					panic!("{}", e);
				}
			};
			assert_eq!(org, r)
		}
	}

	#[test]
	fn b() {
		println!("{}", Emoji::ALPHABET.iter().collect::<String>())
	}
}
