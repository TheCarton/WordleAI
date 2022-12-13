use std::collections::HashMap;
use crate::game::{get_coloring, TileColor};
use crate::{Word, WORDS_N};


pub struct Transition{
    map: HashMap<WordContainer, f32>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct WordContainer {
    bool_array: [bool; WORDS_N],
    word_indices: Vec<usize>,
}

impl WordContainer {
    fn new() -> WordContainer{
        WordContainer {
            bool_array: [false; WORDS_N],
            word_indices: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.word_indices.len()
    }

    fn add_word(&mut self, word: &Word){
        self.bool_array[word.index] = true;
        self.word_indices.push(word.index);
    }

    fn has_word(&self, word: &Word) -> bool{
        self.bool_array[word.index]
    }
}

impl Iterator for WordContainer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn is_valid_solution(action: &Word, tile_coloring: [TileColor; 5], solution: &Word) -> bool {
    for i in 0..5 {
        let action_c = action.chars[i];
        let solution_c = solution.chars[i];
        let color = tile_coloring[i];
        if color == TileColor::Green && !(action_c == solution_c) {
            return false;
        }
        if color == TileColor::Yellow && !solution.has_letter(action_c) {
            return false;
        }
    }
    return true;
}

struct Wordle {
    words: [Word; WORDS_N]
}

impl Wordle {
    fn get_transition<'a>(&self, state: &'a WordContainer, action: &'a Word) -> Transition {
        let mut map: HashMap<WordContainer, f32> = HashMap::new();
        for solution_index in &state.word_indices {
            let solution = self.words[*solution_index];
            let tile_coloring = get_coloring(action, &solution);
            let mut s_temp = WordContainer::new();
            for temp_sol_index in &state.word_indices {
                let temp_sol = self.words[*temp_sol_index];
                if is_valid_solution(action, tile_coloring, &temp_sol) {
                    s_temp.add_word(&temp_sol);
                }
            }
            let transition_pr = (1 / state.len()) as f32;
            if let Some(pr) = map.get_mut(&s_temp) {
                *pr += transition_pr;
            } else {
                map.insert(s_temp, transition_pr);
            }
        }
        Transition {
            map,
        }
    }
}



#[test]
fn is_not_possible_solution() {
    let bad_solution = Word::new("uncle", 0);
    let action = Word::new("untie", 1);
    let secret = Word::new("union", 2);

    let coloring = get_coloring(&secret, &action);
    assert!(!is_valid_solution(&action, coloring, &bad_solution))
}

#[test]
fn is_possible_solution() {
    let solution = Word::new("union", 0);

    let action = Word::new("untie", 1);
    let secret = Word::new("union", 0);

    let coloring = get_coloring(&secret, &action);
    assert!(is_valid_solution(&action, coloring, &solution))
}