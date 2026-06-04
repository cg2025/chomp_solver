# Design

The system is structured into modular components:

- State representation (`state.rs`)
- Transition function (`transitions.rs`)
- Solver (`solver.rs`)
- Policy extraction (`policy.rs`)

Pipeline:

State Space → Transition Graph → Dynamic Programming → Policy
