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

#[derive(Eq, Hash, PartialEq, Debug)]
struct StateTurn {
    state: WordContainer,
    turn: usize,
}

struct Wordle {
    valid_solutions: [Word; WORDS_N],
    valid_guesses: [Word; WORDS_N],
    v_mem: HashMap<StateTurn, f32>,
}

impl Wordle {
    fn new(valid_solutions: [Word; WORDS_N], valid_guesses: [Word; WORDS_N]) -> Wordle {
        Wordle {
            valid_solutions,
            valid_guesses,
            v_mem: HashMap::new(),
        }
    }
    fn get_transition(&self, state: &WordContainer, action: &Word) -> Transition {
        let mut map: HashMap<WordContainer, f32> = HashMap::new();
        for solution_index in &state.word_indices {
            let solution = self.valid_solutions[*solution_index];
            let tile_coloring = get_coloring(action, &solution);
            let mut s_temp = WordContainer::new();
            for temp_sol_index in &state.word_indices {
                let temp_sol = self.valid_solutions[*temp_sol_index];
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

    fn get_state_value(&mut self, state_turn: StateTurn, v_mem: Transition) -> f32 {
        let t = state_turn.turn;
        let state = &state_turn.state;
        if t == 6 || (t == 5 && state.len() > 1) {
            return f32::INFINITY;
        }
        else if t == 5 {
            return 1f32;
        }
        else if state.len() == 1 {
            return 1f32;
        }
        else if state.len() == 2 {
            return 1.5
        }
        else if let Some(v) = self.v_mem.get(&state_turn) {
            return *v;
        }
        0f32
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

#[test]
fn word_containers_are_equal() {
    let action = Word::new("untie", 1);
    let mut container_1 = WordContainer::new();
    container_1.add_word(&action);
    let mut container_2 = WordContainer::new();
    container_2.add_word(&action);
    assert_eq!(container_1, container_2);
    let secret = Word::new("union", 0);
    container_1.add_word(&secret);
    container_2.add_word(&secret);
    assert_eq!(container_1, container_2);
}

#[test]
fn word_containers_are_not_equal() {
    let action = Word::new("untie", 1);
    let secret = Word::new("union", 0);

    let mut container_1 = WordContainer::new();
    container_1.add_word(&action);
    let mut container_2 = WordContainer::new();
    container_2.add_word(&secret);

    let container_3 = WordContainer::new();
    assert_ne!(container_1, container_2);
    assert_ne!(container_1, container_3)
}