use crate::solver::Solver;
use crate::state::State;
use crate::transitions::get_next_states;

pub fn best_move(solver: &Solver, state: &State) -> Option<(usize, u8)> {
    let next_states = get_next_states(state);

    for next in next_states {
        if let Some(val) = solver.values.get(&next) {
            if val.max == -1 {
                // find move diff
                let diff: Vec<u8> = state
                    .rows
                    .iter()
                    .zip(next.rows.iter())
                    .map(|(&a, &b)| a - b)
                    .collect();

                if let Some(idx) = diff.iter().position(|&d| d != 0) {
                    return Some((idx, diff[idx]));
                }
            }
        }
    }

    None
}
