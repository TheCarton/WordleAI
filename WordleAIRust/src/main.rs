mod game;
mod bert;

use std::fs;
use crate::game::Word;

fn main() {

    let file_path = "/home/luke/WordleAIProject/WordleAI/word-bank.csv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let strings: Vec<&str> = contents.split_whitespace().collect();

    let mut words: Vec<Word> = vec![];
    for string in strings{
        let word = Word::new(string);
        words.push(word)
    }
}
