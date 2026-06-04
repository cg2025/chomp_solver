use crate::state::State;

#[derive(Clone, Debug)]
pub struct StepResult {
    pub state: State,
    pub reward: i32,
    pub done: bool,
}

pub struct ChompEnv {
    pub state: State,
}

impl ChompEnv {
    pub fn new(initial: State) -> Self {
        Self { state: initial }
    }

    pub fn reset(&mut self, initial: State) -> State {
        self.state = initial.clone();
        self.state.clone()
    }

    /// Action = (row_index, amount_to_remove)
    pub fn step(&mut self, action: (usize, u8)) -> StepResult {
        let (row, amount) = action;

        let mut new_rows = self.state.rows.clone();
        let x = new_rows[row] - amount;

        for j in row..new_rows.len() {
            new_rows[j] = std::cmp::min(new_rows[j], x);
        }

        let new_state = State::new(new_rows);
        let done = new_state.is_terminal();

        let reward = if done { -1 } else { 0 };

        self.state = new_state.clone();

        StepResult {
            state: new_state,
            reward,
            done,
        }
    }

    pub fn valid_actions(&self) -> Vec<(usize, u8)> {
        let mut actions = Vec::new();

        for i in 0..self.state.rows.len() {
            for k in 1..=self.state.rows[i] {
                actions.push((i, k));
            }
        }

        actions
    }
}
