# Chomp Solver: State-Space Search & Minimax Engine in Rust

Overview

This project implements a full state-space solver for the combinatorial game Chomp using dynamic programming and minimax reasoning.

The system enumerates valid states, computes optimal policies, and identifies winning configurations. It is structured as a lightweight simulation framework, enabling experimentation and future reinforcement learning extensions.

## Features

- State-space generation with combinatorial constraints
- Transition function for legal moves
- Dynamic programming over game graph
- Minimax-style value computation
- Optimal policy extraction

## Architecture

State Generation → Transition Function → Value Computation → Policy Extraction


## Performance

- Explores full state space up to size 8
- Efficient due to monotonic constraints
- Linear pass DP over states

## Future Work

- RL agent (Q-learning / PPO)
- Gym-style environment wrapper
- Parallel state computation
- Larger combinatorial games
