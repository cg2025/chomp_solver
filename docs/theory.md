# Theory

Chomp is a finite deterministic combinatorial game.

Key properties:
- Finite state space
- No cycles (monotonic decrease)
- Perfect information

Thus:
- Can be solved via dynamic programming
- Equivalent to minimax value propagation

Winning states are those with a move to a losing state.
