use std::collections::HashMap;

use crate::state::State;
use crate::transitions::get_next_states;

#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub min: i32,
    pub max: i32,
}

pub struct Solver {
    pub values: HashMap<State, Value>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn solve(&mut self, states: &[State]) {
        // base case
        self.values.insert(
            State::new(vec![0, 0, 0, 0, 0]),
            Value { min: -1, max: 1 },
        );

        for state in states {
            let next_states = get_next_states(state);

            let minimax: Vec<i32> = next_states
                .iter()
                .map(|s| self.values.get(s).unwrap_or(&Value { min: 0, max: 0 }).max)
                .collect();

            let maximin: Vec<i32> = next_states
                .iter()
                .map(|s| self.values.get(s).unwrap_or(&Value { min: 0, max: 0 }).min)
                .collect();

            let min_minimax = *minimax.iter().min().unwrap_or(&0);
            let max_maximin = *maximin.iter().max().unwrap_or(&0);

            self.values.insert(
                state.clone(),
                Value {
                    min: min_minimax,
                    max: max_maximin,
                },
            );
        }
    }
}
