mod game;
mod bert;

use std::fs;
use crate::game::Word;

const WORDS_N: usize = 2315;
const GUESSES_N: usize = 12972;

fn main() {

    let file_path_sols = r"C:\Users\Luke\Documents\GitHub\WordleAI\word-bank.csv";

    let contents_sols = fs::read_to_string(file_path_sols)
        .expect("Should have been able to read the file");

    let strings_sols: Vec<&str> = contents_sols.split_whitespace().collect();

    let mut solutions: Vec<Word> = vec![];
    for (i, string) in strings_sols.iter().enumerate(){
        let word = Word::new(string, i);
        solutions.push(word)
    }

    let file_path_guesses = r"C:\Users\Luke\Documents\GitHub\WordleAI\valid-words.csv";
    let contents_guesses = fs::read_to_string(file_path_guesses)
        .expect("Should have been able to read the file");


    let strings_guesses: Vec<&str> = contents_guesses.split_whitespace().collect();
    let mut guesses: Vec<Word> = vec![];

    for (i, string) in strings_guesses.iter().enumerate(){
        let word = Word::new(string, i);
        guesses.push(word);
    }


    assert_eq!(WORDS_N, solutions.len());
    assert_eq!(GUESSES_N, guesses.len());
}
