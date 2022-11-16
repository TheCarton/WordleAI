
#[derive(PartialEq)]
enum GameState {
    Active,
    Win,
    Lose,
}

#[derive(Copy, Clone)]
enum TileColor {
    Green,
    Yellow,
    Grey,
}

struct Word {
    pub string: String,
    pub letter_container: LetterContainer,
}

impl Word {
    fn new(word: &str) -> Word {
        if word.len() != 5 {
            panic!("Word must have length 5.")
        }
        if !word.is_ascii() {
            panic!("Word must contain ACII characters only.")
        }
        let w = word.to_ascii_lowercase();
        let letter_container = LetterContainer::new(&w);
        Word {
            string: w,
            letter_container,
        }
    }
}

struct LetterContainer {
    pub bool_array: [bool; 26]
}

fn map_char_to_index(c: char) -> usize {
    match c {
        'a' => return 0,
        'b' => return 1,
        'c' => return 2,
        'd' => return 3,
        'e' => return 4,
        'f' => return 5,
        'g' => return 6,
        'h' => return 7,
        'i' => return 8,
        'j' => return 9,
        'k' => return 10,
        'l' => return 11,
        'm' => return 12,
        'n' => return 13,
        'o' => return 14,
        'p' => return 15,
        'q' => return 16,
        'r' => return 17,
        's' => return 18,
        't' => return 19,
        'u' => return 20,
        'v' => return 21,
        'w' => return 22,
        'x' => return 23,
        'y' => return 24,
        'z' => return 25,
        _ => {
            panic!("Characters must be lowercase ASCII.")}
    }
}

impl LetterContainer {
    fn new(word: &str) -> LetterContainer {
        let mut bool_array: [bool; 26] = [false; 26];
        for c in word.chars() {
            bool_array[map_char_to_index(c)] = true
        }
        LetterContainer {
            bool_array,
        }
    }
}

fn get_coloring(word: Word, hidden_word: Word) -> [TileColor; 5] {
    let mut tiles: [TileColor; 5] = [TileColor::Grey; 5];
    for i in 0..4 {
        if word.string.as_bytes()[i] == hidden_word.string.as_bytes()[i] {
            tiles[i] = TileColor::Green;
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