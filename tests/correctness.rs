use chomp_solver::state::State;

#[test]
fn test_terminal() {
    let s = State::new(vec![0, 0, 0, 0, 0]);
    assert!(s.is_terminal());
}
