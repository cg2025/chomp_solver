#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub rows: Vec<u8>, // monotonic non-increasing
}

impl State {
    pub fn new(rows: Vec<u8>) -> Self {
        Self { rows }
    }

    pub fn is_terminal(&self) -> bool {
        self.rows.iter().all(|&x| x == 0)
    }

    pub fn pretty_print(&self) {
        println!("State: {:?}", self.rows);
    }
}
