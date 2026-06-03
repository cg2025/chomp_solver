use crate::state::State;

pub fn get_next_states(state: &State) -> Vec<State> {
    let mut next_states = Vec::new();
    let n = state.rows.len();

    for i in 0..n {
        for k in 1..=state.rows[i] {
            let x = state.rows[i] - k;

            let mut new_rows = state.rows.clone();
            for j in i..n {
                new_rows[j] = std::cmp::min(new_rows[j], x);
            }

            next_states.push(State::new(new_rows));
        }
    }

    next_states
}
