use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use crate::game::{get_coloring, TileColor};
use crate::{Word, WORDS_N};


pub struct Transition<'a>{
    map: HashMap<WordList<'a>, f32>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct WordList<'a> {
    words: [&'a Word; WORDS_N],
}

impl <'a> WordList<'a> {
    fn new() -> WordList<'a> {
        WordList {
            words: [&Word::new("zonal", 2314); WORDS_N],
        }
    }

    fn len(&self) -> usize {
        self.words.len()
    }
}

impl <'a> Index<usize> for WordList<'a> {
    type Output = &'a Word;

    fn index(&self, index: usize) -> &Self::Output {
        &self.words[index]
    }
}

impl <'a> IndexMut<usize> for WordList<'a>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.words[index]
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


pub fn get_transition<'a>(state: &'a WordList, action: &'a Word) -> Transition<'a> {
    let mut map: HashMap<WordList, f32> = HashMap::new();
    for solution in state.words{
        let tile_coloring = get_coloring(action, &solution);
        let mut s_temp = WordList::new();
        for temp_sol in state.words{
            if is_valid_solution(action, tile_coloring, temp_sol) {
                s_temp[temp_sol.index] = temp_sol;
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

#[test]
fn wordlists_are_same() {
    let union = Word::new("union");
    let sabet = Word::new("sabet");
    let mut list_1 = WordList::new();
    list_1.push(&union);
    list_1.push(&sabet);

    let mut list_2 = WordList::new();
    list_2.push(&sabet);
    list_2.push(&union);

    assert_eq!(list_1, list_2);
}