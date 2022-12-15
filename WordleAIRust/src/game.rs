use crate::{GUESSES_N, WORDS_N};

#[derive(PartialEq)]
pub enum GameState {
    Active,
    Win,
    Lose,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileColor {
    Green,
    Yellow,
    Grey,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct Word {
    pub chars: [char; 5],
    pub letters: LetterContainer,
    pub index: usize,
}

impl Word {
    pub fn new(word: &str, index: usize) -> Word {
        if word.len() != 5 {
            panic!("Word must have length 5.")
        }
        if !word.chars().all(|c| c.is_ascii_alphabetic()) {
            panic!("Word must contain ACII alphabetic characters only.")
        }
        if index > GUESSES_N {
            panic!("Word index out of range.")
        }
        let w = word.to_ascii_lowercase();
        let letter_container = LetterContainer::new(&w);
        let mut chars: [char; 5] = ['a'; 5];
        for (i, char) in word.chars().enumerate(){
            chars[i] = char;
        }
        Word {
            chars,
            letters: letter_container,
            index,
        }
    }

    pub fn has_letter(&self, c: char) -> bool {
        self.letters.contains_char(c)
    }
}


fn get_alphabet_index(c: char) -> usize {
    c as usize - 'a' as usize
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct LetterContainer {
    bool_array: [bool; 26]
}

impl LetterContainer {
    fn new(word: &str) -> LetterContainer {
        let mut bool_array: [bool; 26] = [false; 26];
        for c in word.chars() {
            bool_array[get_alphabet_index(c)] = true
        }
        LetterContainer {
            bool_array,
        }
    }
    fn contains_char(&self, c: char) -> bool {
        self.bool_array[get_alphabet_index(c)]
    }
}

pub fn get_coloring(secret_word: &Word, guess_word: &Word) -> [TileColor; 5] {
    let mut tiles: [TileColor; 5] = [TileColor::Grey; 5];
    for i in 0..5 {
        let secret_c = secret_word.chars[i];
        let guess_c = guess_word.chars[i];
        if secret_c == guess_c {
            tiles[i] = TileColor::Green;
        }
        else if secret_word.has_letter(guess_c) {
            tiles[i] = TileColor::Yellow;
        }
    }
    tiles
}

pub struct Game<'a> {
    pub state: GameState,
    pub attempts: u8,
    hidden_word: &'a Word,
}

impl Game<'_> {
    fn new<'a>(hidden_word: &'a Word) -> Game<'a> {
        Game {
            state: GameState::Active,
            attempts: 0,
            hidden_word,
        }
    }

    pub fn submit_guess(&mut self, guess: &Word) -> [TileColor; 5]{
        self.check_guess(guess);
        get_coloring(self.hidden_word, guess)
    }
    fn check_guess(&mut self, guess: &Word) {
        self.attempts += 1;
        if self.attempts > 6 {
            panic!("more than 6 guesses.");
        }
        if self.state != GameState::Active {
            panic!("game must be active to check a guess.")
        }
        if guess == self.hidden_word {
            self.state = GameState::Win;
        }
        else if self.attempts == 6 {
            self.state = GameState::Lose;
        }
    }
}

#[test]
fn alphabet_maps_to_indices() {
    assert_eq!(get_alphabet_index('a'), 0);
    assert_eq!(get_alphabet_index('z'), 25)
}

#[test]
fn tiles_all_grey() {
    let crank = Word::new("crank", 0);
    let extol = Word::new("extol", 1);
    let tiles = get_coloring(&crank, &extol);
    for tile in tiles {
        assert_eq!(tile, TileColor::Grey)
    }
}

#[test]
fn tiles_all_green() {
    let crank = Word::new("crank",0);
    let also_crank = Word::new("crank", 0);
    let tiles = get_coloring(&crank, &also_crank);
    for tile in tiles {
        assert_eq!(tile, TileColor::Green)
    }
}

#[test]
fn tiles_mixed() {
    let crank = Word::new("crank", 0);
    let caset = Word::new("caset", 1);
    let tiles = get_coloring(&crank, &caset);
    assert_eq!(tiles[0], TileColor::Green);
    assert_eq!(tiles[1], TileColor::Yellow);
    assert_eq!(tiles[2], TileColor::Grey);
    assert_eq!(tiles[3], TileColor::Grey);
    assert_eq!(tiles[4], TileColor::Grey);
}