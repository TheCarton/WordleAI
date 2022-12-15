use std::collections::HashMap;
use crate::game::{Game, get_coloring, TileColor};
use crate::{GUESSES_N, Word, WORDS_N};


pub struct Transition{
    map: HashMap<WordContainer, f32>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct WordContainer {
    bool_array: [bool; GUESSES_N],
    word_indices: Vec<usize>,
}

impl WordContainer {
    fn new() -> WordContainer{
        WordContainer {
            bool_array: [false; GUESSES_N],
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

    fn disjoint(&self, other: &WordContainer) -> WordContainer {
        let mut result_array = self.bool_array.clone();
        for index in &other.word_indices {
            result_array[*index] = false;
        }
        WordContainer {
            bool_array: result_array,
            word_indices: Vec::new(),
        }
    }

    fn union(&self, other: &WordContainer) -> WordContainer {
        let mut result_array = self.bool_array.clone();
        for index in &other.word_indices {
            result_array[*index] = true;
        }
        WordContainer {
            bool_array: result_array,
            word_indices: Vec::new(),
        }
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

pub struct Wordle {
    words: [Word; GUESSES_N],
    solutions: [Word; WORDS_N],
    guess_container: WordContainer,
    v_mem: HashMap<StateTurn, f32>,
}

impl Wordle {
    fn get_state(&self, coloring: [TileColor; 5], prev_state: &WordContainer, action: &Word) -> WordContainer {
        let mut new_state = WordContainer::new();
        for index in &prev_state.word_indices {
            let sol = self.words[*index];
            if is_valid_solution(action, coloring, &sol) {
                new_state.add_word(&sol);
            }
        }
        new_state
    }

    pub fn new(words: [Word; GUESSES_N], solutions: [Word; WORDS_N]) -> Wordle {
        let guess_container = WordContainer {
            bool_array: [true; GUESSES_N],
            word_indices: (0..GUESSES_N).collect(),
        };
        Wordle {
            words,
            solutions,
            guess_container,
            v_mem: HashMap::new(),
        }
    }
    fn get_transition(&self, state: &WordContainer, action: &Word) -> Transition {
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

    fn get_state_value(&self, state_turn: StateTurn) -> f32 {
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
        let mut state_value = f32::INFINITY;
        let new_guesses = self.guess_container.disjoint(state);
        let actions = state.union(&new_guesses);
        for action_index in actions.word_indices {
            let mut temp = 1f32;
            let action = &self.words[action_index];
            let next_states = self.get_transition(state, action);
            if next_states.map.len() == 1 && next_states.map.contains_key(&state) {
                continue;
            }
            for next_state in next_states.map.keys() {
                temp += (2 * next_state.len() - 1) as f32 / next_state.len() as f32;
            }

            for (next_state, val) in next_states.map {
                if temp >= val {
                    break;
                }
                else if next_state.len() == 1 && next_state.has_word(action) {
                    continue;
                }
                let next_state_turn = StateTurn {
                    turn: t + 1,
                    state: next_state,
                };
                temp += self.get_state_value(next_state_turn);
            }
            if temp < state_value {
                state_value = temp;
            }
        }
        return state_value;
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

#[test]
fn stateturn_hashmap_works() {
    let mut wc1 = WordContainer::new();
    let union = Word::new("union", 0);
    let untie = Word::new("untie", 1);
    wc1.add_word(&union);

    let turn_1 = StateTurn{
        state: wc1,
        turn: 1,
    };
    let mut wc2 = WordContainer::new();
    wc2.add_word(&union);
    wc2.add_word(&untie);

    let turn_2 = StateTurn{
        state: wc2,
        turn: 2,
    };

    let mut st_hashmap1: HashMap<&StateTurn, f32> = HashMap::new();
    st_hashmap1.insert(&turn_1, 0.5);
    if let Some(p) = st_hashmap1.get(&turn_1) {
        assert_eq!(*p, 0.5);
    } else {
        assert!(false);
    }

    let mut st_hashmap2: HashMap<&StateTurn, f32> = HashMap::new();
    st_hashmap2.insert(&turn_2, 1.0);
    if let Some(p) = st_hashmap2.get(&turn_2) {
        assert_eq!(*p, 1.0);
    } else {
        assert!(false);
    }
}