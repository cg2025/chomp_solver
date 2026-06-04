mod state;
mod transitions;
mod solver;
mod policy;
mod env;
mod agent;

use state::State;
use solver::Solver;
use policy::best_move;

// generate monotonic states
fn get_states(start: u8, m: usize) -> Vec<State> {
    if m == 1 {
        return vec![State::new(vec![start])];
    }

    let mut result = Vec::new();

    for i in 0..=start {
        for mut rest in get_states(start - i, m - 1) {
            let mut v = vec![start];
            v.append(&mut rest.rows);
            result.push(State::new(v));
        }
    }

    result
}

fn main() {
    let mut state_space = Vec::new();

    for s in 1..=8 {
        state_space.extend(get_states(s, 5));
    }

    state_space.sort_by(|a, b| a.rows.cmp(&b.rows));

    let mut solver = Solver::new();
    solver.solve(&state_space);

    let initial = State::new(vec![8, 8, 8, 8, 8]);

    initial.pretty_print();

    if let Some((row, amount)) = best_move(&solver, &initial) {
        println!("Best move: row {}, remove {}", row + 1, amount);
    } else {
        println!("No winning move");
    }
}
