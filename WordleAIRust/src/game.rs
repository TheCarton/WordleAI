
#[derive(PartialEq)]
enum GameState {
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

#[derive(Eq, Hash, PartialEq)]
pub struct Word {
    pub string: String,
    pub letters: LetterContainer,
}

impl Word {
    pub fn new(word: &str) -> Word {
        if word.len() != 5 {
            panic!("Word must have length 5.")
        }
        if !word.chars().all(|c| c.is_ascii_alphabetic()) {
            panic!("Word must contain ACII alphabetic characters only.")
        }
        let w = word.to_ascii_lowercase();
        let letter_container = LetterContainer::new(&w);
        Word {
            string: w,
            letters: letter_container,
        }
    }

    pub fn has_letter(&self, c: char) -> bool {
        self.letters.contains_char(c)
    }
}


fn get_alphabet_index(c: char) -> usize {
    c as usize - 'a' as usize
}

#[derive(Eq, Hash, PartialEq)]
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
        let secret_c = secret_word.string.as_bytes()[i] as char;
        let guess_c = guess_word.string.as_bytes()[i] as char;
        if secret_c == guess_c {
            tiles[i] = TileColor::Green;
        }
        else if secret_word.has_letter(guess_c) {
            tiles[i] = TileColor::Yellow;
        }
    }
    tiles
}

struct Game<'a> {
    pub state: GameState,
    pub attempts: u8,
    pub words: Vec<&'a str>,
    hidden_word: &'a str,
}

impl Game<'_>{
    fn new<'a>(hidden_word: &'a str, words: Vec<&'a str>) -> Game<'a> {
        if hidden_word.chars().count() != 5 {
            panic!("hidden word must be 5 characters.")
        }
        if !words.contains(&hidden_word) {
            panic!("words vector must contain hidden word.")
        }
        Game {
            state: GameState::Active,
            attempts: 0,
            words,
            hidden_word,
        }
    }
    fn check_guess(&mut self, guess: &str){
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
fn is_active() {
    let test_words = vec!["aahed", "aalii", "aargh", "aarti", "abaca", "abaci", "aback", "penis"];
    let mut game = Game::new("aback", test_words);
    assert!(game.state == GameState::Active);
    game.check_guess("aalii");
    assert!(game.state == GameState::Active);
}

#[test]
fn can_win() {
    let test_words = vec!["aahed", "aalii", "aargh", "aarti", "abaca", "abaci", "aback", "penis"];
    let mut game = Game::new("aback", test_words);
    game.check_guess("aback");
    assert!(game.state == GameState::Win);
}

#[test]
fn can_lose() {
    let test_words = vec!["aahed", "aalii", "aargh", "aarti", "abaca", "abaci", "aback", "penis"];
    let mut game = Game::new("aback", test_words);
    game.check_guess("aahed");
    game.check_guess("aalii");
    game.check_guess("aargh");
    game.check_guess("aarti");
    game.check_guess("abaca");
    game.check_guess("abaci");
    assert!(game.state == GameState::Lose);
}

#[test]
#[should_panic]
fn wrong_word_length() {
    let test_words = vec!["aahed", "aalii", "aargh", "aarti", "abaca", "abaci", "aback", "penis"];
    let game = Game::new("expedite", test_words);
}

#[test]
#[should_panic]
fn hidden_word_not_in_words() {
    let test_words = vec!["aahed", "aalii", "aargh", "aarti", "abaca", "abaci", "aback", "penis"];
    let game = Game::new("salet", test_words);
}

#[test]
#[should_panic]
fn too_many_guesses(){
    let test_words = vec!["aahed", "aalii", "aargh", "aarti", "abaca", "abaci", "aback", "penis"];
    let mut game = Game::new("aback", test_words);
    game.check_guess("aalii");
    game.check_guess("aargh");
    game.check_guess("aarti");
    game.check_guess("abaca");
    game.check_guess("abaci");
    game.check_guess("abacs");
    game.check_guess("penis");
}

#[test]
#[should_panic]
fn guess_after_win() {
    let test_words = vec!["aahed", "aalii", "aargh", "aarti", "abaca", "abaci", "aback", "penis"];
    let mut game = Game::new("aback", test_words);
    game.check_guess("aback");
    game.check_guess("aalii");
}

#[test]
fn alphabet_maps_to_indices() {
    assert_eq!(get_alphabet_index('a'), 0);
    assert_eq!(get_alphabet_index('z'), 25)
}

#[test]
fn tiles_all_grey() {
    let crank = Word::new("crank");
    let extol = Word::new("extol");
    let tiles = get_coloring(&crank, &extol);
    for tile in tiles {
        assert_eq!(tile, TileColor::Grey)
    }
}

#[test]
fn tiles_all_green() {
    let crank = Word::new("crank");
    let also_crank = Word::new("crank");
    let tiles = get_coloring(&crank, &also_crank);
    for tile in tiles {
        assert_eq!(tile, TileColor::Green)
    }
}

#[test]
fn tiles_mixed() {
    let crank = Word::new("crank");
    let caset = Word::new("caset");
    let tiles = get_coloring(&crank, &caset);
    assert_eq!(tiles[0], TileColor::Green);
    assert_eq!(tiles[1], TileColor::Yellow);
    assert_eq!(tiles[2], TileColor::Grey);
    assert_eq!(tiles[3], TileColor::Grey);
    assert_eq!(tiles[4], TileColor::Grey);
}