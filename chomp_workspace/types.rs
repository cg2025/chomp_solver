pub type State = Vec<u8>;
pub type QValue = (i32, i32);

#[derive(Debug, Clone)]
pub struct Move {
    pub row: usize,
    pub value: u8,
}
