mod game;

use std::fs;

fn main() {

    let file_path = "/home/luke/WordleAIProject/WordleAI/word-bank.csv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let words: Vec<&str> = contents.split_whitespace().collect();
}
