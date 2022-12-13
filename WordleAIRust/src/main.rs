mod game;
mod bert;

use std::fs;
use crate::game::Word;

const WORDS_N: usize = 2315;

fn main() {

    let file_path = r"C:\Users\Luke\Documents\GitHub\WordleAI\word-bank.csv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let strings: Vec<&str> = contents.split_whitespace().collect();

    let mut words: Vec<Word> = vec![];
    for (i, string) in strings.iter().enumerate(){
        let word = Word::new(string, i);
        words.push(word)
    }
    assert_eq!(WORDS_N, words.len());
}
