#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Correctness {
	Correct,
	Misplaced,
	Wrong,
}

pub struct Game {
	pub winning_word: String,

	pub current_word: [Letter; 5],
	pub cursor: usize,

	pub previous_words: [[(Letter, Correctness); 5]; 6],
}
