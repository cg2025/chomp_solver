mod state;
mod transitions;
mod solver;
mod policy;
mod env;
mod agent;

use state::State;
use solver::Solver;
use policy::best_move;
use env::ChompEnv;
use agent::random_action;
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

fn run_episode(env: &mut ChompEnv) -> i32 {
    let mut total_reward = 0;

    loop {
        let actions = env.valid_actions();

        if actions.is_empty() {
            break;
        }

        let action = actions[0];

        let result = env.step(action);

        total_reward += result.reward;

        if result.done {
            return total_reward;
        }
    }

    total_reward
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

    let mut env = ChompEnv::new(initial.clone());

    println!("Starting RL-style simulation...");
    env.state.pretty_print();

    loop {
        let actions = env.valid_actions();

        if actions.is_empty() {
            break;
        }

        // hybrid policy: optimal if available, else random
        let action = if let Some((r, a)) = best_move(&solver, &env.state) {
            (r, a)
        } else {
            random_action(&actions).unwrap()
        };

        println!("Action taken: {:?}", action);

        let result = env.step(action);

        result.state.pretty_print();

        if result.done {
            println!("Game over. Reward: {}", result.reward);
            break;
        }
    }
}
