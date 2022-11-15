#[derive(PartialEq)]
enum GameState {
    Active,
    Win,
    Lose,
}

struct Game<'a> {
    pub state: GameState,
    pub attempts: u8,
    hidden_word: &'a str,
}

impl Game<'_>{
    fn new(hidden_word: &str) -> Game {
        if hidden_word.chars().count() != 5 {
            panic!("hidden word must be 5 characters.")
        }
        Game {
            state: GameState::Active,
            attempts: 0,
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
    let mut game = Game::new("aback");
    assert!(game.state == GameState::Active);
    game.check_guess("aalii");
    assert!(game.state == GameState::Active);
}

#[test]
fn can_win() {
    let mut game = Game::new("aback");
    game.check_guess("aback");
    assert!(game.state == GameState::Win);
}

#[test]
fn can_lose() {
    let mut game = Game::new("aback");
    game.check_guess("aalii");
    game.check_guess("aargh");
    game.check_guess("aarti");
    game.check_guess("abaca");
    game.check_guess("abaci");
    game.check_guess("abacs");
    assert!(game.state == GameState::Lose);
}

#[test]
#[should_panic]
fn wrong_word_length() {
    let game = Game::new("expedite");
}

#[test]
#[should_panic]
fn too_many_guesses(){
    let mut game = Game::new("aback");
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
    let mut game = Game::new("aback");
    game.check_guess("aback");
    game.check_guess("aalii");
}