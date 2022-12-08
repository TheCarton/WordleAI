use std::collections::HashMap;
use crate::game::{get_coloring, TileColor};
use crate::Word;


pub struct Transition<'a> {
    map: HashMap<WordList<'a>, f32>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct WordList<'a> {
    words: Vec<&'a Word>,
}

impl <'a> WordList<'a> {
    fn new() -> WordList<'a> {
        WordList {
            words: Vec::new(),
        }
    }
    fn push(&mut self, word: &'a Word) {
        self.words.push(word);
    }

    fn len(&self) -> usize {
        self.words.len()
    }
}

fn is_valid_solution(action: &Word, tile_coloring: [TileColor; 5], solution: &Word) -> bool {
    for i in 0..5 {
        let action_c = action.string.as_bytes()[i] as char;
        let solution_c = solution.string.as_bytes()[i] as char;
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


pub fn get_transition<'a>(state: &'a WordList, action: &'a Word) -> Transition<'a> {
    let mut map: HashMap<WordList, f32> = HashMap::new();
    for solution in &state.words{
        let tile_coloring = get_coloring(action, &solution);
        let mut s_temp = WordList::new();
        for temp_sol in &state.words{
            if is_valid_solution(action, tile_coloring, temp_sol) {
                s_temp.push(&solution);
            }

        }
        let transition_pr = (1 / s_temp.len()) as f32;
        map.insert(s_temp, transition_pr);
    }
    Transition {
        map,
    }
}
#[test]
fn is_not_possible_solution() {
    let bad_solution = Word::new("uncle");
    let action = Word::new("untie");
    let secret = Word::new("union");

    let coloring = get_coloring(&secret, &action);
    assert!(!is_valid_solution(&action, coloring, &bad_solution))
}

#[test]
fn is_possible_solution() {
    let solution = Word::new("union");

    let action = Word::new("untie");
    let secret = Word::new("union");

    let coloring = get_coloring(&secret, &action);
    assert!(is_valid_solution(&action, coloring, &solution))
}