#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Letter {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
}

impl Letter {
	pub fn from_ascii_char(c: u8) -> Option<Self> {
		match c.to_ascii_uppercase() {
			b'A' => Some(Self::A),
			b'B' => Some(Self::B),
			b'C' => Some(Self::C),
			b'D' => Some(Self::D),
			b'E' => Some(Self::E),
			b'F' => Some(Self::F),
			b'G' => Some(Self::G),
			b'H' => Some(Self::H),
			b'I' => Some(Self::I),
			b'J' => Some(Self::J),
			b'K' => Some(Self::K),
			b'L' => Some(Self::L),
			b'M' => Some(Self::M),
			b'N' => Some(Self::N),
			b'O' => Some(Self::O),
			b'P' => Some(Self::P),
			b'Q' => Some(Self::Q),
			b'R' => Some(Self::R),
			b'S' => Some(Self::S),
			b'T' => Some(Self::T),
			b'U' => Some(Self::U),
			b'V' => Some(Self::V),
			b'W' => Some(Self::W),
			b'X' => Some(Self::X),
			b'Y' => Some(Self::Y),
			b'Z' => Some(Self::Z),
			_ => None,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Correctness {
	Correct,
	Misplaced,
	Incorrect,
}

pub struct Game {
	pub valid_words: Vec<[Letter; Self::WORD_SIZE]>,

	pub winning_word: [Letter; Self::WORD_SIZE],

	pub current_word: [Letter; Self::WORD_SIZE],
	pub cursor: usize,

	pub previous_words: [[(Letter, Correctness); Self::WORD_SIZE]; Self::MAX_TRIES],
	pub current_try: usize,
}

impl Game {
	pub const WORD_SIZE: usize = 5;
	pub const MAX_TRIES: usize = 6;

	pub fn new(winning_word: [Letter; Self::WORD_SIZE], mut valid_words: Vec<[Letter; Self::WORD_SIZE]>) -> Self {
		valid_words.sort_unstable();
		
		Self {
			valid_words,
			winning_word,

			current_word: [Letter::A; Self::WORD_SIZE],
			cursor: 0,

			previous_words: [[(Letter::A, Correctness::Incorrect); Self::WORD_SIZE]; Self::MAX_TRIES],
			current_try: 0,
		}
	}

	pub fn type_letter(&mut self, letter: Letter) {
		if self.cursor == Self::WORD_SIZE {
			return;
		}
	
		self.current_word[self.cursor] = letter;
		self.cursor += 1;
	}
	
	pub fn cancel_letter(&mut self) {
		if self.cursor != 0 {
			self.cursor -= 1;
		}
	}

	pub fn confirm_word(&mut self) {
		if self.cursor != Self::WORD_SIZE {
			return;
		}

		if self.valid_words.binary_search(&self.current_word).is_err() {
			return;
		}

		for i in 0..Self::WORD_SIZE {
			let mut correctness = Correctness::Incorrect;

			if self.current_word[i] == self.winning_word[i] {
				correctness = Correctness::Correct;
			}

			self.previous_words[self.current_try][i] = (self.current_word[i], correctness);
		}

		for current_letter in self.winning_word {
			let mut count = 0;
			for (letter, correctness) in self.previous_words[self.current_try] {
				if letter == current_letter && correctness == Correctness::Incorrect {
					count += 1;
				}
			}

			for (letter, correctness) in &mut self.previous_words[self.current_try] {
				if *letter == current_letter {
					*correctness = Correctness::Misplaced;

					count -= 1;
					if count == 0 {
						break;
					}
				}
			}
		}

		if self.winning_word == self.current_word {
			todo!("You won!");
		}
		
		self.current_try += 1;
		if self.current_try == Self::MAX_TRIES {
			todo!("You lost!");
		}

		self.cursor = 0;
	}
}
