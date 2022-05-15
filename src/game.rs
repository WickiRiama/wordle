/// A letter that the player can type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    /// Converts an ASCII character into a [`Letter`] instance.
    ///
    /// If the character is not an ACII letter, [`None`] is returned instead.
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

/// Describes how correct a letter is.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Correctness {
    /// The letter is not in the winning word.
    Incorrect,
    /// The letter exists in the winning word but is not in the right place.
    Misplaced,
	/// The letter is in the right place.
    Correct,
}

/// Stores the current state of the game.
pub struct Game {
    /// A collection of valid words. Words outside of this list won't be accepted
    /// as guesses.
    pub valid_words: Vec<[Letter; Self::WORD_SIZE]>,

    /// The winning word, that the player is trying to guess.
    pub winning_word: [Letter; Self::WORD_SIZE],

    /// The current word, that the player is writing. It is not yet confiremed.
    pub current_word: [Letter; Self::WORD_SIZE],
    /// The number of character written within the `current_word` array.
    pub cursor: usize,

    /// The words that were previously guessed by the player.
    pub previous_words: [[(Letter, Correctness); Self::WORD_SIZE]; Self::MAX_TRIES],
    /// The number of guesses the player tried.
    pub current_try: usize,
	/// The state of each letter
	pub letters_state: [Option<Correctness>; 26],
}

impl Game {
    /// The number of letters in each word.
    pub const WORD_SIZE: usize = 5;
    /// The maximum number of times the player can try a word before the game ends.
    pub const MAX_TRIES: usize = 6;

    /// Creates a new [`Game`] instance.
    ///
    /// A winning word will be choosen from the given word list.
    pub fn new(mut valid_words: Vec<[Letter; Self::WORD_SIZE]>) -> Self {
        if valid_words.len() == 0 {
            panic!("The input word list must contain at least one value.");
        }

        valid_words.sort_unstable();

        let index = unsafe { libc::rand() as usize % valid_words.len() };
        let winning_word = valid_words[index];

        #[cfg(debug_assertions)]
        {
            println!(
                "Winning Word: {:?}{:?}{:?}{:?}{:?}",
                winning_word[0], winning_word[1], winning_word[2], winning_word[3], winning_word[4]
            );
        }

        Self {
            valid_words,
            winning_word,

            current_word: [Letter::A; Self::WORD_SIZE],
            cursor: 0,

            previous_words: [[(Letter::A, Correctness::Incorrect); Self::WORD_SIZE];
                Self::MAX_TRIES],
            current_try: 0,

			letters_state: [None; 26],
        }
    }

    /// Types a new letter for the current game.
    pub fn type_letter(&mut self, letter: Letter) {
        if self.cursor == Self::WORD_SIZE {
            return;
        }

        self.current_word[self.cursor] = letter;
        self.cursor += 1;
    }

    /// Cancels the last typed letter.
    pub fn cancel_letter(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
    }

    /// Tries to confirm the current word.
    pub fn confirm_word(&mut self) {
        // All five letters must have been typed.
        if self.cursor != Self::WORD_SIZE {
            return;
        }

        // Verifies that the word is allowed.
        if self.valid_words.binary_search(&self.current_word).is_err() {
            return;
        }

        // Start by checking chich letters are correct. Every other one are
        // marked as `Incorrect`.
        for i in 0..Self::WORD_SIZE {
            let mut correctness = Correctness::Incorrect;

            if self.current_word[i] == self.winning_word[i] {
                correctness = Correctness::Correct;
            }

            self.previous_words[self.current_try][i] = (self.current_word[i], correctness);
        }

        // This array remembers whether a letter within the winning word has
        // already beem marked as `Misplaced`.
        let mut seen = [false; Self::WORD_SIZE];

        for (letter, correctness) in &mut self.previous_words[self.current_try] {
            // Only incorrect letters can be misplaced.
            if *correctness == Correctness::Correct {
                continue;
            }

            for i in 0..Self::WORD_SIZE {
                if !seen[i] && *letter == self.winning_word[i] {
                    seen[i] = true;
                    *correctness = Correctness::Misplaced;
                }
            }
        }

        // If the winning word is the current word, then the player won.
        if self.winning_word == self.current_word {
            todo!("You won!");
        }

		for (letter, correctness) in self.previous_words[self.current_try]{
			if self.letters_state[letter as usize] < Some(correctness) {
				self.letters_state[letter as usize] = Some(correctness);
			}
		}
		
        self.current_try += 1;
        if self.current_try == Self::MAX_TRIES {
            todo!("You lost!");
        }

        // The user did not find the word but they can try again!
        self.cursor = 0;
    }
}
